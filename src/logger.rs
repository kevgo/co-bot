pub struct Logger {
    pub verbose: bool,
}

impl Logger {
    pub fn log(&self, args: std::fmt::Arguments) {
        if self.verbose {
            println!("{}", args);
        }
    }
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log(format_args!($($arg)*))
    };
}
