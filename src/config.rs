pub struct GameConfig {
    pub lower_bound: u32,
    pub upper_bound: u32,
}

impl GameConfig {
    pub fn new(lower_bound: u32, upper_bound: u32) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }
}

