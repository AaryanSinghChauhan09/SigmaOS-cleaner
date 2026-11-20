//! Integration tests for SigmaOS-cleaner.

use sigmaos_cleaner_lib::{Component, ComponentLifecycle};

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
