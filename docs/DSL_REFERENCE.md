# Oxidate DSL Reference

Complete syntax reference for the Oxidate FSM Domain-Specific Language.

---

## Table of Contents

1. [File Structure](#file-structure)
2. [States](#states)
3. [Transitions](#transitions)
4. [Events](#events)
5. [Guards](#guards)
6. [Actions](#actions)
7. [Timers](#timers)
8. [Choice Points](#choice-points)
9. [Comments](#comments)
10. [Complete Examples](#complete-examples)

---

## File Structure

An Oxidate file contains one or more FSM definitions:

```
fsm MachineName {
    // FSM content here
}

fsm AnotherMachine {
    // Another FSM
}
```

Each FSM is independent and can reference its own states, transitions, timers, and choice points.

---

## States

### Simple State

```
state Idle
```

### State with Description

```
state Running: "The system is actively processing"
```

The description appears as a tooltip in the GUI and as documentation in generated code.

### State with Body

```
state Active {
    entry / on_enter()
    exit / on_exit()
}
```

### State Body Items

| Syntax | Description |
|--------|-------------|
| `entry / action()` | Execute action when entering state |
| `exit / action()` | Execute action when leaving state |
| `start_timer(name)` | Start a named timer |
| `stop_timer(name)` | Stop a named timer |
| `Event / action()` | Internal transition (handle event without leaving) |

### Entry and Exit Actions

```
state Connecting {
    entry / start_connection()
    exit / cleanup_connection()
}
```

### Internal Transitions

Handle an event while staying in the same state:

```
state Monitoring {
    entry / begin_monitoring()
    DataReceived / process_data()
    Heartbeat / reset_timeout()
}
```

### Timer Control in States

```
state WaitingForResponse {
    entry / start_timer(response_timeout)
    exit / stop_timer(response_timeout)
}
```

---

## Transitions

### Basic Syntax

```
SourceState --> TargetState
```

or with the shorter arrow:

```
SourceState -> TargetState
```

### Initial State

The initial state uses `[*]` as the source:

```
[*] --> Idle
```

### Final State

Use `[*]` as the target for final states:

```
Completed --> [*]
```

### Transition with Event

```
Idle --> Running : StartButton
```

### Transition with Guard

```
Ready --> Processing : Submit [is_valid]
```

### Transition with Action

```
Idle --> Active : Trigger / initialize()
```

### Full Transition Syntax

```
Source --> Target : Event [guard_condition] / action()
```

All components except Source and Target are optional:

```
// Just states
A --> B

// With event
A --> B : click

// With event and guard
A --> B : click [enabled]

// With event and action
A --> B : click / handle_click()

// Full form
A --> B : click [enabled] / handle_click()
```

### Self-Transitions

A state can transition to itself:

```
Polling --> Polling : tick / check_status()
```

---

## Events

Events are identifiers that trigger transitions:

```
Idle --> Running : StartPressed
Running --> Idle : StopPressed
Running --> Error : FaultDetected
```

### Event Naming Conventions

- Use `PascalCase` or `snake_case`
- Be descriptive: `ButtonPressed`, `TimerExpired`, `DataReceived`
- Events are shared across the FSM (same name = same event)

---

## Guards

Guards are boolean conditions that must be true for a transition to occur:

```
Idle --> Processing : Submit [form_valid]
Idle --> Error : Submit [!form_valid]
```

### Guard Syntax

```
[expression]
```

The expression inside brackets is passed directly to generated code. Examples:

```
[is_ready]
[count > 0]
[buffer.is_empty()]
[self.temperature < MAX_TEMP]
```

### Multiple Guards

Use separate transitions with different guards:

```
Checking --> Approved : Evaluate [score >= 70]
Checking --> Rejected : Evaluate [score < 70]
```

---

## Actions

Actions are function calls executed during transitions or state entry/exit:

### Transition Actions

```
Idle --> Active : Start / begin_processing()
```

### Actions with Parameters

```
state Logging {
    entry / log_message("Entered logging state")
}

Error --> Recovery : Reset / reset_with_code(0)
```

### Action Naming

- Use `snake_case` for action names
- Actions map to trait methods in generated code

---

## Timers

Timers trigger events after a specified duration.

### Timer Definition

```
timer timer_name = duration_ms -> EventName
timer timer_name = duration_ms -> EventName periodic
```

### Examples

```
// One-shot timer: fires once after 5 seconds
timer timeout = 5000 -> Timeout

// Periodic timer: fires every 500ms
timer heartbeat = 500 -> Tick periodic
```

### Timer Control

Control timers from within states:

```
state Waiting {
    entry / start_timer(timeout)
    exit / stop_timer(timeout)
}

// Or inline in a generic action context:
// start_timer(timer_name)
// stop_timer(timer_name)
```

### Timer-Triggered Transitions

```
timer blink = 500 -> BlinkTick periodic

state LedOn {
    entry / led_on()
    start_timer(blink)
}

state LedOff {
    entry / led_off()
}

LedOn --> LedOff : BlinkTick
LedOff --> LedOn : BlinkTick
```

---

## Choice Points

Choice points (decision nodes) route transitions based on conditions.

### Definition

```
choice ChoiceName {
    [condition1] -> TargetState1
    [condition2] -> TargetState2 / action()
    [else] -> DefaultState
}
```

### Usage

Reference a choice point with `<<name>>`:

```
Processing --> <<ValidateResult>> : Complete

choice ValidateResult {
    [result.is_ok()] -> Success / log_success()
    [result.is_warning()] -> PartialSuccess
    [else] -> Failure / log_error()
}
```

### Choice Point Rules

1. Conditions are evaluated in order
2. First matching condition wins
3. `[else]` catches all remaining cases
4. Each branch can have an optional action

---

## Comments

### Single-Line Comments

```
// This is a comment
state Idle  // Inline comment
```

### Multi-Line Comments

```
/* 
   This is a
   multi-line comment
*/
```

---

## Complete Examples

### Traffic Light

```
fsm TrafficLight {
    // Timers
    timer red_timer = 5000 -> RedExpired
    timer yellow_timer = 2000 -> YellowExpired
    timer green_timer = 4000 -> GreenExpired

    // Initial state
    [*] --> Red

    // States
    state Red: "Stop" {
        entry / display_red()
        entry / start_timer(red_timer)
    }

    state Yellow: "Caution" {
        entry / display_yellow()
        entry / start_timer(yellow_timer)
    }

    state Green: "Go" {
        entry / display_green()
        entry / start_timer(green_timer)
    }

    // Transitions
    Red --> Green : RedExpired
    Green --> Yellow : GreenExpired
    Yellow --> Red : YellowExpired
}
```

### Door Lock System

```
fsm DoorLock {
    timer auto_lock = 30000 -> AutoLock

    [*] --> Locked

    state Locked: "Door is secured" {
        entry / engage_lock()
        entry / arm_alarm()
    }

    state Unlocked: "Door can be opened" {
        entry / disengage_lock()
        entry / start_timer(auto_lock)
        exit / stop_timer(auto_lock)
    }

    state Alarming: "Intrusion detected" {
        entry / sound_alarm()
        entry / notify_security()
    }

    // Normal operation
    Locked --> Unlocked : ValidCode
    Unlocked --> Locked : LockButton
    Unlocked --> Locked : AutoLock

    // Security
    Locked --> Alarming : TamperDetected
    Locked --> Alarming : InvalidCode [attempts > 3]
    Alarming --> Locked : AlarmReset [authorized]
}
```

### Connection State Machine

```
fsm ConnectionManager {
    timer connect_timeout = 10000 -> ConnectTimeout
    timer keepalive = 30000 -> KeepaliveTick periodic

    [*] --> Disconnected

    state Disconnected {
        entry / reset_connection()
    }

    state Connecting {
        entry / initiate_connection()
        entry / start_timer(connect_timeout)
        exit / stop_timer(connect_timeout)
    }

    state Connected {
        entry / start_timer(keepalive)
        exit / stop_timer(keepalive)
        KeepaliveTick / send_keepalive()
    }

    state Reconnecting {
        entry / schedule_reconnect()
    }

    // Happy path
    Disconnected --> Connecting : Connect
    Connecting --> Connected : ConnectionEstablished

    // Error handling
    Connecting --> Disconnected : ConnectTimeout
    Connecting --> Disconnected : ConnectionFailed
    Connected --> Reconnecting : ConnectionLost
    Reconnecting --> Connecting : ReconnectTimer

    // Manual disconnect
    Connected --> Disconnected : Disconnect
    Connecting --> Disconnected : Cancel
}
```

### Form Validation with Choice Points

```
fsm FormSubmission {
    [*] --> Editing

    state Editing {
        DataChanged / validate_field()
    }

    state Validating {
        entry / run_validation()
    }

    state Submitting {
        entry / send_to_server()
    }

    state Success {
        entry / show_success_message()
    }

    state Error {
        entry / show_error_message()
    }

    choice ValidationResult {
        [all_fields_valid] -> Submitting
        [else] -> Editing / highlight_errors()
    }

    choice SubmitResult {
        [response.is_success()] -> Success
        [response.is_retryable()] -> Submitting / increment_retry()
        [else] -> Error
    }

    Editing --> Validating : Submit
    Validating --> <<ValidationResult>> : ValidationComplete
    Submitting --> <<SubmitResult>> : ResponseReceived
    Error --> Editing : Retry
    Success --> [*]
}
```

---

## Grammar Reference

The complete grammar is defined in `src/parser/fsm.pest`. Key rules:

| Rule | Description |
|------|-------------|
| `file` | Root: zero or more FSM definitions |
| `fsm_definition` | `fsm Name { body }` |
| `state_simple` | `state Name` or `state Name: "desc"` |
| `state_with_body` | `state Name { items }` |
| `transition` | `Source --> Target : label` |
| `timer_def` | `timer name = ms -> Event [mode]` |
| `choice_def` | `choice Name { branches }` |
| `identifier` | `[a-zA-Z_][a-zA-Z0-9_]*` |

---

## Tips & Best Practices

1. **Name states clearly** — Use nouns or adjectives: `Idle`, `Running`, `Connected`
2. **Name events as past tense or signals** — `ButtonPressed`, `DataReceived`, `Timeout`
3. **Keep guards simple** — Complex logic belongs in action code
4. **Use choice points** for multi-way branches instead of multiple guarded transitions
5. **Document with descriptions** — They become code comments and GUI tooltips
6. **Validate early** — The GUI shows parse errors in real-time
