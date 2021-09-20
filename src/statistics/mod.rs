extern crate statrs;
extern crate rand;
use self::rand::Rng;
use self::rand::{SeedableRng, rngs::StdRng};

use crate::paireddata::PairedData;
use self::statrs::distribution;

pub trait DistributedVariable{
    fn inv_cdf(&self, probability: f64) -> f64;
}
pub trait Fittable{
    fn fit(&self, sample: &Vec<f64>) -> Box<dyn DistributedVariable>;
}
pub trait InlineStats{
    fn add_observation(&mut self, sample: &f64);
    fn add_observations(&mut self, sample: &[f64]){
        for s in sample.iter(){
            self.add_observation(s);
        }
    }
}
pub struct ProductMoments{
    pub min: f64,
    pub max: f64,
    pub count: i64,
    pub mean: f64,
    pub sample_variance: f64
}
impl ProductMoments{
    pub fn new() -> Self{
        Self{
            min: f64::MAX,
            max: f64::MIN,
            mean: 0.0,
            count: 0,
            sample_variance: 0.0
        }
    }
}
impl InlineStats for ProductMoments{
    fn add_observation(&mut self, sample: &f64) {
        if self.count == 0 {
            self.max = *sample;
            self.min = *sample;
            self.mean = *sample;
            self.sample_variance = 0.0;
            self.count = 1;
        } else {
            if *sample > self.max {
                self.max = *sample;
            } else if *sample < self.min {
                self.min = *sample
            }
            self.count += 1;
            self.sample_variance = (({self.count-2} as f64 / {self.count-1} as f64 ) * self.sample_variance) + ({sample-self.mean}.powf(2.0))/self.count as f64;
            self.mean = self.mean + ((sample - self.mean) / self.count as f64)
        }
    }
}
pub struct UniformDistribution{
    pub min: f64,
    pub max: f64
}
impl UniformDistribution{
    pub fn new(min: f64, max: f64) -> Self{
        Self{
            min,
            max
        }
    }
}
impl DistributedVariable for UniformDistribution{
    fn inv_cdf(&self, probability: f64) -> f64{
        self.min + ((self.max - self.min) * probability)
    }
}
impl Fittable for UniformDistribution{
    fn fit(&self, sample: &Vec<f64>) -> Box<dyn DistributedVariable>{
        let mut p = ProductMoments::new();
        p.add_observations(sample);
        let min = p.min;
        let max = p.max;
        Box::new(UniformDistribution{min, max})
    }
}
pub struct NormalDistribution{
    pub dist: distribution::Normal,
}
impl NormalDistribution {
    pub fn new(mean: f64, stdev :f64) -> Self{
        Self{
            dist: distribution::Normal::new(mean,stdev).unwrap()
        }
    }
}
impl DistributedVariable for NormalDistribution{
    fn inv_cdf(&self, probability: f64) -> f64{
        distribution::ContinuousCDF::inverse_cdf(&self.dist, probability)
    }
}
impl Fittable for NormalDistribution{
    fn fit(&self, sample: &Vec<f64>) -> Box<dyn DistributedVariable>{
        let mut p = ProductMoments::new();
        p.add_observations(sample);
        let mean = p.mean;
        let st_dev = p.sample_variance.sqrt();//i think this is right.
        let n = NormalDistribution::new(mean,st_dev);
        Box::new(n)
    }
}
pub struct ShiftedGammaDistribution{
    pub dist: distribution::Gamma,
    pub shift: f64
}
impl ShiftedGammaDistribution{
    pub fn new(alpha: f64, beta: f64, shift: f64) -> Self{
        Self{
            dist: distribution::Gamma::new(alpha, 1.0/beta).unwrap(),//verify
            shift
        }
    }
}
impl DistributedVariable for ShiftedGammaDistribution{
    fn inv_cdf(&self, probability: f64) -> f64{
        distribution::ContinuousCDF::inverse_cdf(&self.dist, probability) + self.shift
    }
}
pub struct PearsonIIIDistribution{
    pub dist: Box<dyn DistributedVariable>
}
impl PearsonIIIDistribution{
    pub fn new(mean: f64, standard_deviation: f64, skew: f64) -> Self{
        let zskewtest = skew.abs();
        if zskewtest < 0.0001{
            let zdist = NormalDistribution::new(mean,standard_deviation);
            Self{
                dist: Box::new(zdist)
            }
        }else{
            let alpha = 4.0/(skew*skew);
            let mut beta = 0.5*(standard_deviation*skew);
            if skew < 0.0{
                //negative skew
                beta = -beta;
                let shift = -mean + (2.0*(standard_deviation/skew));
                let ndist = ShiftedGammaDistribution::new(alpha, beta, shift);
                Self{
                    dist: Box::new(ndist)
                }
            }else{
                //positive skew
                let shift = mean - (2.0*(standard_deviation/skew));
                let pdist = ShiftedGammaDistribution::new(alpha, beta, shift);
                Self{
                    dist: Box::new(pdist)
                }
            }
        }
    }
}
impl DistributedVariable for PearsonIIIDistribution{
    fn inv_cdf(&self, probability: f64) -> f64{
        self.dist.inv_cdf(probability)
    }
}
pub struct LogPearsonIIIDistribution{
    pub dist: PearsonIIIDistribution,
    pub skew: f64
}
impl LogPearsonIIIDistribution{
    pub fn new(mean: f64, standard_deviation: f64, skew: f64) -> Self{
        Self{
            dist: PearsonIIIDistribution::new(mean,standard_deviation, skew),
            skew
        }
    }
}
impl DistributedVariable for LogPearsonIIIDistribution{
    fn inv_cdf(&self, probability: f64) -> f64{
        let base: f64 = 10.0;
        base.powf(self.dist.inv_cdf(probability))
    }
}
impl Fittable for LogPearsonIIIDistribution{
    fn fit(&self, sample: &Vec<f64>) -> Box<dyn DistributedVariable>{
        let mut lsample = Vec::new();
        sample.iter().for_each(|s| {
            lsample.push(s.log10());
        });
        let mut p = ProductMoments::new();
        p.add_observations(&lsample);
        let mean = p.mean;
        let st_dev = p.sample_variance.sqrt();//i think this is right.
        //TODO: compute skew, currently using fixed skew.
        let mut skew_sum = 0.0;
        lsample.iter().for_each(|s| {
            skew_sum += (s-mean).powf(3.0);
        });
        let skew = (p.count as f64 * skew_sum) / ({p.count as f64 - 1.0} * {p.count as f64 - 2.0} * st_dev.powf(3.0));
        let n = LogPearsonIIIDistribution::new(mean,st_dev, skew);
        Box::new(n)
    }
}
pub fn bootstrap_to_distribution(dist: &(impl DistributedVariable + Fittable), eyor: i64, seed: u64) -> Box<dyn DistributedVariable>{
    let mut randy = StdRng::seed_from_u64(seed);
    //bootstrap
    let mut bootstrap = Vec::new();
    for i in 0..eyor{
        bootstrap.push(dist.inv_cdf(randy.gen()));//get a random number generator.
    }
    //fit
    dist.fit(&bootstrap)
}
pub fn bootstrap_to_paireddata(dist: &(impl DistributedVariable + Fittable), eyor: i64, ordinates: i64, seed: u64) -> PairedData {
    let bootstrapdist = bootstrap_to_distribution(dist, eyor, seed);
    //create paired data
    let mut bootstrappd = PairedData::new();
    for i in 0..ordinates{
        let p = {i as f64/ordinates as f64} as f64;
        bootstrappd.add_pair(p, bootstrapdist.inv_cdf(p));
    }
    bootstrappd
}