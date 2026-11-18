use harness::{Observed, Tokens, Transitionable};
use signal::Following;

use crate::{Changed, Observations};

impl Following for Observations {
    type Change = Changed;

    fn after(mut state: Self, change: Self::Change) -> Self {
        if let Changed::Transition(transition) = change {
            if let Some(observed) = state
                .0
                .iter_mut()
                .find(|observed| observed.flow_id == transition.flow_id)
            {
                observed.after(transition.event);
                observed.state = transition.state;
            } else {
                let mut observed = Observed {
                    flow_id: transition.flow_id,
                    state: transition.state.clone(),
                    tokens: Tokens(0),
                    background_work_vector: vec![],
                };
                observed.after(transition.event);
                observed.state = transition.state;
                state.0.push(observed);
            }
        }
        state
    }
}
