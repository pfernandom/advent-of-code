#![allow(dead_code)]
use std::{collections::HashSet, fmt::Debug};

pub fn assert_contains_all<T: Eq + std::hash::Hash + Debug>(actual: Vec<T>, expected: Vec<T>) {
    let mut exp = HashSet::new();
    for e in expected {
        exp.insert(e);
    }

    for a in actual {
        exp.remove(&a);
    }

    assert!(
        exp.is_empty(),
        "The following elements were not found: {:?}",
        exp
    );
}
