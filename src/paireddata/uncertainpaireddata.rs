use super::PairedData;
use statistics::DistributedVariable;

pub trait PairedDataSampler{
    fn sample(&self, randomvalue: f64) -> PairedData;
}
pub struct UncertainPairedData{
    pub xvals : Vec<f64>,
    pub yvals : Vec<Box<dyn DistributedVariable>>
}

impl UncertainPairedData {
    pub fn new() -> Self{
        Self{
            yvals: Vec::new(),
            xvals: Vec::new()
        }
    }
    pub fn add_pair<S: DistributedVariable + 'static>(&mut self,x: f64, dist: S) -> &mut Self{
        self.yvals.push(Box::new(dist));
        self.xvals.push(x);
        self
    }
}
impl PairedDataSampler for UncertainPairedData{
    fn sample(&self, randomvalue: f64)-> PairedData{
        let size = self.xvals.len();
        let mut ys = Vec::new();//how do i allocate a slice?
        let mut xs = Vec::new();
        for i in 0..size{
            ys.push(self.yvals[i].inv_cdf(randomvalue));
            let x = self.xvals[i];
            xs.push(x);
        }
        
        PairedData{xvals: xs, yvals: ys}
    }
}