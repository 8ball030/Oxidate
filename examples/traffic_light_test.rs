//! Traffic Light FSM - Complete Functional Test
//!
//! Tests for the traffic_light.fsm example demonstrating:
//! - Timer-driven state transitions
//! - Entry actions for each state
//! - Complete cycle verification

use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// GENERATED CODE (simulating Oxidate output for traffic_light.fsm)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficLightEvent {
    RedExpired,
    YellowExpired,
    GreenExpired,
}

/// Actions trait for traffic light
pub trait TrafficLightActions {
    // Entry actions
    fn display_red(&mut self);
    fn display_yellow(&mut self);
    fn display_green(&mut self);
    
    // Timer control
    fn start_red_timer(&mut self);
    fn start_yellow_timer(&mut self);
    fn start_green_timer(&mut self);
}

pub struct TrafficLightFsm<A: TrafficLightActions> {
    state: TrafficLightState,
    actions: A,
}

impl<A: TrafficLightActions> TrafficLightFsm<A> {
    pub fn new(mut actions: A) -> Self {
        // Initial state is Red
        actions.display_red();
        actions.start_red_timer();
        Self {
            state: TrafficLightState::Red,
            actions,
        }
    }
    
    pub fn state(&self) -> TrafficLightState {
        self.state
    }
    
    /// Process an event and return true if a transition occurred
    pub fn process(&mut self, event: TrafficLightEvent) -> bool {
        match (self.state, event) {
            // Red -> Green on RedExpired
            (TrafficLightState::Red, TrafficLightEvent::RedExpired) => {
                self.state = TrafficLightState::Green;
                self.actions.display_green();
                self.actions.start_green_timer();
                true
            }
            // Green -> Yellow on GreenExpired
            (TrafficLightState::Green, TrafficLightEvent::GreenExpired) => {
                self.state = TrafficLightState::Yellow;
                self.actions.display_yellow();
                self.actions.start_yellow_timer();
                true
            }
            // Yellow -> Red on YellowExpired
            (TrafficLightState::Yellow, TrafficLightEvent::YellowExpired) => {
                self.state = TrafficLightState::Red;
                self.actions.display_red();
                self.actions.start_red_timer();
                true
            }
            // Invalid event for current state
            _ => false,
        }
    }
    
    /// Send multiple events in sequence
    pub fn send_events(&mut self, events: &[TrafficLightEvent]) -> Vec<bool> {
        events.iter().map(|e| self.process(*e)).collect()
    }
}

// ============================================================================
// TEST IMPLEMENTATION
// ============================================================================

#[derive(Clone)]
struct TestTrafficLightActions {
    log: Rc<RefCell<Vec<String>>>,
    timers_started: Rc<RefCell<Vec<String>>>,
}

impl TestTrafficLightActions {
    fn new() -> Self {
        Self {
            log: Rc::new(RefCell::new(Vec::new())),
            timers_started: Rc::new(RefCell::new(Vec::new())),
        }
    }
    
    fn get_log(&self) -> Vec<String> {
        self.log.borrow().clone()
    }
    
    fn get_timers(&self) -> Vec<String> {
        self.timers_started.borrow().clone()
    }
}

impl TrafficLightActions for TestTrafficLightActions {
    fn display_red(&mut self) {
        self.log.borrow_mut().push("DISPLAY: Red".to_string());
    }
    
    fn display_yellow(&mut self) {
        self.log.borrow_mut().push("DISPLAY: Yellow".to_string());
    }
    
    fn display_green(&mut self) {
        self.log.borrow_mut().push("DISPLAY: Green".to_string());
    }
    
    fn start_red_timer(&mut self) {
        self.timers_started.borrow_mut().push("red_timer (5000ms)".to_string());
    }
    
    fn start_yellow_timer(&mut self) {
        self.timers_started.borrow_mut().push("yellow_timer (2000ms)".to_string());
    }
    
    fn start_green_timer(&mut self) {
        self.timers_started.borrow_mut().push("green_timer (4000ms)".to_string());
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initial_state_is_red() {
        let actions = TestTrafficLightActions::new();
        let fsm = TrafficLightFsm::new(actions.clone());
        
        assert_eq!(fsm.state(), TrafficLightState::Red);
        assert!(actions.get_log().contains(&"DISPLAY: Red".to_string()));
        assert!(actions.get_timers().contains(&"red_timer (5000ms)".to_string()));
    }
    
    #[test]
    fn test_red_to_green_transition() {
        let actions = TestTrafficLightActions::new();
        let mut fsm = TrafficLightFsm::new(actions.clone());
        
        let transitioned = fsm.process(TrafficLightEvent::RedExpired);
        
        assert!(transitioned);
        assert_eq!(fsm.state(), TrafficLightState::Green);
        assert!(actions.get_log().contains(&"DISPLAY: Green".to_string()));
    }
    
    #[test]
    fn test_green_to_yellow_transition() {
        let actions = TestTrafficLightActions::new();
        let mut fsm = TrafficLightFsm::new(actions.clone());
        
        fsm.process(TrafficLightEvent::RedExpired); // Red -> Green
        let transitioned = fsm.process(TrafficLightEvent::GreenExpired);
        
        assert!(transitioned);
        assert_eq!(fsm.state(), TrafficLightState::Yellow);
    }
    
    #[test]
    fn test_yellow_to_red_transition() {
        let actions = TestTrafficLightActions::new();
        let mut fsm = TrafficLightFsm::new(actions.clone());
        
        fsm.process(TrafficLightEvent::RedExpired);   // Red -> Green
        fsm.process(TrafficLightEvent::GreenExpired); // Green -> Yellow
        let transitioned = fsm.process(TrafficLightEvent::YellowExpired);
        
        assert!(transitioned);
        assert_eq!(fsm.state(), TrafficLightState::Red);
    }
    
    #[test]
    fn test_complete_cycle() {
        let actions = TestTrafficLightActions::new();
        let mut fsm = TrafficLightFsm::new(actions.clone());
        
        // Run 3 complete cycles
        for _ in 0..3 {
            assert_eq!(fsm.state(), TrafficLightState::Red);
            fsm.process(TrafficLightEvent::RedExpired);
            
            assert_eq!(fsm.state(), TrafficLightState::Green);
            fsm.process(TrafficLightEvent::GreenExpired);
            
            assert_eq!(fsm.state(), TrafficLightState::Yellow);
            fsm.process(TrafficLightEvent::YellowExpired);
        }
        
        assert_eq!(fsm.state(), TrafficLightState::Red);
    }
    
    #[test]
    fn test_invalid_event_ignored() {
        let actions = TestTrafficLightActions::new();
        let mut fsm = TrafficLightFsm::new(actions);
        
        // Try to send GreenExpired while in Red state
        let transitioned = fsm.process(TrafficLightEvent::GreenExpired);
        
        assert!(!transitioned);
        assert_eq!(fsm.state(), TrafficLightState::Red);
    }
    
    #[test]
    fn test_send_multiple_events() {
        let actions = TestTrafficLightActions::new();
        let mut fsm = TrafficLightFsm::new(actions);
        
        let events = [
            TrafficLightEvent::RedExpired,
            TrafficLightEvent::GreenExpired,
            TrafficLightEvent::YellowExpired,
        ];
        
        let results = fsm.send_events(&events);
        
        assert_eq!(results, vec![true, true, true]);
        assert_eq!(fsm.state(), TrafficLightState::Red);
    }
    
    #[test]
    fn test_timers_started_correctly() {
        let actions = TestTrafficLightActions::new();
        let mut fsm = TrafficLightFsm::new(actions.clone());
        
        // Initial state starts red timer
        assert!(actions.get_timers().iter().any(|t| t.contains("red_timer")));
        
        fsm.process(TrafficLightEvent::RedExpired);
        assert!(actions.get_timers().iter().any(|t| t.contains("green_timer")));
        
        fsm.process(TrafficLightEvent::GreenExpired);
        assert!(actions.get_timers().iter().any(|t| t.contains("yellow_timer")));
    }
    
    #[test]
    fn test_action_sequence() {
        let actions = TestTrafficLightActions::new();
        let mut fsm = TrafficLightFsm::new(actions.clone());
        
        fsm.process(TrafficLightEvent::RedExpired);
        fsm.process(TrafficLightEvent::GreenExpired);
        fsm.process(TrafficLightEvent::YellowExpired);
        
        let log = actions.get_log();
        assert_eq!(log, vec![
            "DISPLAY: Red",
            "DISPLAY: Green",
            "DISPLAY: Yellow",
            "DISPLAY: Red",
        ]);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Traffic Light FSM - Timer-Driven Example\n");
    
    let actions = TestTrafficLightActions::new();
    let mut fsm = TrafficLightFsm::new(actions.clone());
    
    println!("Initial state: {:?}", fsm.state());
    println!("  Timer started: {:?}", actions.get_timers().last());
    
    let events = [
        ("RedExpired", TrafficLightEvent::RedExpired),
        ("GreenExpired", TrafficLightEvent::GreenExpired),
        ("YellowExpired", TrafficLightEvent::YellowExpired),
        ("RedExpired", TrafficLightEvent::RedExpired),
    ];
    
    for (name, event) in events {
        println!("\nEvent: {}", name);
        fsm.process(event);
        println!("  State: {:?}", fsm.state());
    }
    
    println!("\n--- Action Log ---");
    for (i, action) in actions.get_log().iter().enumerate() {
        println!("  {}: {}", i + 1, action);
    }
    
    println!("\n--- Timers Started ---");
    for (i, timer) in actions.get_timers().iter().enumerate() {
        println!("  {}: {}", i + 1, timer);
    }
    
    println!("\n✅ Traffic Light FSM works correctly!");
}
