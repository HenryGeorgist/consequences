use paireddata::paireddata::ValueSampler;

mod paireddata;
fn main() {
    let name = "uniform distribution in Rust";
    let u= UniformDistribution{min: 1.0, max :2.0};
    let x = [1.0,2.0,3.0,4.0];
    let y = [10.0,20.0,30.0,40.0];
    let pd = paireddata::paireddata::PairedData{xvals: &x, yvals: &y};
    let output = pd.f(4.01);
    println!("searched value was, {}!", output);
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