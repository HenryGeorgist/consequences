use paireddata::ValueSampler;
use statistics::DistributedVariable;

use crate::paireddata::uncertainpaireddata::PairedDataSampler;
mod paireddata;
mod statistics;
fn main() {
    let name = "uniform distribution in Rust";
    let u= statistics::UniformDistribution{min: 1.0, max :2.0};
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

    let pd2 = upd.sample(0.75);
    let pd3 = npd.sample(0.5);
    let output = pd.f(4.01);
    println!("searched value was, {}!", output);
    println!("{}, {}!", name, u.inv_cdf(0.25));
    let output2 = pd2.f(4.01);
    println!("searched value was, {}!", output2);
    let output3 = pd3.f(4.01);
    println!("searched value was, {}!", output3);
}
