extern crate statrs;

use crate::paireddata::PairedData;
use self::statrs::distribution;

pub trait DistributedVariable{
    fn inv_cdf(&self, probability: f64) -> f64;
}
pub trait Bootstrapper : DistributedVariable + Fittable{
    fn bootstrap(&self, eyor: i64, ordinates: i64) -> PairedData;
}
pub trait Fittable{
    fn fit(&self, sample: Vec<f64>) -> dyn DistributedVariable;
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
/*
impl Fittable for UniformDistribution{
    fn fit(&self, sample: Vec<f64>) -> (dyn DistributedVariable + 'static){
        let min = *sample.iter().min().unwrap();
        let max = *sample.iter().max().unwrap();
        UniformDistribution{min, max}
    }
}
impl Bootstrapper for UniformDistribution{
    fn bootstrap(&self, eyor: i64, ordinates: i64) -> PairedData {
        todo!()
    }
}
impl Bootstrapper for DistributedVariable{
    fn bootstrap(&self, eyor: i64, ordinates: i64) -> PairedData {
        
        let mut bootstrap = Vec::new();
        for i in 0..size{
            ys.push(self.inv_cdf(randomvalue));
            let x = self.xvals[i];
            xs.push(x);
        }
        
        PairedData{xvals: xs, yvals: ys}
    }
}
*/

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
            dist: distribution::Gamma::new(alpha, beta).unwrap(),//verify
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
    pub dist: dyn DistributedVariable
}
impl PearsonIIIDistribution{
    pub fn new(mean: f64, standard_deviation: f64, skew: f64) -> Self{
        let mut distribution : dyn DistributedVariable;
        let zskewtest = skew.abs();
        if zskewtest < 0.0001{
            distribution = NormalDistribution::new(mean,standard_deviation);
        }else{
            let alpha = 4.0*(skew*skew);
            let mut beta = 0.5*(standard_deviation*skew);
            if skew < 0.0{
                //negative skew
                beta = -beta;
                let shift = -mean + (2.0*(standard_deviation/skew));
                distribution = ShiftedGammaDistribution::new(alpha, beta, shift);
            }else{
                //positive skew
                let shift = mean - (2.0*(standard_deviation/skew));
                distribution = ShiftedGammaDistribution::new(alpha, beta, shift);
            }
        }
        Self{
            dist: distribution
        }
    }
}
impl DistributedVariable for PearsonIIIDistribution{
    fn inv_cdf(&self, probability: f64) -> f64{
        self.inv_cdf(probability)
    }
}