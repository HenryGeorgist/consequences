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
    upd.push(1.0, statistics::UniformDistribution{min:5.0,max:15.0});
    upd.push(2.0, statistics::UniformDistribution{min:15.0,max:25.0});
    upd.push(3.0, statistics::UniformDistribution{min:25.0,max:35.0});
    upd.push(4.0, statistics::UniformDistribution{min:35.0,max:45.0});

    let pd2 = upd.sample(0.75);
    let output = pd.f(4.01);
    println!("searched value was, {}!", output);
    println!("{}, {}!", name, u.inv_cdf(0.25));
    let output2 = pd2.f(4.01);
    println!("searched value was, {}!", output2);
}
