# leptos-statig

Reactive wrapper for [statig][statig] state machines in [Leptos][leptos].

Bridges statig's synchronous finite state machines with Leptos reactive signals,
so state transitions automatically trigger UI updates.

## Usage

Add `leptos-statig` to your `Cargo.toml`:

```toml
[dependencies]
leptos-statig = "0.1"
```

Wrap any statig state machine in a [`ReactiveStateMachine`][rsm] to get reactive
reads and writes:

```rust,ignore
use leptos::prelude::*;
use leptos_statig::ReactiveStateMachine;

let fsm = ReactiveStateMachine::new(MyMachine::default());

// Reactive — re-runs when state changes
let current = move || fsm.state();

// Dispatch events to drive transitions
fsm.dispatch(&Event::Toggle);
```

### Context helpers

Share a state machine across a component tree with `provide_fsm` and `use_fsm`:

```rust,ignore
use leptos_statig::{provide_fsm, use_fsm};

// In a parent component
let fsm = provide_fsm(MyMachine::default());

// In any descendant component
let fsm = use_fsm::<MyMachine>();
let state = fsm.state();
```

## License

Licensed under either of [Apache License, Version 2.0][apache] or [MIT
License][mit] at your option.

[apache]: LICENSE-APACHE
[leptos]: https://docs.rs/leptos
[mit]: LICENSE-MIT
[rsm]:
  https://docs.rs/leptos-statig/latest/leptos_statig/struct.ReactiveStateMachine.html
[statig]: https://docs.rs/statig
