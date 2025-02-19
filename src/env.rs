//! Environment



#[derive(Clone)]
pub struct Environment(std::sync::Arc<parking_lot::RwLock<EnvironmentImpl>>);

impl Default for Environment {
    fn default() -> Self {
        Self(std::sync::Arc::new(parking_lot::RwLock::new(EnvironmentImpl {
            packages: Packages::default(),
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

impl Environment {
    pub fn packages<R>(&self, reader: impl FnOnce(&Packages) -> R) -> R {
        self.read(move |env| reader(&env.packages))
    }

    pub fn packages_mut<R>(&self, reader: impl FnOnce(&mut Packages) -> R) -> R {
        self.write(move |env| reader(&mut env.packages))
    }
}

struct EnvironmentImpl {
    packages: Packages,
}



pub struct Packages {
    /// Manifests loaded into memory.
    manifests: Vec<Manifest>,
    /// Package objects loaded into memory.
    running: Vec<Box<dyn Package>>,
}

impl Default for Packages {
    fn default() -> Self {
        Self {
            manifests: vec![
                calculator_manifest(),
                manual_manifest(),
            ],
            running: vec![],
        }
    }
}

impl Packages {
    pub fn manifests(&self) -> &Vec<Manifest> {
        &self.manifests
    }

    pub fn exec(&mut self, pkg_name: &str) -> bool {
        if let Some(manifest) = self.manifests.iter().find(|m| m.name == pkg_name) {
            self.running.push((manifest.exec_fn)());
            return true;
        }

        false
    }
}



/// A package manifest.
#[derive(Clone)]
pub struct Manifest {
    name: &'static str, // All manifests are hard-coded.
    exec_fn: fn() -> Box<dyn Package>,
}

impl Manifest {
    pub fn title(&self) -> &'static str {
        self.name
    }
}

struct Calculator;

impl Package for Calculator {}

fn calculator_manifest() -> Manifest {
    Manifest { name: "Calculator", exec_fn: exec_calculator }
}

fn exec_calculator() -> Box<dyn Package> {
    println!("Starting calculator...");
    Box::new(Calculator)
}

struct Manual;

impl Package for Manual {}

fn manual_manifest() -> Manifest {
    Manifest { name: "Manual", exec_fn: exec_manual }
}

fn exec_manual() -> Box<dyn Package> {
    println!("Starting manual...");
    Box::new(Manual)
}



/// A package object.
pub trait Package {}
