use super::PairedData;
use statistics::DistributedVariable;

pub trait PairedDataSampler{
    fn sample(&self, randomvalue: f64) -> PairedData;
}
pub struct UncertainPairedData{
    pub xvals : Vec<f64>,
    pub yvals : Vec<Box<dyn DistributedVariable>>
}
impl PairedDataSampler for UncertainPairedData{
    fn sample(&self, randomvalue: f64)-> PairedData{
        let size = self.xvals.len();
        let mut ys = Vec::with_capacity(size);//how do i allocate a slice?
        let mut xs = Vec::with_capacity(size);
        for i in 0..size{
            ys[i] = self.yvals[i].inv_cdf(randomvalue);
            let x = self.xvals[i];
            xs[i] = x;
        }
        
        PairedData{xvals: xs, yvals: ys}
    }
}