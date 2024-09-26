use logic::expressions::proposition::Proposition;
use logic::operators::{Always, And, Eventually, Implies, Not};
use logic::Formula;
use logic::Trace;

use std::collections::HashMap;
 
fn main() {
    // Realizable Example
    // Requirement: Always, if the battery is low, then eventually the spacecraft will recharge.
    let formula_realizable = Always::new(Implies::new(
        Proposition::new("battery_low"),
        Eventually::new(Proposition::new("recharge")),
    ));

    // Create a trace that satisfies the realizable requirement
    let trace_realizable = Trace::from_iter([
        (
            0.0,
            HashMap::from([
                ("battery_low".to_string(), false),
                ("recharge".to_string(), false),
            ]),
        ),
        (
            1.0,
            HashMap::from([
                ("battery_low".to_string(), true),
                ("recharge".to_string(), false),
            ]),
        ),
        (
            2.0,
            HashMap::from([
                ("battery_low".to_string(), true),
                ("recharge".to_string(), false),
            ]),
        ),
        (
            3.0,
            HashMap::from([
                ("battery_low".to_string(), true),
                ("recharge".to_string(), true), // Recharging starts here
            ]),
        ),
        (
            4.0,
            HashMap::from([
                ("battery_low".to_string(), false),
                ("recharge".to_string(), true),
            ]),
        ),
    ]);

    // Evaluate the formula over the trace
    let satisfied_realizable = formula_realizable.satisfied_by(&trace_realizable).unwrap();
    println!("Realizable formula is satisfied: {}", satisfied_realizable);

    // Non-Realizable Example
    // Requirement 1: Always, if the battery is low, then eventually the spacecraft will recharge.
    // Requirement 2: Always, the spacecraft cannot recharge.
    let formula_non_realizable = And::new(
        Always::new(Implies::new(
            Proposition::new("battery_low"),
            Eventually::new(Proposition::new("recharge")),
        )),
        Always::new(Not::new(Proposition::new("recharge"))),
    );

    // Create a trace attempting to satisfy the non-realizable requirements
    let trace_non_realizable = Trace::from_iter([
        (
            0.0,
            HashMap::from([
                ("battery_low".to_string(), false),
                ("recharge".to_string(), false),
            ]),
        ),
        (
            1.0,
            HashMap::from([
                ("battery_low".to_string(), true),
                ("recharge".to_string(), false),
            ]),
        ),
        (
            2.0,
            HashMap::from([
                ("battery_low".to_string(), true),
                ("recharge".to_string(), false),
            ]),
        ),
        (
            3.0,
            HashMap::from([
                ("battery_low".to_string(), true),
                ("recharge".to_string(), false), // Cannot recharge
            ]),
        ),
        (
            4.0,
            HashMap::from([
                ("battery_low".to_string(), false),
                ("recharge".to_string(), false),
            ]),
        ),
    ]);

    // Evaluate the non-realizable formula over the trace
    let satisfied_non_realizable = formula_non_realizable
        .satisfied_by(&trace_non_realizable)
        .unwrap();
    println!(
        "Non-realizable formula is satisfied: {}",
        satisfied_non_realizable
    );
}
