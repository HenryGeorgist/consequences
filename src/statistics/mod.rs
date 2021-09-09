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