use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::{time::Duration, panic};

pub trait Indicator {
    fn start_pb(&self, _size: u64, _prefix: &str) {}

    fn inc_pb(&self, _i: u64) {}

    fn finish_pb(&self, _name: &str, _duration: Duration) {}

    fn debug_msg(&self, _msg: &str) {}
}

pub struct SilentIndicator;

impl Indicator for SilentIndicator {}

pub struct ConsoleIndicator {
    pb: ProgressBar,
}

impl ConsoleIndicator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ConsoleIndicator {
    fn default() -> Self {
        let pb = ProgressBar::new(0);
        Self { pb }
    }
}

impl Indicator for ConsoleIndicator {
    fn start_pb(&self, size: u64, name: &str) {
        let result = panic::catch_unwind(|| {
            self.pb.set_length(size);
            self.pb.set_prefix(name.to_owned());
            let template =
                ProgressStyle::default_bar()
                    .template(
                        "[Dumping: {prefix}] [|{bar:50}|] {pos} of {len} rows [{percent}%] ({eta})",
                    );
            match template {
                Ok(t) => {
                    self.pb.set_style(
                        t.progress_chars("#>-"),
                    );
                }
                Err(e) => {
                    self.debug_msg(&format!("{}", e));
                }
            }
        });
        match result {
            Ok(_) => {},
            Err(e) => {
                self.debug_msg(&format!("inc_pb panic caught: {:?}", e))
            },
        }
    }

    fn inc_pb(&self, i: u64) {
        let result = panic::catch_unwind(||{
            self.pb.inc(i);
        });
        match result {
            Ok(_) => {},
            Err(e) => {
                self.debug_msg(&format!("inc_pb panic caught: {:?}", e))
            },
        }
    }

    fn finish_pb(&self, name: &str, duration: Duration) {
        let result = panic::catch_unwind(||{
            self.pb.finish();
            self.pb.reset();
        });
        match result {
            Ok(_) => {},
            Err(e) => {
                self.debug_msg(&format!("pb finish/reset panic caught: {:?}", e))
            },
        }

        self.debug_msg(
            format!(
                "[Dumping: {}] Finished in {}",
                name, HumanDuration(duration))
            .as_str(),
        );
    }

    fn debug_msg(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // just test that there is no panic
    mod console_indicator {
        use super::*;

        #[test]
        fn debug_msg() {
            ConsoleIndicator::new().debug_msg("some message");
        }

        #[test]
        fn pb_start_finish() {
            let ci = ConsoleIndicator::new();
            ci.start_pb(100, "name");
            ci.finish_pb("name", Duration::new(1, 0));
        }

        #[test]
        fn pb_some_progress() {
            let ci = ConsoleIndicator::new();
            ci.start_pb(100, "name");
            ci.inc_pb(1);
            ci.inc_pb(10);
            ci.finish_pb("name", Duration::new(1, 0));
        }

        #[test]
        fn pb_overflow_progress() {
            let ci = ConsoleIndicator::new();
            ci.start_pb(100, "name");
            ci.inc_pb(1);
            ci.inc_pb(10);
            ci.inc_pb(100);
            ci.finish_pb("name", Duration::new(1, 0));
        }
    }
}
