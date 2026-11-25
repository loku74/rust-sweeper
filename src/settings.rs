#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
}

impl Difficulty {
    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            Difficulty::Beginner => (9, 9),
            Difficulty::Intermediate => (16, 16),
            Difficulty::Expert => (30, 16),
        }
    }

    pub fn mines(&self) -> usize {
        match self {
            Difficulty::Beginner => 10,
            Difficulty::Intermediate => 40,
            Difficulty::Expert => 99,
        }
    }
}
