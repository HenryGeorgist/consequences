use super::PairedData;
use statistics::DistributedVariable;

use std::convert::TryInto;

fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
pub trait PairedDataSampler{
    fn sample(&self, randomvalue: f64) -> PairedData;
}
pub struct UncertainPairedData<'a>{
    pub xvals : Vec<&'a f64>,
    pub yvals : Vec<&'a dyn DistributedVariable>
}
impl PairedDataSampler for UncertainPairedData<'_>{
    fn sample(&self, randomvalue: f64)-> PairedData{
        let size = self.xvals.len();
        let mut ys = Vec::with_capacity(size);//how do i allocate a slice?
        let mut xs = Vec::with_capacity(size);
        for i in 0..size{
            ys[i] = &self.yvals[i].inv_cdf(randomvalue);
            let x = *self.xvals[i];
            xs[i] = &x;
        }
        
        PairedData{xvals: xs, yvals: ys}
    }
}