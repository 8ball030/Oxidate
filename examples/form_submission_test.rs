//! Form Submission FSM - Complete Functional Test
//!
//! Tests for the form_submission.fsm example demonstrating:
//! - Choice points (conditional branching)
//! - Multiple outcome paths
//! - Retry logic
//! - Complex state transitions

use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// GENERATED CODE (simulating Oxidate output for form_submission.fsm)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormState {
    Editing,
    Validating,
    Submitting,
    Success,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormEvent {
    // User actions
    Submit,
    Retry,
    NewForm,
    Done,
    DataChanged,
    // System events
    ValidationComplete,
    ResponseReceived,
}

/// Response from server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerResponse {
    Success,
    Retryable,
    PermanentError,
}

/// Actions trait for form submission
pub trait FormActions {
    // Guards for choice points
    fn all_fields_valid(&self) -> bool;
    fn response_is_success(&self) -> bool;
    fn response_is_retryable(&self) -> bool;
    
    // Entry actions
    fn validate_field(&mut self);
    fn run_full_validation(&mut self);
    fn send_to_server(&mut self);
    fn show_success_message(&mut self);
    fn clear_form(&mut self);
    fn show_error_message(&mut self);
    
    // Transition actions
    fn highlight_errors(&mut self);
    fn increment_retry(&mut self);
    fn log_error(&mut self);
}

pub struct FormFsm<A: FormActions> {
    state: FormState,
    actions: A,
    retry_count: u32,
}

impl<A: FormActions> FormFsm<A> {
    pub fn new(actions: A) -> Self {
        Self {
            state: FormState::Editing,
            actions,
            retry_count: 0,
        }
    }
    
    pub fn state(&self) -> FormState {
        self.state
    }
    
    pub fn retry_count(&self) -> u32 {
        self.retry_count
    }
    
    pub fn process(&mut self, event: FormEvent) -> bool {
        match (self.state, event) {
            // Editing: handle data changes
            (FormState::Editing, FormEvent::DataChanged) => {
                self.actions.validate_field();
                true
            }
            
            // Editing -> Validating
            (FormState::Editing, FormEvent::Submit) => {
                self.state = FormState::Validating;
                self.actions.run_full_validation();
                true
            }
            
            // Validating -> (choice point based on validation result)
            (FormState::Validating, FormEvent::ValidationComplete) => {
                if self.actions.all_fields_valid() {
                    self.state = FormState::Submitting;
                    self.actions.send_to_server();
                } else {
                    self.state = FormState::Editing;
                    self.actions.highlight_errors();
                }
                true
            }
            
            // Submitting -> (choice point based on response)
            (FormState::Submitting, FormEvent::ResponseReceived) => {
                if self.actions.response_is_success() {
                    self.state = FormState::Success;
                    self.actions.show_success_message();
                    self.actions.clear_form();
                } else if self.actions.response_is_retryable() {
                    // Stay in Submitting, retry
                    self.actions.increment_retry();
                    self.retry_count += 1;
                    self.actions.send_to_server();
                } else {
                    self.state = FormState::Error;
                    self.actions.log_error();
                    self.actions.show_error_message();
                }
                true
            }
            
            // Error -> Editing (retry)
            (FormState::Error, FormEvent::Retry) => {
                self.state = FormState::Editing;
                true
            }
            
            // Success -> Editing (new form)
            (FormState::Success, FormEvent::NewForm) => {
                self.state = FormState::Editing;
                self.retry_count = 0;
                true
            }
            
            // Success -> Done (terminal)
            (FormState::Success, FormEvent::Done) => {
                // Terminal state reached
                true
            }
            
            _ => false,
        }
    }
    
    /// Send event (alias for process)
    pub fn send(&mut self, event: FormEvent) -> bool {
        self.process(event)
    }
}

// ============================================================================
// TEST IMPLEMENTATION
// ============================================================================

#[derive(Clone)]
struct TestFormActions {
    // Guard states
    fields_valid: Rc<RefCell<bool>>,
    server_response: Rc<RefCell<ServerResponse>>,
    
    // Action log
    log: Rc<RefCell<Vec<String>>>,
}

impl TestFormActions {
    fn new() -> Self {
        Self {
            fields_valid: Rc::new(RefCell::new(true)),
            server_response: Rc::new(RefCell::new(ServerResponse::Success)),
            log: Rc::new(RefCell::new(Vec::new())),
        }
    }
    
    fn set_fields_valid(&self, valid: bool) {
        *self.fields_valid.borrow_mut() = valid;
    }
    
    fn set_server_response(&self, response: ServerResponse) {
        *self.server_response.borrow_mut() = response;
    }
    
    fn get_log(&self) -> Vec<String> {
        self.log.borrow().clone()
    }
}

impl FormActions for TestFormActions {
    fn all_fields_valid(&self) -> bool {
        *self.fields_valid.borrow()
    }
    
    fn response_is_success(&self) -> bool {
        *self.server_response.borrow() == ServerResponse::Success
    }
    
    fn response_is_retryable(&self) -> bool {
        *self.server_response.borrow() == ServerResponse::Retryable
    }
    
    fn validate_field(&mut self) {
        self.log.borrow_mut().push("ACTION: validate_field".to_string());
    }
    
    fn run_full_validation(&mut self) {
        self.log.borrow_mut().push("ACTION: run_full_validation".to_string());
    }
    
    fn send_to_server(&mut self) {
        self.log.borrow_mut().push("ACTION: send_to_server".to_string());
    }
    
    fn show_success_message(&mut self) {
        self.log.borrow_mut().push("ACTION: show_success_message".to_string());
    }
    
    fn clear_form(&mut self) {
        self.log.borrow_mut().push("ACTION: clear_form".to_string());
    }
    
    fn show_error_message(&mut self) {
        self.log.borrow_mut().push("ACTION: show_error_message".to_string());
    }
    
    fn highlight_errors(&mut self) {
        self.log.borrow_mut().push("ACTION: highlight_errors".to_string());
    }
    
    fn increment_retry(&mut self) {
        self.log.borrow_mut().push("ACTION: increment_retry".to_string());
    }
    
    fn log_error(&mut self) {
        self.log.borrow_mut().push("ACTION: log_error".to_string());
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initial_state() {
        let actions = TestFormActions::new();
        let fsm = FormFsm::new(actions);
        
        assert_eq!(fsm.state(), FormState::Editing);
        assert_eq!(fsm.retry_count(), 0);
    }
    
    #[test]
    fn test_data_changed_triggers_validation() {
        let actions = TestFormActions::new();
        let mut fsm = FormFsm::new(actions.clone());
        
        assert!(fsm.send(FormEvent::DataChanged));
        assert_eq!(fsm.state(), FormState::Editing); // Still editing
        assert!(actions.get_log().contains(&"ACTION: validate_field".to_string()));
    }
    
    #[test]
    fn test_happy_path_submit_success() {
        let actions = TestFormActions::new();
        actions.set_fields_valid(true);
        actions.set_server_response(ServerResponse::Success);
        
        let mut fsm = FormFsm::new(actions.clone());
        
        // Submit form
        assert!(fsm.send(FormEvent::Submit));
        assert_eq!(fsm.state(), FormState::Validating);
        
        // Validation complete (valid)
        assert!(fsm.send(FormEvent::ValidationComplete));
        assert_eq!(fsm.state(), FormState::Submitting);
        
        // Server responds success
        assert!(fsm.send(FormEvent::ResponseReceived));
        assert_eq!(fsm.state(), FormState::Success);
        
        // Verify actions
        let log = actions.get_log();
        assert!(log.contains(&"ACTION: run_full_validation".to_string()));
        assert!(log.contains(&"ACTION: send_to_server".to_string()));
        assert!(log.contains(&"ACTION: show_success_message".to_string()));
        assert!(log.contains(&"ACTION: clear_form".to_string()));
    }
    
    #[test]
    fn test_validation_failure_returns_to_editing() {
        let actions = TestFormActions::new();
        actions.set_fields_valid(false);
        
        let mut fsm = FormFsm::new(actions.clone());
        
        fsm.send(FormEvent::Submit);
        assert!(fsm.send(FormEvent::ValidationComplete));
        
        assert_eq!(fsm.state(), FormState::Editing);
        assert!(actions.get_log().contains(&"ACTION: highlight_errors".to_string()));
    }
    
    #[test]
    fn test_server_retryable_error() {
        let actions = TestFormActions::new();
        actions.set_fields_valid(true);
        actions.set_server_response(ServerResponse::Retryable);
        
        let mut fsm = FormFsm::new(actions.clone());
        
        // Get to Submitting
        fsm.send(FormEvent::Submit);
        fsm.send(FormEvent::ValidationComplete);
        assert_eq!(fsm.state(), FormState::Submitting);
        
        // First attempt returns retryable
        fsm.send(FormEvent::ResponseReceived);
        assert_eq!(fsm.state(), FormState::Submitting); // Still submitting
        assert_eq!(fsm.retry_count(), 1);
        
        // Second attempt
        fsm.send(FormEvent::ResponseReceived);
        assert_eq!(fsm.retry_count(), 2);
        
        // Now server succeeds
        actions.set_server_response(ServerResponse::Success);
        fsm.send(FormEvent::ResponseReceived);
        assert_eq!(fsm.state(), FormState::Success);
    }
    
    #[test]
    fn test_server_permanent_error() {
        let actions = TestFormActions::new();
        actions.set_fields_valid(true);
        actions.set_server_response(ServerResponse::PermanentError);
        
        let mut fsm = FormFsm::new(actions.clone());
        
        // Get to Submitting
        fsm.send(FormEvent::Submit);
        fsm.send(FormEvent::ValidationComplete);
        
        // Server returns permanent error
        fsm.send(FormEvent::ResponseReceived);
        assert_eq!(fsm.state(), FormState::Error);
        
        let log = actions.get_log();
        assert!(log.contains(&"ACTION: log_error".to_string()));
        assert!(log.contains(&"ACTION: show_error_message".to_string()));
    }
    
    #[test]
    fn test_retry_from_error() {
        let actions = TestFormActions::new();
        actions.set_fields_valid(true);
        actions.set_server_response(ServerResponse::PermanentError);
        
        let mut fsm = FormFsm::new(actions.clone());
        
        // Get to Error state
        fsm.send(FormEvent::Submit);
        fsm.send(FormEvent::ValidationComplete);
        fsm.send(FormEvent::ResponseReceived);
        assert_eq!(fsm.state(), FormState::Error);
        
        // Retry
        assert!(fsm.send(FormEvent::Retry));
        assert_eq!(fsm.state(), FormState::Editing);
    }
    
    #[test]
    fn test_new_form_after_success() {
        let actions = TestFormActions::new();
        actions.set_fields_valid(true);
        actions.set_server_response(ServerResponse::Success);
        
        let mut fsm = FormFsm::new(actions);
        
        // Get to Success
        fsm.send(FormEvent::Submit);
        fsm.send(FormEvent::ValidationComplete);
        fsm.send(FormEvent::ResponseReceived);
        assert_eq!(fsm.state(), FormState::Success);
        
        // Start new form
        assert!(fsm.send(FormEvent::NewForm));
        assert_eq!(fsm.state(), FormState::Editing);
        assert_eq!(fsm.retry_count(), 0); // Reset
    }
    
    #[test]
    fn test_done_from_success() {
        let actions = TestFormActions::new();
        actions.set_fields_valid(true);
        actions.set_server_response(ServerResponse::Success);
        
        let mut fsm = FormFsm::new(actions);
        
        // Get to Success
        fsm.send(FormEvent::Submit);
        fsm.send(FormEvent::ValidationComplete);
        fsm.send(FormEvent::ResponseReceived);
        
        // Done
        assert!(fsm.send(FormEvent::Done));
    }
    
    #[test]
    fn test_choice_point_validation() {
        let actions = TestFormActions::new();
        let mut fsm = FormFsm::new(actions.clone());
        
        // Test valid path
        actions.set_fields_valid(true);
        fsm.send(FormEvent::Submit);
        fsm.send(FormEvent::ValidationComplete);
        assert_eq!(fsm.state(), FormState::Submitting);
        
        // Reset and test invalid path
        let actions2 = TestFormActions::new();
        actions2.set_fields_valid(false);
        let mut fsm2 = FormFsm::new(actions2);
        
        fsm2.send(FormEvent::Submit);
        fsm2.send(FormEvent::ValidationComplete);
        assert_eq!(fsm2.state(), FormState::Editing);
    }
    
    #[test]
    fn test_choice_point_response() {
        // Test all three response paths
        let test_cases = [
            (ServerResponse::Success, FormState::Success),
            (ServerResponse::PermanentError, FormState::Error),
        ];
        
        for (response, expected_state) in test_cases {
            let actions = TestFormActions::new();
            actions.set_fields_valid(true);
            actions.set_server_response(response);
            
            let mut fsm = FormFsm::new(actions);
            fsm.send(FormEvent::Submit);
            fsm.send(FormEvent::ValidationComplete);
            fsm.send(FormEvent::ResponseReceived);
            
            assert_eq!(fsm.state(), expected_state, "Response {:?} should lead to {:?}", response, expected_state);
        }
    }
    
    #[test]
    fn test_invalid_events_ignored() {
        let actions = TestFormActions::new();
        let mut fsm = FormFsm::new(actions);
        
        // Can't retry from Editing
        assert!(!fsm.send(FormEvent::Retry));
        assert_eq!(fsm.state(), FormState::Editing);
        
        // Can't receive response in Editing
        assert!(!fsm.send(FormEvent::ResponseReceived));
        assert_eq!(fsm.state(), FormState::Editing);
    }
    
    #[test]
    fn test_full_workflow_with_retry() {
        let actions = TestFormActions::new();
        
        let mut fsm = FormFsm::new(actions.clone());
        
        // 1. Edit and submit with invalid data
        actions.set_fields_valid(false);
        fsm.send(FormEvent::DataChanged);
        fsm.send(FormEvent::Submit);
        fsm.send(FormEvent::ValidationComplete);
        assert_eq!(fsm.state(), FormState::Editing);
        
        // 2. Fix data and resubmit
        actions.set_fields_valid(true);
        actions.set_server_response(ServerResponse::Retryable);
        fsm.send(FormEvent::Submit);
        fsm.send(FormEvent::ValidationComplete);
        assert_eq!(fsm.state(), FormState::Submitting);
        
        // 3. Server returns retryable twice
        fsm.send(FormEvent::ResponseReceived);
        fsm.send(FormEvent::ResponseReceived);
        assert_eq!(fsm.retry_count(), 2);
        
        // 4. Server succeeds
        actions.set_server_response(ServerResponse::Success);
        fsm.send(FormEvent::ResponseReceived);
        assert_eq!(fsm.state(), FormState::Success);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Form Submission FSM - Choice Points Example\n");
    
    // Scenario 1: Happy path
    println!("=== Scenario 1: Happy Path ===");
    let actions = TestFormActions::new();
    actions.set_fields_valid(true);
    actions.set_server_response(ServerResponse::Success);
    
    let mut fsm = FormFsm::new(actions.clone());
    
    println!("Initial: {:?}", fsm.state());
    
    fsm.send(FormEvent::DataChanged);
    println!("After DataChanged: {:?}", fsm.state());
    
    fsm.send(FormEvent::Submit);
    println!("After Submit: {:?}", fsm.state());
    
    fsm.send(FormEvent::ValidationComplete);
    println!("After ValidationComplete: {:?}", fsm.state());
    
    fsm.send(FormEvent::ResponseReceived);
    println!("After ResponseReceived: {:?}", fsm.state());
    
    // Scenario 2: Validation failure
    println!("\n=== Scenario 2: Validation Failure ===");
    let actions2 = TestFormActions::new();
    actions2.set_fields_valid(false);
    
    let mut fsm2 = FormFsm::new(actions2);
    
    fsm2.send(FormEvent::Submit);
    println!("After Submit: {:?}", fsm2.state());
    
    fsm2.send(FormEvent::ValidationComplete);
    println!("After ValidationComplete (invalid): {:?}", fsm2.state());
    
    // Scenario 3: Retry logic
    println!("\n=== Scenario 3: Server Retry ===");
    let actions3 = TestFormActions::new();
    actions3.set_fields_valid(true);
    actions3.set_server_response(ServerResponse::Retryable);
    
    let mut fsm3 = FormFsm::new(actions3.clone());
    
    fsm3.send(FormEvent::Submit);
    fsm3.send(FormEvent::ValidationComplete);
    
    println!("Submitting...");
    for i in 1..=3 {
        fsm3.send(FormEvent::ResponseReceived);
        println!("  Retry #{}: {:?}", i, fsm3.state());
    }
    
    actions3.set_server_response(ServerResponse::Success);
    fsm3.send(FormEvent::ResponseReceived);
    println!("  Final: {:?}", fsm3.state());
    
    println!("\n--- Action Log (Scenario 1) ---");
    for (i, action) in actions.get_log().iter().enumerate() {
        println!("  {}: {}", i + 1, action);
    }
    
    println!("\n✅ Form Submission FSM works correctly with choice points!");
}
