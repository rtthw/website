//! Environment



#[derive(Clone)]
pub struct Environment(std::sync::Arc<parking_lot::RwLock<EnvironmentImpl>>);

impl Default for Environment {
    fn default() -> Self {
        Self(std::sync::Arc::new(parking_lot::RwLock::new(EnvironmentImpl {
            manifests: vec![
                calculator_manifest(),
                manual_manifest(),
            ],
        })))
    }
}

impl Environment {
    fn read<R>(&self, reader: impl FnOnce(&EnvironmentImpl) -> R) -> R {
        reader(&self.0.read())
    }

    fn write<R>(&self, writer: impl FnOnce(&mut EnvironmentImpl) -> R) -> R {
        writer(&mut self.0.write())
    }
}

struct EnvironmentImpl {
    manifests: Vec<Manifest>,
}



pub struct Manifest {
    name: &'static str, // All manifests are hard-coded.
}

fn calculator_manifest() -> Manifest {
    Manifest { name: "Calculator" }
}

fn manual_manifest() -> Manifest {
    Manifest { name: "Manual" }
}
