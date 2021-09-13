extern crate statrs;
use self::statrs::distribution;

pub trait DistributedVariable{
    fn inv_cdf(&self, probability: f64) -> f64;
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