use super::*;

/// [`Interval`] implementation that does nothing
#[derive(Default, Clone)]
pub struct NonInterval {}

impl NonInterval {
    pub fn new() -> Self {
        Self {}
    }
}

impl Interval for NonInterval {
    #[inline(always)]
    fn interval(&self) {}
}
