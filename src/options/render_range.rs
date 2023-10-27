pub struct RenderRange {
    pub start: usize,
    pub end: usize,
}

impl RenderRange {
    pub fn new(start: usize, end: usize) -> Self { Self { start, end } }
}