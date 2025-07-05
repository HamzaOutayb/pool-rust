#[derive(Debug, Clone)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers.clone()
    }

    pub fn latest(&self) -> Option<u32> {
        Some(self.numbers[self.numbers.len()-1])
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().copied().max()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut v = self.numbers.to_vec();
        v.sort_by(|a, b| b.cmp(a));
        v.truncate(3);
        v
    }
}