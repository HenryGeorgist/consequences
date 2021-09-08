    pub trait ValueSampler{
        fn f(&self, x: f64) -> f64;
    }
    pub struct PairedData<'a>{
        pub xvals : &'a[f64],
        pub yvals : &'a[f64]

    }
    impl ValueSampler for PairedData<'_>{
        fn f(&self, x: f64) -> f64{
            if x < self.xvals[0] {
                return 0.0; //xval is less than lowest x value
            }
            let size = self.xvals.len();
            if x >= self.xvals[size-1] {
                return self.yvals[size-1]; //xval yeilds largest y value
            }
            let error_margin = 0.000000000001;
            if (x - self.xvals[0]).abs() < error_margin {
                return self.yvals[0];
            }
            let output = bisearch(self.xvals, size, &x);
            let upper = output.unwrap();
            //interpolate
            let lower = upper - 1; // safe because we trapped the 0 case earlier
            let slope = (self.yvals[upper] - self.yvals[lower]) / (self.xvals[upper] - self.xvals[lower]);
            let a = self.yvals[lower];
            a + slope*(x-self.xvals[lower])

        }
    }
    fn bisearch(a: &[f64], len: usize, target_value: &f64) -> Option<usize> {
        let mut low: i8 = 0;
        let mut high: i8 = len as i8 - 1;

        while low <= high {
            let mid = ((high - low) / 2) + low;
            let mid_index = mid as usize;
            let val = &a[mid_index];

            if (val - target_value).abs() < 0.00000001 {
                return Some(mid_index);
            }

            // Search values that are greater than val - to right of current mid_index
            if val < target_value {
                low = mid + 1;
            }

            // Search values that are less than val - to the left of current mid_index
            if val > target_value {
                high = mid - 1;
            }
        }
        let low_out = low as usize;
        Some(low_out)
        
    }