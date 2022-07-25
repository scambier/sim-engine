pub struct FillPattern {
    pattern: u16,
}

impl FillPattern {
    pub fn new() -> Self {
        Self { pattern: u16::MAX }
    }
}
