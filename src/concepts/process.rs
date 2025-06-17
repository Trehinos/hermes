
pub trait Process<R = (), E = String> {
    fn execute(&mut self) -> Result<R, E>;
    fn initialize(&mut self) -> Result<(), E> {
        Ok(())
    }
    fn finalize(&mut self) -> Result<(), E> {
        Ok(())
    }

    fn run(&mut self) -> Result<R, E> {
        self.initialize()?;
        let result = self.execute()?;
        self.finalize()?;
        Ok(result)
    }
}

pub type Kernel<E> = dyn Process<(), E>;

pub struct Application<C, E> {
    pub config: C,
    kernel: Box<Kernel<E>>,
}

impl<C, E> Application<C, E> {
    pub fn new(config: C, kernel: &Kernel<E>) -> Self
    where
        Kernel<E>: Clone,
    {
        Self {
            config,
            kernel: Box::new(kernel.clone()),
        }
    }
}

impl<C, E> Process<(), E> for Application<C, E> {
    fn execute(&mut self) -> Result<(), E> {
        self.kernel.execute()
    }
}
