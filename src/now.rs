use std::time::{Duration, Instant};

pub struct Now(Instant);

impl std::fmt::Debug for Now {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Now({:?})", self.elapsed())
    }
}

impl Default for Now {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl Now {
    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }

    pub fn reset(&mut self) {
        self.0 = Instant::now();
    }
}
