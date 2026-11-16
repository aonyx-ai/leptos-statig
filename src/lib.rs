//! Reactive wrapper for [statig][statig] state machines in [Leptos][leptos]
//!
//! Bridges statig's synchronous finite state machines with Leptos reactive
//! signals, so state transitions automatically trigger UI updates.
//!
//! # Examples
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_statig::ReactiveStateMachine;
//!
//! let fsm = ReactiveStateMachine::new(MyMachine::default());
//!
//! // Reactive — re-runs when state changes
//! let current = move || fsm.state();
//!
//! // Dispatch events to drive transitions
//! fsm.dispatch(&Event::Toggle);
//! ```
//!
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
/// # Examples
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_statig::ReactiveStateMachine;
/// # use statig::blocking::Outcome;
/// # use statig::prelude::*;
/// # #[derive(Clone, PartialEq, Debug)]
/// # enum Light { Off, On }
/// # struct Flip;
/// # struct Switch { count: u32 }
/// # impl IntoStateMachine for Switch {
/// #     type State = Light;
/// #     type Superstate<'a> = ();
/// #     type Event<'a> = Flip;
/// #     type Context<'a> = ();
/// #     fn initial() -> Light { Light::Off }
/// # }
/// # impl State<Switch> for Light {
/// #     fn call_handler(&mut self, _s: &mut Switch, _e: &Flip, _c: &mut ()) -> Outcome<Self> {
/// #         match self {
/// #             Light::Off => Outcome::Transition(Light::On),
/// #             Light::On => Outcome::Transition(Light::Off),
/// #         }
/// #     }
/// # }
/// # let owner = Owner::new();
/// # owner.set();
/// let fsm = ReactiveStateMachine::new(Switch { count: 0 });
/// let copy = fsm;
/// fsm.dispatch(&Flip);
/// assert_eq!(copy.state(), Light::On);
/// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_statig::ReactiveStateMachine;
    /// # use statig::blocking::Outcome;
    /// # use statig::prelude::*;
    /// # #[derive(Clone, PartialEq, Debug)]
    /// # enum Light { Off, On }
    /// # struct Flip;
    /// # struct Switch { count: u32 }
    /// # impl IntoStateMachine for Switch {
    /// #     type State = Light;
    /// #     type Superstate<'a> = ();
    /// #     type Event<'a> = Flip;
    /// #     type Context<'a> = ();
    /// #     fn initial() -> Light { Light::Off }
    /// # }
    /// # impl State<Switch> for Light {
    /// #     fn call_handler(&mut self, _s: &mut Switch, _e: &Flip, _c: &mut ()) -> Outcome<Self> {
    /// #         match self {
    /// #             Light::Off => Outcome::Transition(Light::On),
    /// #             Light::On => Outcome::Transition(Light::Off),
    /// #         }
    /// #     }
    /// # }
    /// # let owner = Owner::new();
    /// # owner.set();
    /// let fsm = ReactiveStateMachine::new(Switch { count: 0 });
    /// assert_eq!(fsm.state(), Light::Off);
    /// ```
    pub fn new(machine: M) -> Self {
        Self {
            inner: RwSignal::new(machine.uninitialized_state_machine().init().into()),
        }
    }

    /// Returns the current state (reactive read)
    ///
    /// Calling this inside a reactive context (e.g. a component body or
    /// [`Effect`]) subscribes to state changes.
    /// # Examples
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_statig::ReactiveStateMachine;
    /// # use statig::blocking::Outcome;
    /// # use statig::prelude::*;
    /// # #[derive(Clone, PartialEq, Debug)]
    /// # enum Light { Off, On }
    /// # struct Flip;
    /// # struct Switch { count: u32 }
    /// # impl IntoStateMachine for Switch {
    /// #     type State = Light;
    /// #     type Superstate<'a> = ();
    /// #     type Event<'a> = Flip;
    /// #     type Context<'a> = ();
    /// #     fn initial() -> Light { Light::Off }
    /// # }
    /// # impl State<Switch> for Light {
    /// #     fn call_handler(&mut self, _s: &mut Switch, _e: &Flip, _c: &mut ()) -> Outcome<Self> {
    /// #         match self {
    /// #             Light::Off => Outcome::Transition(Light::On),
    /// #             Light::On => Outcome::Transition(Light::Off),
    /// #         }
    /// #     }
    /// # }
    /// # let owner = Owner::new();
    /// # owner.set();
    /// let fsm = ReactiveStateMachine::new(Switch { count: 0 });
    /// assert_eq!(fsm.state(), Light::Off);
    /// ```
    ///
    /// [`Effect`]: leptos::prelude::Effect
    pub fn state(&self) -> M::State {
        self.inner.with(|m| m.state().clone())
    }

    /// Dispatches an event, potentially triggering a state transition
    ///
    /// This is a reactive write. Any subscriber of [`state`] is notified.
    ///
    /// [`state`]: Self::state
    ///
    /// # Examples
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_statig::ReactiveStateMachine;
    /// # use statig::blocking::Outcome;
    /// # use statig::prelude::*;
    /// # #[derive(Clone, PartialEq, Debug)]
    /// # enum Light { Off, On }
    /// # struct Flip;
    /// # struct Switch { count: u32 }
    /// # impl IntoStateMachine for Switch {
    /// #     type State = Light;
    /// #     type Superstate<'a> = ();
    /// #     type Event<'a> = Flip;
    /// #     type Context<'a> = ();
    /// #     fn initial() -> Light { Light::Off }
    /// # }
    /// # impl State<Switch> for Light {
    /// #     fn call_handler(&mut self, _s: &mut Switch, _e: &Flip, _c: &mut ()) -> Outcome<Self> {
    /// #         match self {
    /// #             Light::Off => Outcome::Transition(Light::On),
    /// #             Light::On => Outcome::Transition(Light::Off),
    /// #         }
    /// #     }
    /// # }
    /// # let owner = Owner::new();
    /// # owner.set();
    /// let fsm = ReactiveStateMachine::new(Switch { count: 0 });
    /// fsm.dispatch(&Flip);
    /// assert_eq!(fsm.state(), Light::On);
    /// ```
    pub fn dispatch(&self, event: &M::Event<'_>) {
        self.inner.update(|m| {
            m.handle(event);
        });
    }

    /// Provides read-only access to the underlying machine data
    ///
    /// # Examples
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_statig::ReactiveStateMachine;
    /// # use statig::blocking::Outcome;
    /// # use statig::prelude::*;
    /// # #[derive(Clone, PartialEq, Debug)]
    /// # enum Light { Off, On }
    /// # struct Flip;
    /// # struct Switch { count: u32 }
    /// # impl IntoStateMachine for Switch {
    /// #     type State = Light;
    /// #     type Superstate<'a> = ();
    /// #     type Event<'a> = Flip;
    /// #     type Context<'a> = ();
    /// #     fn initial() -> Light { Light::Off }
    /// # }
    /// # impl State<Switch> for Light {
    /// #     fn call_handler(&mut self, _s: &mut Switch, _e: &Flip, _c: &mut ()) -> Outcome<Self> {
    /// #         match self {
    /// #             Light::Off => Outcome::Transition(Light::On),
    /// #             Light::On => Outcome::Transition(Light::Off),
    /// #         }
    /// #     }
    /// # }
    /// # let owner = Owner::new();
    /// # owner.set();
    /// let fsm = ReactiveStateMachine::new(Switch { count: 42 });
    /// assert_eq!(fsm.with_inner(|m| m.count), 42);
    /// ```
    pub fn with_inner<R>(&self, f: impl FnOnce(&M) -> R) -> R {
        self.inner.with(|m| f(m.inner()))
    }
}

/// Clones the handle to the shared signal
///
/// A manual impl avoids the `M: Clone` bound that `#[derive]` would add.
/// [`RwSignal`] is [`Copy`] regardless of its inner type. The wrapper should
/// be too
impl<M> Clone for ReactiveStateMachine<M>
where
    M: IntoStateMachine,
    M::State: Clone + 'static,
{
    fn clone(&self) -> Self {
        *self
    }
}

/// Copies the handle to the shared signal
///
/// A manual impl avoids the `M: Copy` bound that `#[derive]` would add.
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
/// # Examples
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_statig::{provide_fsm, use_fsm};
/// # use statig::blocking::Outcome;
/// # use statig::prelude::*;
/// # #[derive(Clone, PartialEq, Debug)]
/// # enum Light { Off, On }
/// # struct Flip;
/// # struct Switch { count: u32 }
/// # impl IntoStateMachine for Switch {
/// #     type State = Light;
/// #     type Superstate<'a> = ();
/// #     type Event<'a> = Flip;
/// #     type Context<'a> = ();
/// #     fn initial() -> Light { Light::Off }
/// # }
/// # impl State<Switch> for Light {
/// #     fn call_handler(&mut self, _s: &mut Switch, _e: &Flip, _c: &mut ()) -> Outcome<Self> {
/// #         match self {
/// #             Light::Off => Outcome::Transition(Light::On),
/// #             Light::On => Outcome::Transition(Light::Off),
/// #         }
/// #     }
/// # }
/// # let owner = Owner::new();
/// # owner.set();
/// let fsm = provide_fsm(Switch { count: 0 });
/// assert_eq!(fsm.state(), Light::Off);
/// assert_eq!(use_fsm::<Switch>().state(), Light::Off);
/// ```
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
/// # Examples
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_statig::{provide_fsm, use_fsm};
/// # use statig::blocking::Outcome;
/// # use statig::prelude::*;
/// # #[derive(Clone, PartialEq, Debug)]
/// # enum Light { Off, On }
/// # struct Flip;
/// # struct Switch { count: u32 }
/// # impl IntoStateMachine for Switch {
/// #     type State = Light;
/// #     type Superstate<'a> = ();
/// #     type Event<'a> = Flip;
/// #     type Context<'a> = ();
/// #     fn initial() -> Light { Light::Off }
/// # }
/// # impl State<Switch> for Light {
/// #     fn call_handler(&mut self, _s: &mut Switch, _e: &Flip, _c: &mut ()) -> Outcome<Self> {
/// #         match self {
/// #             Light::Off => Outcome::Transition(Light::On),
/// #             Light::On => Outcome::Transition(Light::Off),
/// #         }
/// #     }
/// # }
/// # let owner = Owner::new();
/// # owner.set();
/// provide_fsm(Switch { count: 0 });
/// let fsm = use_fsm::<Switch>();
/// assert_eq!(fsm.state(), Light::Off);
/// ```
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
    fn copy_shares_underlying_signal() {
        let owner = Owner::new();
        owner.set();

        let fsm_a = ReactiveStateMachine::new(ToggleMachine { count: 0 });
        let fsm_b = fsm_a;

        fsm_a.dispatch(&ToggleEvent::Toggle);

        assert_eq!(fsm_b.state(), ToggleState::On);
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
    fn new_initializes_to_initial_state() {
        let owner = Owner::new();
        owner.set();

        let fsm = ReactiveStateMachine::new(ToggleMachine { count: 0 });

        assert_eq!(fsm.state(), ToggleState::Off);
    }

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
    fn with_inner_reads_machine_data() {
        let owner = Owner::new();
        owner.set();

        let fsm = ReactiveStateMachine::new(ToggleMachine { count: 42 });

        let count = fsm.with_inner(|m| m.count);

        assert_eq!(count, 42);
    }

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
