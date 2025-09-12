pub(crate) use crate::*;

pub struct TestContext {
    pub(crate) start: std::time::Instant,
    pub(crate) mode: RunMode,
    pub(crate) run_ignored: bool,
}

impl TestContext {
    pub fn ignore(&self) -> Result<(), RunError> {
        if self.run_ignored {
            Ok(())
        } else {
            Err(RunError::ignore())
        }
    }

    pub fn ignore_for(&self, reason: impl std::fmt::Display) -> Result<(), RunError> {
        if self.run_ignored {
            Ok(())
        } else {
            Err(RunError::ignore_for(reason.to_string()))
        }
    }

    pub fn current_mode(&self) -> RunMode {
        self.mode
    }

    pub fn elapased_s(&self) -> notify::Elapsed {
        notify::Elapsed(self.start.elapsed())
    }
}
