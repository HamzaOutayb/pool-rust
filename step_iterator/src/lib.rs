use std::ops::Add;
use std::cmp::PartialOrd;

pub struct StepIterator<T> {
    beg: T,
    end: T,
    step: T,
}

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
    }
}

impl<T: Add<Output = T> + Copy + PartialOrd> Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg >= self.end {
            return None;
        }
        let current = self.beg;
        self.beg = self.beg + self.step;
        Some(current)
    }
}
