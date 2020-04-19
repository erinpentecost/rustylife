pub struct Samples {
    samples: Vec<f64>,
    idx: usize,
}

impl Samples {
    pub fn new(capacity: usize) -> Samples {
        Samples {
            samples: Vec::<f64>::with_capacity(capacity),
            idx: 0,
        }
    }
    pub fn average(&self) -> f64 {
        if self.samples.is_empty() {
            0 as f64
        } else {
            self.samples.iter().sum::<f64>() / self.samples.len() as f64
        }
    }

    pub fn add(&mut self, sample: f64) {
        if self.samples.len() == self.samples.capacity() {
            if self.idx >= self.samples.capacity() {
                self.idx = 0;
            }
            self.samples[self.idx] = sample;
        } else {
            self.samples.push(sample);
        }
    }
}
