use super::*;
use std::{
    thread::{sleep, yield_now},
    time::{Duration, Instant},
};

/// A [`Waiter`] implementation for `std`.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use waiter_trait::{Waiter, WaiterStatus, StdWaiter, StdInterval};
///
/// let w = StdWaiter::new(Duration::from_millis(10), StdInterval::new(Duration::from_millis(10)));
/// let mut t = w.start();
/// assert!(!t.timeout());
/// assert!(t.timeout());
///
/// t.restart();
/// assert!(!t.timeout());
/// assert!(t.timeout());
/// ```
pub struct StdWaiter<I> {
    timeout: Duration,
    interval: I,
}

impl<I: Interval> StdWaiter<I> {
    /// - `timeout`
    /// - `interval`: Before the time limit expires,
    ///    this action will execute each time `timeout()` is called.
    pub fn new(timeout: Duration, interval: I) -> Self {
        Self { timeout, interval }
    }
}

impl<I: Interval> Waiter for StdWaiter<I> {
    #[inline]
    fn start(&self) -> impl WaiterStatus {
        StdWaiterStatus {
            start_time: Instant::now(),
            waiter: self,
        }
    }
}

pub struct StdWaiterStatus<'a, I> {
    start_time: Instant,
    waiter: &'a StdWaiter<I>,
}

impl<'a, I: Interval> WaiterStatus for StdWaiterStatus<'a, I> {
    #[inline]
    fn timeout(&mut self) -> bool {
        if self.start_time.elapsed() >= self.waiter.timeout {
            true
        } else {
            self.waiter.interval.interval();
            false
        }
    }

    #[inline(always)]
    fn restart(&mut self) {
        self.start_time = Instant::now();
    }
}

impl TickInstant for Instant {
    #[inline(always)]
    fn now() -> Self {
        Instant::now()
    }

    #[inline(always)]
    fn tick_since(self, earlier: Self) -> u32 {
        self.duration_since(earlier).as_nanos() as u32
    }
}

#[derive(Clone)]
pub struct StdInterval {
    duration: Duration,
}

impl StdInterval {
    /// - `duration`: the action in `interval()`.
    ///     - `Duration::ZERO`: call `yield_now()`
    ///     - `Duration`: call `sleep(duration)`
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl Interval for StdInterval {
    #[inline]
    fn interval(&self) {
        match self.duration {
            Duration::ZERO => yield_now(),
            duration => sleep(duration),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_waiter() {
        let w = StdWaiter::new(Duration::from_millis(10), NonInterval::new());
        let mut t = w.start();
        assert!(!t.timeout());
        sleep(Duration::from_millis(1));
        assert!(!t.timeout());
        sleep(Duration::from_millis(9));
        assert!(t.timeout());
        assert!(t.timeout());

        let w = StdWaiter::new(
            Duration::from_millis(500),
            StdInterval::new(Duration::from_millis(260)),
        );
        let mut t = w.start();
        assert!(!t.timeout());
        assert!(!t.timeout());
        assert!(t.timeout());
        assert!(t.timeout());

        t.restart();
        assert!(!t.timeout());
        assert!(!t.timeout());
        assert!(t.timeout());
        assert!(t.timeout());
    }
}
