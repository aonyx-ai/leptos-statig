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

Define a state machine with statig's `#[state_machine]` macro, then wrap it in a
[`ReactiveStateMachine`][rsm]:

```rust
use leptos::prelude::*;
use leptos_statig::ReactiveStateMachine;
use statig::prelude::*;

struct Toggle;

#[state_machine(initial = "State::off()", state(derive(Clone, Debug)))]
impl Toggle {
    #[state]
    fn off(event: &Event) -> Outcome<State> {
        match event {
            Event::Toggle => Transition(State::on()),
        }
    }

    #[state]
    fn on(event: &Event) -> Outcome<State> {
        match event {
            Event::Toggle => Transition(State::off()),
        }
    }
}

enum Event {
    Toggle,
}

#[component]
fn ToggleButton() -> impl IntoView {
    let fsm = ReactiveStateMachine::new(Toggle);

    view! {
        <button on:click=move |_| fsm.dispatch(&Event::Toggle)>
            {move || format!("{:?}", fsm.state())}
        </button>
    }
}
```

`fsm.state()` is a reactive read — any component that calls it re-renders when
an event causes a state transition. `fsm.dispatch()` is a reactive write that
drives the machine forward.

### Context helpers

Share a state machine across a component tree with `provide_fsm` and `use_fsm`:

```rust
use leptos_statig::{provide_fsm, use_fsm};

// In a parent component
let fsm = provide_fsm(MyMachine::default());

// In any descendant component
let fsm = use_fsm::<MyMachine>();
let state = fsm.state();
```

### Note on imports

Both `leptos::prelude` and `statig::prelude` export a `Transition` name. If you
glob-import both, add an explicit re-import to shadow the Leptos one:

```rust
use leptos::prelude::*;
use statig::prelude::*;
use statig::Outcome::Transition; // shadows leptos::prelude::Transition
```

### Examples

See the [`examples/`][examples] directory for runnable demos:

```sh
cd examples/counter && trunk serve
```

## License

Licensed under either of [Apache License, Version 2.0][apache] or [MIT
License][mit] at your option.

[apache]: LICENSE-APACHE
[examples]: https://github.com/aonyx-ai/leptos-statig/tree/main/examples
[leptos]: https://docs.rs/leptos
[mit]: LICENSE-MIT
[rsm]:
  https://docs.rs/leptos-statig/latest/leptos_statig/struct.ReactiveStateMachine.html
[statig]: https://docs.rs/statig
