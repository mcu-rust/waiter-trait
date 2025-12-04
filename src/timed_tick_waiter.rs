use super::*;
use core::marker::PhantomData;
use fugit::MicrosDurationU32;

/// A [`Waiter`] implementation for embedded system.
///
/// The difference from [`TickWaiter`] is that it supports setting timeout at `start()`.
///
/// # Examples
///
/// ```
/// use std::time::{Duration, Instant};
/// use waiter_trait::{prelude::*, TimedTickWaiter, StdInterval};
///
/// let w = TimedTickWaiter::<Instant, _>::new(
///     StdInterval::new(Duration::from_millis(400)),
///     Duration::from_secs(1).as_nanos() as u32,
/// );
///
/// let mut t = w.start(500.millis());
/// assert!(!t.timeout());
/// assert!(!t.timeout());
/// assert!(t.timeout());
///
/// let mut t = w.start(500.millis());
/// assert!(!t.timeout());
/// assert!(!t.timeout());
/// t.restart();
/// assert!(!t.timeout());
/// assert!(!t.timeout());
/// assert!(t.timeout());
/// ```
pub struct TimedTickWaiter<T, I> {
    frequency: u32,
    interval: I,
    _t: PhantomData<T>,
}

impl<T, I> TimedTickWaiter<T, I>
where
    T: TickInstant,
    I: Interval,
{
    pub fn new(interval: I, frequency: u32) -> Self {
        assert_eq!(frequency % 1_000_000, 0);
        Self {
            frequency,
            interval,
            _t: PhantomData,
        }
    }
}

impl<T, I> TimedWaiter for TimedTickWaiter<T, I>
where
    T: TickInstant,
    I: Interval,
{
    fn start(&self, timeout: MicrosDurationU32) -> impl WaiterStatus {
        TimedTickWaiterStatus::<T, I> {
            tick: T::now(),
            timeout_tick: timeout
                .ticks()
                .checked_mul(self.frequency / 1_000_000)
                .unwrap(),
            elapsed_tick: 0,
            waiter: self,
        }
    }
}

pub struct TimedTickWaiterStatus<'a, T: TickInstant, I: Interval> {
    tick: T,
    timeout_tick: u32,
    elapsed_tick: u32,
    waiter: &'a TimedTickWaiter<T, I>,
}

impl<'a, T, I> WaiterStatus for TimedTickWaiterStatus<'a, T, I>
where
    T: TickInstant,
    I: Interval,
{
    /// Can be reused without calling `restart()`.
    #[inline]
    fn timeout(&mut self) -> bool {
        let now = T::now();
        self.elapsed_tick = self.elapsed_tick.add_u32(now.tick_since(self.tick));
        self.tick = now;

        if self.elapsed_tick >= self.timeout_tick {
            self.elapsed_tick -= self.timeout_tick;
            true
        } else {
            self.waiter.interval.interval();
            false
        }
    }

    #[inline(always)]
    fn restart(&mut self) {
        self.tick = T::now();
        self.elapsed_tick = 0;
    }
}
