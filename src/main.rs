use paireddata::ValueSampler;
use statistics::DistributedVariable;
mod paireddata;
mod statistics;
fn main() {
    let name = "uniform distribution in Rust";
    let u= statistics::UniformDistribution{min: 1.0, max :2.0};
    let x = vec![&1.0,&2.0,&3.0,&4.0];
    let y = vec![&10.0,&20.0,&30.0,&40.0];
    let pd = paireddata::PairedData{xvals: x, yvals: y};
    let output = pd.f(4.01);
    println!("searched value was, {}!", output);
    println!("{}, {}!", name, u.inv_cdf(0.25));
}
