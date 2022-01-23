pub struct ThreadPuddle;

impl ThreadPuddle {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
