//! Environment



#[derive(Clone)]
pub struct Environment(std::sync::Arc<parking_lot::RwLock<EnvironmentImpl>>);

impl Default for Environment {
    fn default() -> Self {
        Self(std::sync::Arc::new(parking_lot::RwLock::new(EnvironmentImpl {
            window_manager: WindowManager,
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

// Host-facing.
impl Environment {
    pub fn windows<R>(&self, reader: impl FnOnce(&WindowManager) -> R) -> R {
        self.read(move |env| reader(&env.window_manager))
    }

    pub fn windows_mut<R>(&self, reader: impl FnOnce(&mut WindowManager) -> R) -> R {
        self.write(move |env| reader(&mut env.window_manager))
    }
}

// Package-facing.
impl Environment {
    pub fn create_window(&self, title: &str, ty: WindowType) -> Option<u32> {
        None
    }
}

struct EnvironmentImpl {
    window_manager: WindowManager,
}



pub struct WindowManager;

#[derive(Default)]
pub enum WindowType {
    #[default]
    Normal,
    Popup,
}



pub struct Packages {
    /// Manifests loaded into memory.
    manifests: Vec<Manifest>,
    /// Package objects loaded into memory.
    running: Vec<PackageRuntime>,
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

    pub fn exec(&mut self, env: &mut Environment, pkg_name: &str) -> bool {
        if let Some(manifest) = self.manifests.iter().find(|m| m.name == pkg_name) {
            self.running.push(PackageRuntime {
                pkg: (manifest.exec_fn)(env),
            });
            return true;
        }

        false
    }
}

struct PackageRuntime {
    pkg: Box<dyn Package>,
}



/// A package manifest.
#[derive(Clone)]
pub struct Manifest {
    name: &'static str, // All manifests are hard-coded.
    exec_fn: fn(&mut Environment) -> Box<dyn Package>,
}

impl Manifest {
    pub fn title(&self) -> &'static str {
        self.name
    }
}



/// A package object.
pub trait Package {}



struct Calculator {
    window: u32,
}

impl Package for Calculator {}

fn calculator_manifest() -> Manifest {
    Manifest { name: "Calculator", exec_fn: exec_calculator }
}

fn exec_calculator(env: &mut Environment) -> Box<dyn Package> {
    println!("Starting calculator...");
    let window = env.create_window("Calculator", WindowType::Normal).unwrap();
    Box::new(Calculator { window })
}

struct Manual {
    window: u32,
}

impl Package for Manual {}

fn manual_manifest() -> Manifest {
    Manifest { name: "Manual", exec_fn: exec_manual }
}

fn exec_manual(env: &mut Environment) -> Box<dyn Package> {
    println!("Starting manual...");
    let window = env.create_window("Manual", WindowType::Normal).unwrap();
    Box::new(Manual { window })
}
