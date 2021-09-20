use paireddata::{Integratable, ValueSampler};
extern crate rand;
use self::rand::Rng;
use self::rand::{SeedableRng, rngs::StdRng};
use statistics::{DistributedVariable, LogPearsonIIIDistribution, UniformDistribution, bootstrap_to_distribution};
use crate::paireddata::uncertainpaireddata::UncertainPairedData;
use crate::statistics::{InlineStats, bootstrap_to_paireddata};
use crate::{paireddata::{Composable, uncertainpaireddata::PairedDataSampler}, statistics::ProductMoments};
mod paireddata;
mod statistics;
fn main() {
    let x = vec![1.0,2.0,3.0,4.0];
    let y = vec![10.0,20.0,30.0,40.0];
    let pd = paireddata::PairedData{xvals: x, yvals: y};
    let mut upd = paireddata::uncertainpaireddata::UncertainPairedData::new();
    upd.add_pair(1.0, statistics::UniformDistribution{min:5.0,max:15.0});
    upd.add_pair(2.0, statistics::UniformDistribution{min:15.0,max:25.0});
    upd.add_pair(3.0, statistics::UniformDistribution{min:25.0,max:35.0});
    upd.add_pair(4.0, statistics::UniformDistribution{min:35.0,max:45.0});
    
    let mut npd = paireddata::uncertainpaireddata::UncertainPairedData::new();
    npd.add_pair(1.0, statistics::NormalDistribution::new(10.0,1.0));
    npd.add_pair(2.0, statistics::NormalDistribution::new(20.0,2.0));
    npd.add_pair(3.0, statistics::NormalDistribution::new(30.0,3.0 ));
    npd.add_pair(4.0, statistics::NormalDistribution::new(40.0,4.0));

    let pd2 = upd.sample(0.5);
    let pd3 = npd.sample(0.5);
    let pd4 = pd2.compose(&pd3);
    let output = pd.f(1.25);
    println!("searched value was, {}!", output);
    let output2 = pd2.f(1.5);
    println!("searched value was, {}!", output2);
    let output3 = pd3.f(2.5);
    println!("searched value was, {}!", output3);
    let output4 = pd4.f(3.25);
    println!("searched value was, {}!", output4);
    fda_ead_deterministic();
    test_lpiii_dist();
    test_fda_ead_uncertainty();
}
fn test_lpiii_dist(){
    let lpiii = LogPearsonIIIDistribution::new(3.368, 0.246, 0.668);
	let probs = [0.998, 0.995, 0.99, 0.98, 0.95, 0.9, 0.8, 0.5, 0.2, 0.1, 0.05, 0.01];
	let expected = [18878.87515053270180942491, 14246.58825980164874636102, 11408.83966308754315832630, 9043.72657283687294693664, 6511.95816420457322237780, 4961.12702987368902540766, 3656.87315507261564562214, 2191.79779904862152761780, 1435.93911608508096833248, 1189.92079576230275961279, 1035.43101823480742496031, 827.66401592971760692308];
	let size = probs.len();
    for idx in  0..size {
		let got = lpiii.inv_cdf(probs[idx]);
		let diff = expected[idx] - got;
		if diff.abs() > 0.5 {
			println!("InvCDF{} = {}; expected {}", probs[idx], got, expected[idx])
		}
	}
    let seed = 1234;
    let _dist2 = bootstrap_to_distribution(&lpiii, 100, seed);
}
fn fda_ead_deterministic(){
    //create a flow frequency curve
    let flow_distribution = UniformDistribution{min: 0.0, max: 1000.0};
    let mut flow_frequency = paireddata::PairedData::new();
    let ords = 100000;
    for i in 0..ords{
        let p = {i as f64/ords as f64} as f64;
        flow_frequency.add_pair(p, flow_distribution.inv_cdf(p));
    }

    let mut flow_stage = paireddata::PairedData::new();
    flow_stage.add_pair(0.0, 0.0);
    flow_stage.add_pair(1.0, 2.0);
    flow_stage.add_pair(5.0, 10.0);
    flow_stage.add_pair(100.0, 200.0);
    flow_stage.add_pair(1000.0, 2000.0);

    let mut stage_damage = paireddata::PairedData::new();
    stage_damage.add_pair(0.0, 0.0);
    stage_damage.add_pair(2.0, 2.0);
    stage_damage.add_pair(10.0, 10.0);
    stage_damage.add_pair(200.0, 200.0);
    stage_damage.add_pair(2000.0, 2000.0);

    let frequency_stage = flow_stage.compose(&flow_frequency);
    let frequency_damage = stage_damage.compose(&frequency_stage);
    let ead = frequency_damage.integrate();
    println!("EAD was {}!", ead);
}
fn test_fda_ead_uncertainty(){
    //create an unregulated frequency curve
    let lpiii = LogPearsonIIIDistribution::new(3.368, 0.246, 0.668);
    //create rating curve
    let mut flow_stage_u = UncertainPairedData::new();
    flow_stage_u.add_pair(0.0, UniformDistribution::new(0.0,0.3));
    flow_stage_u.add_pair(1.0, UniformDistribution::new(1.0,3.0));
    flow_stage_u.add_pair(5.0, UniformDistribution::new(5.0,15.0));
    flow_stage_u.add_pair(100.0, UniformDistribution::new(100.0,300.0));
    flow_stage_u.add_pair(1000000.0, UniformDistribution::new(1000000.0,3000000.0));

    //create stage damage curve
    let mut stage_damage_u = UncertainPairedData::new();
    stage_damage_u.add_pair(0.0, UniformDistribution::new(0.0,0.3));
    stage_damage_u.add_pair(1.0, UniformDistribution::new(1.0,3.0));
    stage_damage_u.add_pair(5.0, UniformDistribution::new(5.0,15.0));
    stage_damage_u.add_pair(100.0, UniformDistribution::new(100.0,300.0));
    stage_damage_u.add_pair(1000000.0, UniformDistribution::new(1000000.0,3000000.0));
    // create basic pm to store outputs
    let mut ead_dist = ProductMoments::new();
    //create a random number generator for the loop.
    let seed = 1234;
    let mut randy = StdRng::seed_from_u64(seed);
    let iterations = 100;
    for i in 0..iterations{
        //sample flow frequency
        let flow_frequency = bootstrap_to_paireddata(&lpiii, 100, 1000, randy.gen());
        //sample flow stage
        let flow_stage = flow_stage_u.sample(randy.gen());
        //sample stage damage
        let stage_damage = stage_damage_u.sample(randy.gen());
        //compute frequency stage
        let frequency_stage = flow_stage.compose(&flow_frequency);
        //compute frequency damage
        let frequency_damage = stage_damage.compose(&frequency_stage);
        //integrate
        let ead = frequency_damage.integrate();
        //track
        ead_dist.add_observation(&ead);
        println!("EAD was {}!", ead);        
    }
    println!("mean EAD was {}, max was {}, min was {}!", ead_dist.mean, ead_dist.max, ead_dist.min); 
}