//! Traits used to wait and timeout in a `no-std` embedded system.
//!
//! # Features
//!
//!- `std`: Disabled by default.
//!
//! # Usage
//!
//! 1. New a [`Waiter`] or [`TimedWaiter`] implementation.
//! 2. Call `start()` to get a [`WaiterStatus`] implementation.
//! 3. Call [`timeout()`](WaiterStatus::timeout) to check if the time limit expires.
//!     1. [`Interval::interval`] is usually called in [`timeout()`](WaiterStatus::timeout)
//!        before the time limit expires. It also depends on your implementation.
//! 4. Call [`restart()`](WaiterStatus::restart) to reset the timeout condition if necessary.
//!
//! ## Example in `std` environment
//!
//! ```
//! use waiter_trait::{Waiter, WaiterStatus, StdWaiter, StdInterval};
//! use std::time::Duration;
//!
//! // Initialize limit time and interval time.
//! let waiter = StdWaiter::new(Duration::from_millis(80), StdInterval::new(Duration::from_millis(50)));
//!
//! fn foo(waiter: impl Waiter) {
//!     let mut t = waiter.start();
//!     loop {
//!         // Wait for something.
//!
//!         // Reset if it's necessary.
//!         t.restart();
//!
//!         if t.timeout() {
//!             break;
//!         }
//!     }
//! }
//! ```
//!
//! # Implementations
//!
//! For developers, you can choose one of the following options.
//! - Implement [`Waiter`] or [`TimedWaiter`], and [`WaiterStatus`] then use them.
//! - Implement [`TickInstant`] then use [`TickWaiter`] or [`TimedTickWaiter`].
//!     - Simply give [`NonInterval`] to [`Waiter`], If don't need interval.
//!       In this way, you can also use `DelayNs` or `sleep` separately.
//!     - Or you can implement [`Interval`] for your own interval action.
//! - Using [`Counter`], if you don't have any tick source.
//!
//! It also provides a implementation of `DelayNs` named [`TickDelay`]

#![cfg_attr(not(feature = "std"), no_std)]

mod counter;
pub use counter::*;
mod non_interval;
pub use non_interval::*;
mod tick_waiter;
pub use tick_waiter::*;
mod tick_delay;
pub use tick_delay::*;
mod timed_tick_waiter;
pub use timed_tick_waiter::*;

#[cfg(feature = "std")]
mod std_impls;
#[cfg(feature = "std")]
pub use std_impls::*;

pub use embedded_hal::delay::DelayNs;
pub use fugit::{self, MicrosDurationU32};

pub mod prelude;

pub trait Waiter {
    /// Start waiting.
    fn start(&self) -> impl WaiterStatus;
}

/// The difference from [`Waiter`] is that it supports setting timeout at `start()`.
pub trait TimedWaiter {
    /// Set timeout and start waiting.
    fn start(&self, timeout: MicrosDurationU32) -> impl WaiterStatus;
}

pub trait WaiterStatus {
    /// Check if the time limit expires. This function may sleeps for a while,
    /// depends on the implementation.
    fn timeout(&mut self) -> bool;
    /// Reset the timeout condition.
    fn restart(&mut self);
}

pub trait TickInstant: Copy {
    fn now() -> Self;
    /// Returns the amount of ticks elapsed from another instant to this one.
    fn tick_since(self, earlier: Self) -> u32;
    /// Returns the amount of ticks elapsed since this instant.
    fn tick_elapsed(self) -> u32 {
        Self::now().tick_since(self)
    }
}

/// It is usually called at [`WaiterStatus::timeout`] before the time limit expires.
/// It can be implemented for `yield`, `sleep` or do nothing.
pub trait Interval: Clone {
    fn interval(&self);
}
