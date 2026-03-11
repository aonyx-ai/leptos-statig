//! Reactive wrapper for [statig][statig] state machines in
//! [Leptos][leptos].
//!
//! Bridges statig's synchronous finite state machines with Leptos reactive
//! signals, so state transitions automatically trigger UI updates.
//!
//! # Examples
//!
//! Define a state machine with statig's `#[state_machine]` macro, then wrap
//! it in a [`ReactiveStateMachine`] to get reactive reads and writes:
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_statig::ReactiveStateMachine;
//! use statig::prelude::*;
//!
//! struct Toggle;
//!
//! #[state_machine(initial = "State::off()", state(derive(Clone, Debug)))]
//! impl Toggle {
//!     #[state]
//!     fn off(event: &Event) -> Outcome<State> {
//!         match event {
//!             Event::Toggle => Transition(State::on()),
//!         }
//!     }
//!
//!     #[state]
//!     fn on(event: &Event) -> Outcome<State> {
//!         match event {
//!             Event::Toggle => Transition(State::off()),
//!         }
//!     }
//! }
//!
//! enum Event {
//!     Toggle,
//! }
//!
//! #[component]
//! fn ToggleButton() -> impl IntoView {
//!     let fsm = ReactiveStateMachine::new(Toggle);
//!
//!     view! {
//!         <button on:click=move |_| fsm.dispatch(&Event::Toggle)>
//!             {move || format!("{:?}", fsm.state())}
//!         </button>
//!     }
//! }
//! ```
//!
//! See the [`examples/`][examples] directory for runnable demos.
//!
//! [examples]: https://github.com/aonyx-ai/leptos-statig/tree/main/examples
//! [leptos]: https://docs.rs/leptos
//! [statig]: https://docs.rs/statig

use leptos::prelude::*;
use statig::blocking::{IntoStateMachine, IntoStateMachineExt, StateMachine};
use statig::prelude::{State, Superstate};

/// Reactive wrapper around a statig state machine
///
/// Wraps a [`StateMachine<M>`] in an [`RwSignal`] so that reads are tracked
/// and writes notify the reactive graph. Because the wrapper only holds a
/// signal handle it is both [`Clone`] and [`Copy`].
///
/// [`StateMachine<M>`]: statig::blocking::StateMachine
pub struct ReactiveStateMachine<M>
where
    M: IntoStateMachine,
    M::State: Clone + 'static,
{
    inner: RwSignal<StateMachine<M>>,
}

impl<M> ReactiveStateMachine<M>
where
    M: IntoStateMachineExt + Send + Sync + 'static,
    for<'ctx> M: IntoStateMachine<Context<'ctx> = ()>,
    M::State: State<M> + Clone + Send + Sync,
    for<'a> M::Superstate<'a>: Superstate<M>,
{
    /// Creates a new reactive state machine, initializing it immediately
    pub fn new(machine: M) -> Self {
        Self {
            inner: RwSignal::new(machine.uninitialized_state_machine().init().into()),
        }
    }

    /// Returns the current state (reactive read)
    ///
    /// Calling this inside a reactive context (e.g. a component body or
    /// [`Effect`]) subscribes to state changes.
    ///
    /// [`Effect`]: leptos::prelude::Effect
    pub fn state(&self) -> M::State {
        self.inner.with(|m| m.state().clone())
    }

    /// Dispatches an event, potentially triggering a state transition
    ///
    /// This is a reactive write — any subscribers of [`state`](Self::state)
    /// will be notified.
    pub fn dispatch(&self, event: &M::Event<'_>) {
        self.inner.update(|m| {
            m.handle(event);
        });
    }

    /// Provides read-only access to the underlying machine data
    pub fn with_inner<R>(&self, f: impl FnOnce(&M) -> R) -> R {
        self.inner.with(|m| f(m.inner()))
    }
}

// Manual Clone/Copy impls avoid the `M: Clone`/`M: Copy` bounds that
// #[derive] would add. RwSignal is Copy regardless of its inner type,
// so the wrapper should be too.
impl<M> Clone for ReactiveStateMachine<M>
where
    M: IntoStateMachine,
    M::State: Clone + 'static,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for ReactiveStateMachine<M>
where
    M: IntoStateMachine,
    M::State: Clone + 'static,
{
}

/// Provides a [`ReactiveStateMachine<M>`] via Leptos context
///
/// Makes the FSM available to any descendant component via [`use_fsm`].
///
/// [`ReactiveStateMachine<M>`]: ReactiveStateMachine
pub fn provide_fsm<M>(machine: M) -> ReactiveStateMachine<M>
where
    M: IntoStateMachineExt + Send + Sync + 'static,
    for<'ctx> M: IntoStateMachine<Context<'ctx> = ()>,
    M::State: State<M> + Clone + Send + Sync,
    for<'a> M::Superstate<'a>: Superstate<M>,
{
    let fsm = ReactiveStateMachine::new(machine);
    provide_context(fsm);
    fsm
}

/// Retrieves a [`ReactiveStateMachine<M>`] from Leptos context
///
/// # Panics
///
/// Panics if no FSM of the matching type was provided by an ancestor
/// component. See [`provide_fsm`].
///
/// [`ReactiveStateMachine<M>`]: ReactiveStateMachine
pub fn use_fsm<M>() -> ReactiveStateMachine<M>
where
    M: IntoStateMachine + 'static,
    M::State: Clone + 'static,
{
    expect_context::<ReactiveStateMachine<M>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::Owner;
    use statig::blocking::Outcome;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ReactiveStateMachine<ToggleMachine>>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ReactiveStateMachine<ToggleMachine>>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<ReactiveStateMachine<ToggleMachine>>();
    }

    #[test]
    fn new_initializes_to_initial_state() {
        let owner = Owner::new();
        owner.set();

        let fsm = ReactiveStateMachine::new(ToggleMachine { count: 0 });

        assert_eq!(fsm.state(), ToggleState::Off);
    }

    #[test]
    fn dispatch_transitions_state() {
        let owner = Owner::new();
        owner.set();

        let fsm = ReactiveStateMachine::new(ToggleMachine { count: 0 });

        fsm.dispatch(&ToggleEvent::Toggle);

        assert_eq!(fsm.state(), ToggleState::On);
    }

    #[test]
    fn dispatch_without_matching_transition_preserves_state() {
        let owner = Owner::new();
        owner.set();

        let fsm = ReactiveStateMachine::new(ToggleMachine { count: 0 });

        fsm.dispatch(&ToggleEvent::Reset);

        assert_eq!(fsm.state(), ToggleState::Off);
    }

    #[test]
    fn dispatch_round_trip_returns_to_original_state() {
        let owner = Owner::new();
        owner.set();

        let fsm = ReactiveStateMachine::new(ToggleMachine { count: 0 });

        fsm.dispatch(&ToggleEvent::Toggle);
        fsm.dispatch(&ToggleEvent::Toggle);

        assert_eq!(fsm.state(), ToggleState::Off);
    }

    #[test]
    fn with_inner_reads_machine_data() {
        let owner = Owner::new();
        owner.set();

        let fsm = ReactiveStateMachine::new(ToggleMachine { count: 42 });

        let count = fsm.with_inner(|m| m.count);

        assert_eq!(count, 42);
    }

    #[test]
    fn copy_shares_underlying_signal() {
        let owner = Owner::new();
        owner.set();

        let fsm_a = ReactiveStateMachine::new(ToggleMachine { count: 0 });
        let fsm_b = fsm_a;

        fsm_a.dispatch(&ToggleEvent::Toggle);

        assert_eq!(fsm_b.state(), ToggleState::On);
    }

    // -- Test fixtures -------------------------------------------------------

    #[derive(Clone, Eq, PartialEq, Debug)]
    enum ToggleState {
        Off,
        On,
    }

    enum ToggleEvent {
        Toggle,
        Reset,
    }

    struct ToggleMachine {
        count: u32,
    }

    impl IntoStateMachine for ToggleMachine {
        type State = ToggleState;
        type Superstate<'a> = ();
        type Event<'a> = ToggleEvent;
        type Context<'a> = ();

        fn initial() -> ToggleState {
            ToggleState::Off
        }
    }

    impl State<ToggleMachine> for ToggleState {
        fn call_handler(
            &mut self,
            _shared_storage: &mut ToggleMachine,
            event: &ToggleEvent,
            _context: &mut (),
        ) -> Outcome<Self> {
            match (self, event) {
                (ToggleState::Off, ToggleEvent::Toggle) => Outcome::Transition(ToggleState::On),
                (ToggleState::On, ToggleEvent::Toggle) => Outcome::Transition(ToggleState::Off),
                (ToggleState::On, ToggleEvent::Reset) => Outcome::Transition(ToggleState::Off),
                (ToggleState::Off, ToggleEvent::Reset) => Outcome::Handled,
            }
        }
    }
}
