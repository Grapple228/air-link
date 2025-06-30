#[derive(Debug, Clone)]
pub struct DisplayParams {
    width: u32,
    height: u32,
}

impl DisplayParams {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
