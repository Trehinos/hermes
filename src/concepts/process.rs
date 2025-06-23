//! Abstractions for running a small application lifecycle.
//!
//! The [`Process`] trait models a task that can be initialised, executed and
//! finalised. [`Application`] is a simple wrapper holding configuration and a
//! boxed kernel implementing [`Process`].

/// Basic process executed by the [`Application`] runtime.
///
/// The trait defines three lifecycle hooks:
/// - [`Process::initialize`] prepares resources.
/// - [`Process::execute`] performs the main logic.
/// - [`Process::finalize`] cleans up after execution.
///
/// The [`Process::run`] helper executes these steps in order.
///
/// # Example
/// ```
/// use hermes::concepts::process::Process;
///
/// struct Task;
/// impl Process<()> for Task {
///     fn execute(&mut self) -> Result<(), String> { Ok(()) }
/// }
///
/// let mut task = Task;
/// assert!(task.run().is_ok());
/// ```
pub trait Process<R = (), E = String> {
    /// Perform the main logic and return a result.
    fn execute(&mut self) -> Result<R, E>;
    /// Prepare the process before [`execute`](Process::execute) runs.
    fn initialize(&mut self) -> Result<(), E> {
        Ok(())
    }
    /// Called after [`execute`](Process::execute) completes to release resources.
    fn finalize(&mut self) -> Result<(), E> {
        Ok(())
    }

    /// Execute `initialize`, `execute` and `finalize` in sequence.
    fn run(&mut self) -> Result<R, E> {
        self.initialize()?;
        let result = self.execute()?;
        self.finalize()?;
        Ok(result)
    }
}

/// A boxed [`Process`] used as the entry point of an application.
pub type Kernel<E> = dyn Process<(), E>;

/// Minimal wrapper executing a [`Kernel`] with some configuration.
///
/// # Example
/// ```
/// use hermes::concepts::process::{Application, Process};
///
/// struct Hello;
/// impl Process<()> for Hello {
///     fn execute(&mut self) -> Result<(), String> { Ok(()) }
/// }
///
/// let mut app = Application::new("config", Hello);
/// assert!(app.run().is_ok());
/// ```
pub struct Application<C, E> {
    pub config: C,
    kernel: Box<Kernel<E>>,
}

impl<C, E> Application<C, E> {
    /// Create a new application wrapping the given kernel and configuration.
    pub fn new<K>(config: C, kernel: K) -> Self
    where
        K: Process<(), E> + 'static,
    {
        Self {
            config,
            kernel: Box::new(kernel),
        }
    }
}

impl<C, E> Process<(), E> for Application<C, E> {
    fn execute(&mut self) -> Result<(), E> {
        self.kernel.execute()
    }
}
