# waiter-trait
Traits used to wait and timeout in a `no-std` embedded system.

## Usage
```shell
cargo add waiter-trait
```

See [crate](https://crates.io/crates/waiter-trait)

## Features

- `std`: Disabled by default.

## Example

```rust
use waiter_trait::{Waiter, WaiterStatus, StdWaiter};
use std::time::Duration;

// Initialize limit time and interval time.
let waiter = StdWaiter::new(Duration::from_millis(80), Some(Duration::from_millis(50)));

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
