use super::PairedData;

pub trait PairedDataSampler{
    fn sample(&self, randomvalue: f64) -> PairedData;
}
pub struct UncertainPairedData<'a>{
    pub xvals : &'a[f64],
    pub yvals : &'a[f64]//dyn DistributedVariable]
}
impl PairedDataSampler for UncertainPairedData<'_>{
    fn sample(&self, randomvalue: f64)-> PairedData{
        PairedData{xvals: self.xvals, yvals: self.yvals}
    }
}