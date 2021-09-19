extern crate statrs;

use crate::paireddata::PairedData;
use self::statrs::distribution;

pub trait DistributedVariable{
    fn inv_cdf(&self, probability: f64) -> f64;
}
pub trait Fittable{
    fn fit(&self, sample: Vec<f64>) -> Box<dyn DistributedVariable>;
}

pub struct UniformDistribution{
    pub min: f64,
    pub max: f64
}
impl DistributedVariable for UniformDistribution{
    fn inv_cdf(&self, probability: f64) -> f64{
        self.min + ((self.max - self.min) * probability)
    }
}
impl Fittable for UniformDistribution{
    fn fit(&self, sample: Vec<f64>) -> Box<dyn DistributedVariable>{
        let min = sample.into_iter().reduce(f64::min).unwrap();
        let max = sample.into_iter().reduce(f64::max).unwrap();
        Box::new(UniformDistribution{min, max})
    }
}
fn bootstrap_to_distribution(dist: &(impl DistributedVariable + Fittable), eyor: i64) -> Box<dyn DistributedVariable>{
    //bootstrap
    let mut bootstrap = Vec::new();
    for i in 0..eyor{
        bootstrap.push(dist.inv_cdf(0.5));//get a random number generator.
    }
    //fit
    dist.fit(bootstrap)
}
fn bootstrap_to_paireddata(dist: &(impl DistributedVariable + Fittable), eyor: i64, ordinates: i64) -> PairedData {
    let bootstrapdist = bootstrap_to_distribution(dist, eyor);
    //create paired data
    let mut bootstrappd = PairedData::new();
    for i in 0..ordinates{
        let p = {i as f64/ordinates as f64} as f64;
        bootstrappd.add_pair(p, bootstrapdist.inv_cdf(p));
    }
    bootstrappd
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
    pub dist: PearsonIIIDistribution
}
impl LogPearsonIIIDistribution{
    pub fn new(mean: f64, standard_deviation: f64, skew: f64) -> Self{
        Self{
            dist: PearsonIIIDistribution::new(mean,standard_deviation, skew)
        }
    }
}
impl DistributedVariable for LogPearsonIIIDistribution{
    fn inv_cdf(&self, probability: f64) -> f64{
        let base: f64 = 10.0;
        base.powf(self.dist.inv_cdf(probability))
    }
}