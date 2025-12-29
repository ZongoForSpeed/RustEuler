pub struct ScopeTimer<'a> {
    pub label: &'a str,
    pub now: std::time::Instant,
    pub debug_only: bool,
}

impl<'a> ScopeTimer<'a> {
    pub fn new(label: &'a str, debug_only: bool) -> Self {
        match debug_only {
            false => println!("Starting: '{}' ...", label),
            true if cfg!(debug_assertions) => println!("Starting: '{}' ...", label),
            _ => (),
        }
        Self {
            label,
            now: std::time::Instant::now(),
            debug_only,
        }
    }

    pub fn elapsed_time(&self) {
        match self.debug_only {
            false => self.print_timer_info(),
            true if cfg!(debug_assertions) => self.print_timer_info(),
            _ => (),
        }
    }

    fn print_timer_info(&self) {
        println!(
            "End '{}' in {} seconds",
            self.label,
            self.now.elapsed().as_secs_f32()
        )
    }
}

impl<'a> Default for ScopeTimer<'a> {
    fn default() -> Self {
        Self {
            label: "timer",
            now: std::time::Instant::now(),
            debug_only: false,
        }
    }
}

impl<'a> Drop for ScopeTimer<'a> {
    fn drop(&mut self) {
        self.elapsed_time();
    }
}
