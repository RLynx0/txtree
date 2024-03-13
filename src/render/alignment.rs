#[derive(Debug, Clone)]
pub enum Alignment {
    Start,
    Middle,
    End,
}

#[derive(Debug, Clone, Default)]
pub struct AlignmentBuilder {
    pub(crate) centered: bool,
    pub(crate) flipped: bool,
}

impl AlignmentBuilder {
    pub fn new() -> AlignmentBuilder {
        Self::default()
    }

    pub fn centered(&mut self, middle: bool) -> &mut Self {
        self.centered = middle;
        self
    }
    pub fn flipped(&mut self, flipped: bool) -> &mut Self {
        self.flipped = flipped;
        self
    }

    pub fn build(&self) -> Alignment {
        match (self.centered, self.flipped) {
            (true, _) => Alignment::Middle,
            (false, true) => Alignment::End,
            (false, false) => Alignment::Start,
        }
    }
}
