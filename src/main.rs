
fn main() {
    let name = "uniform distribution in Rust";
    let u= UniformDistribution{min: 1.0, max :2.0};
    println!("{}, {}!", name, u.inv_cdf(0.25));
}

trait DistributedVariable{
    fn inv_cdf(&self, probability: f64) -> f64;
}
struct UniformDistribution{
    min: f64,
    max: f64
}
impl DistributedVariable for UniformDistribution{
    fn inv_cdf(&self, probability: f64) -> f64{
        self.min + ((self.max - self.min) * probability)
    }
}