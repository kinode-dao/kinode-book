# Timer API

The Timer API allows processes to manage time-based operations within Kinode OS.
This API provides a simple yet powerful mechanism for scheduling actions to be executed after a specified delay.
The entire API is just the `TimerAction`:

```rust
pub enum TimerAction {
    Debug,
    SetTimer(u64),
}
```
This defines just two actions: `Debug` and `SetTimer`
## `Debug`
This action will print information about all active timers to the terminal.
## `SetTimer`
This lets you set a timer to pop after a set number of milliseconds, so e.g. `{"SetTimer": 1000}` would pop after one second.
The timer finishes by sending a `Response` once the timer has popped.
The response will have no information in the `body`.
To keep track of different timers, you can use two methods:
- `send_and_await_response` which will block your app while it is waiting
  - use [`kinode_process_lib::timer::set_and_await_timer`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/timer/fn.set_and_await_timer.html) for this
- use `context` to keep track of multiple timers without blocking
  - use [`kinode_process_lib::timer::set_timer`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/timer/fn.set_timer.html) to set the timer with optional context
