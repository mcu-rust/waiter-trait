use super::*;
use core::marker::PhantomData;
use embedded_hal::delay::DelayNs;
use fugit::ExtU32;

/// [`DelayNs`] implementation
///
/// # Examples
///
/// ```
/// use std::time::{Duration, Instant};
/// use waiter_trait::{TickDelay, DelayNs};
///
/// let mut d = TickDelay::<Instant>::new(
///     Duration::from_secs(1).as_nanos() as u32,
/// );
///
/// let t = Instant::now();
/// d.delay_ns(1_000_000);
/// let elapsed = t.elapsed();
/// assert!(elapsed - Duration::from_nanos(1_000_000) < Duration::from_nanos(100_000));
///
/// let t = Instant::now();
/// d.delay_us(1000);
/// let elapsed = t.elapsed();
/// assert!(elapsed - Duration::from_micros(1000) < Duration::from_micros(500));
/// ```
pub struct TickDelay<T> {
    frequency: u32,
    _t: PhantomData<T>,
}

impl<T> TickDelay<T>
where
    T: TickInstant,
{
    pub fn new(frequency: u32) -> Self {
        Self {
            frequency,
            _t: PhantomData,
        }
    }
}

impl<T> DelayNs for TickDelay<T>
where
    T: TickInstant,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let w = TickWaiter::<T, _, _>::ns(ns.nanos(), NonInterval::new(), self.frequency);
        let mut t = w.start();
        while !t.timeout() {}
    }
}
