use super::PairedData;
use statistics::DistributedVariable;
pub trait PairedDataSampler{
    fn sample(&self, randomvalue: f64) -> PairedData;
}
pub struct UncertainPairedData<'a>{
    pub xvals : &'a[f64],
    pub yvals : Vec<&'a dyn DistributedVariable>
}
impl PairedDataSampler for UncertainPairedData<'_>{
    fn sample(&self, randomvalue: f64)-> PairedData{
        let size = self.xvals.len();
        let ys : [f64;size] = [0.0;size];//how do i allocate a slice?
        for i in 0..size{
            ys[i] = self.yvals[i].inv_cdf(randomvalue);
        }
        PairedData{xvals: self.xvals, yvals: ys}
    }
}