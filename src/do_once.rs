//! DoOnceGate is a simple state machine that can be used to ensure that logic is
//! only executed once before it requires a reset.

#[derive(Debug, Default, PartialEq)]
enum DoOnceState {
    #[default]
    Inactive,
    Active,
    WaitingForReset,
}

#[derive(Debug, Default)]
pub struct DoOnceGate {
    state: DoOnceState,
}

impl DoOnceGate {
    /// Set the state to active
    pub fn set_active(&mut self) {
        self.state = DoOnceState::Active;
    }

    /// Set the state to waiting for reset
    pub fn set_waiting(&mut self) {
        self.state = DoOnceState::WaitingForReset;
    }

    /// Reset the state to inactive
    pub fn reset(&mut self) {
        self.state = DoOnceState::Inactive;
    }

    /// Check if the state is active
    pub fn is_active(&self) -> bool {
        self.state == DoOnceState::Active && !self.is_waiting_for_reset()
    }

    /// Check if the state is waiting for reset
    pub fn is_waiting_for_reset(&self) -> bool {
        self.state == DoOnceState::WaitingForReset
    }

    /// Check if the state is inactive
    pub fn is_inactive(&self) -> bool {
        self.state == DoOnceState::Inactive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_once_gate() {
        let mut do_once_gate = DoOnceGate::default();
        assert_eq!(do_once_gate.is_active(), false);
        assert_eq!(do_once_gate.is_waiting_for_reset(), false);

        do_once_gate.set_active();
        assert_eq!(do_once_gate.is_active(), true);
        assert_eq!(do_once_gate.is_waiting_for_reset(), false);

        do_once_gate.set_waiting();
        assert_eq!(do_once_gate.is_active(), false);
        assert_eq!(do_once_gate.is_waiting_for_reset(), true);

        do_once_gate.reset();
        assert_eq!(do_once_gate.is_active(), false);
        assert_eq!(do_once_gate.is_waiting_for_reset(), false);
    }
}
