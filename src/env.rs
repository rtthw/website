//! Environment



use std::sync::{Arc, Mutex};



#[derive(Clone)]
pub struct Environment(Arc<Mutex<EnvironmentImpl>>);

impl Default for Environment {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(EnvironmentImpl {  })))
    }
}

struct EnvironmentImpl {}
