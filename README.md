# waiter-trait
Traits used to wait and timeout in a `no-std` embedded system.


## Features

- `std`: Disabled by default.

## Usage
```shell
cargo add waiter-trait
```

See [crate](https://crates.io/crates/waiter-trait)

1. New a `Waiter` or `TimedWaiter` implementation.
2. Call `start()` to get a `WaiterStatus` implementation.
3. Call `timeout()` to check if the time limit expires.
    1. `Interval::interval` is usually called in `timeout()`
       before the time limit expires. It also depends on your implementation.
4. Call `restart()` to reset the timeout condition if necessary.

### Example

```
use waiter_trait::prelude::*;
fn foo(waiter: impl Waiter) {
    let mut t = waiter.start();
    loop {
        // Wait for something.
        // Reset if it's necessary.
        t.restart();
        if t.timeout() {
            break;
        }
    }
}
```

### Pre-implemented

- `StdWaiter` and `StdInterval`: Need the `std` feature enabled.
- `NonInterval`: implements `Interval` that does nothing.
- `TickDelay`: implements `DelayNs`

## Implement Your Own

For developers, you can choose one of the following options.
- Implement `Waiter` or `TimedWaiter`, and `WaiterStatus` then use them.
- Implement `TickInstant` then use `TickWaiter` or `TimedTickWaiter`.
    - Simply give `NonInterval` to `Waiter`, If you don't need interval.
      In this way, you can also use `DelayNs` or `sleep` separately.
    - Or you can implement `Interval` and use it.
- Using `Counter`, if you don't have any tick source.
