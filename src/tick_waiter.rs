use super::*;
use core::marker::PhantomData;
use fugit::{
    MicrosDurationU32, MicrosDurationU64, MillisDurationU32, NanosDurationU32, NanosDurationU64,
};

/// A [`Waiter`] implementation for embedded system.
///
/// # Examples
///
/// ```
/// use std::time::{Duration, Instant};
/// use waiter_trait::{prelude::*, TickWaiter, StdInterval};
///
/// let w = TickWaiter::<Instant, _, _>::ns_u64(
///     500.millis().into(),
///     StdInterval::new(Duration::from_millis(400)),
///     Duration::from_secs(1).as_nanos() as u32,
/// );
///
/// let mut t = w.start();
/// assert!(!t.timeout());
/// assert!(!t.timeout());
/// assert!(t.timeout());
///
/// let mut t = w.start();
/// assert!(!t.timeout());
/// assert!(!t.timeout());
/// t.restart();
/// assert!(!t.timeout());
/// assert!(!t.timeout());
/// assert!(t.timeout());
/// ```
pub struct TickWaiter<T, I, N> {
    timeout_tick: N,
    interval: I,
    _t: PhantomData<T>,
}

impl<T, I> TickWaiter<T, I, u32>
where
    T: TickInstant,
    I: Interval,
{
    /// If you want to wait longer, lower the frequency or/and use other initialization functions.
    pub fn ns(timeout: NanosDurationU32, interval: I, frequency: u32) -> Self {
        assert_eq!(frequency % 1_000_000, 0);
        let timeout_tick = (timeout.ticks() as u64)
            .checked_mul((frequency / 1_000_000) as u64)
            .unwrap()
            .div_ceil(1_000);
        assert!(timeout_tick <= u32::MAX as u64);
        Self {
            timeout_tick: timeout_tick as u32,
            interval,
            _t: PhantomData,
        }
    }

    pub fn us(timeout: MicrosDurationU32, interval: I, frequency: u32) -> Self {
        assert_eq!(frequency % 1_000_000, 0);
        Self {
            timeout_tick: timeout.ticks().checked_mul(frequency / 1_000_000).unwrap(),
            interval,
            _t: PhantomData,
        }
    }

    pub fn ms(timeout: MillisDurationU32, interval: I, frequency: u32) -> Self {
        assert_eq!(frequency % 1_000, 0);
        Self {
            timeout_tick: timeout.ticks().checked_mul(frequency / 1_000).unwrap(),
            interval,
            _t: PhantomData,
        }
    }
}

impl<T, I> TickWaiter<T, I, u64>
where
    T: TickInstant,
    I: Interval,
{
    pub fn ns_u64(timeout: NanosDurationU64, interval: I, frequency: u32) -> Self {
        assert_eq!(frequency % 1_000_000, 0);
        Self {
            timeout_tick: timeout
                .ticks()
                .checked_mul((frequency / 1_000_000) as u64)
                .unwrap()
                .div_ceil(1_000),
            interval,
            _t: PhantomData,
        }
    }

    pub fn us_u64(timeout: MicrosDurationU64, interval: I, frequency: u32) -> Self {
        assert_eq!(frequency % 1_000_000, 0);
        Self {
            timeout_tick: timeout
                .ticks()
                .checked_mul((frequency / 1_000_000) as u64)
                .unwrap(),
            interval,
            _t: PhantomData,
        }
    }
}

impl<T, I, N> Waiter for TickWaiter<T, I, N>
where
    N: Num,
    T: TickInstant,
    I: Interval,
{
    fn start(&self) -> impl WaiterStatus {
        TickWaiterStatus::<T, I, N> {
            tick: T::now(),
            elapsed_tick: N::ZERO,
            waiter: self,
        }
    }
}

pub struct TickWaiterStatus<'a, T: TickInstant, I: Interval, N: Num> {
    tick: T,
    elapsed_tick: N,
    waiter: &'a TickWaiter<T, I, N>,
}

impl<'a, T, I, N> WaiterStatus for TickWaiterStatus<'a, T, I, N>
where
    N: Num,
    T: TickInstant,
    I: Interval,
{
    /// Can be reused without calling `restart()`.
    #[inline]
    fn timeout(&mut self) -> bool {
        let now = T::now();
        self.elapsed_tick = self.elapsed_tick.add_u32(now.tick_since(self.tick));
        self.tick = now;

        if self.elapsed_tick >= self.waiter.timeout_tick {
            self.elapsed_tick -= self.waiter.timeout_tick;
            true
        } else {
            self.waiter.interval.interval();
            false
        }
    }

    #[inline(always)]
    fn restart(&mut self) {
        self.tick = T::now();
        self.elapsed_tick = N::ZERO;
    }
}

pub trait Num: Sized + Copy + core::cmp::Ord + core::ops::SubAssign {
    const ZERO: Self;
    fn add_u32(self, v: u32) -> Self;
}

impl Num for u32 {
    const ZERO: Self = 0;
    fn add_u32(self, v: u32) -> Self {
        self.saturating_add(v)
    }
}

impl Num for u64 {
    const ZERO: Self = 0u64;
    fn add_u32(self, v: u32) -> Self {
        self.saturating_add(v as u64)
    }
}
