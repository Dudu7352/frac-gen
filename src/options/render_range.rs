pub struct RenderRange {
    pub start: usize,
    pub end: usize,
}

impl RenderRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}
