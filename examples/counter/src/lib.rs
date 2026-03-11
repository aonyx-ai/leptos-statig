//! Counter example — a Leptos CSR app backed by a statig state machine.
//!
//! Demonstrates how `ReactiveStateMachine` bridges statig with Leptos:
//! state transitions automatically re-render the UI.
//!
//! Run with: `cd examples/counter && trunk serve`

use leptos::prelude::*;
use leptos_statig::ReactiveStateMachine;
use statig::prelude::*;
use statig::Outcome::Transition;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let fsm = ReactiveStateMachine::new(Counter { limit: 5 });

    let state_display = move || match fsm.state() {
        State::Idle {} => "Ready".to_string(),
        State::Counting { count } => format!("{count}"),
        State::Done { _count: count } => format!("{count} — done!"),
    };

    let is_done = move || matches!(fsm.state(), State::Done { .. });

    view! {
        <h1>"leptos-statig counter"</h1>
        <p class:done=is_done class="state">{state_display}</p>
        <div>
            <button on:click=move |_| fsm.dispatch(&Event::Increment)>
                "Increment"
            </button>
            <button on:click=move |_| fsm.dispatch(&Event::Reset)>
                "Reset"
            </button>
        </div>
    }
}

struct Counter {
    limit: u32,
}

enum Event {
    Increment,
    Reset,
}

#[state_machine(initial = "State::idle()", state(derive(Clone, Debug)))]
impl Counter {
    #[state]
    fn idle(event: &Event) -> Outcome<State> {
        match event {
            Event::Increment => Transition(State::counting(1)),
            Event::Reset => Handled,
        }
    }

    #[state]
    fn counting(&self, count: &u32, event: &Event) -> Outcome<State> {
        match event {
            Event::Increment => {
                let next = count + 1;
                if next >= self.limit {
                    Transition(State::done(next))
                } else {
                    Transition(State::counting(next))
                }
            }
            Event::Reset => Transition(State::idle()),
        }
    }

    #[state]
    fn done(_count: &u32, event: &Event) -> Outcome<State> {
        match event {
            Event::Reset => Transition(State::idle()),
            Event::Increment => Handled,
        }
    }
}
