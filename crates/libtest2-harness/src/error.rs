pub(crate) use crate::*;

pub type RunResult = Result<(), RunError>;

#[derive(Debug)]
pub struct RunError {
    status: notify::MessageKind,
    cause: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl RunError {
    pub fn with_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            status: notify::MessageKind::Error,
            cause: Some(Box::new(cause)),
        }
    }

    pub fn fail(cause: impl std::fmt::Display) -> Self {
        Self::with_cause(Message(cause.to_string()))
    }

    /// Should not be called with `libtest_lexarg::RunIgnored::Yes`
    pub fn ignore() -> Self {
        Self {
            status: notify::MessageKind::Ignored,
            cause: None,
        }
    }

    /// Should not be called with `libtest_lexarg::RunIgnored::Yes`
    pub fn ignore_for(reason: String) -> Self {
        Self {
            status: notify::MessageKind::Ignored,
            cause: Some(Box::new(Message(reason))),
        }
    }

    pub(crate) fn status(&self) -> notify::MessageKind {
        self.status
    }

    pub(crate) fn cause(&self) -> Option<&(dyn std::error::Error + Send + Sync)> {
        self.cause.as_ref().map(|b| b.as_ref())
    }
}

impl<E> From<E> for RunError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        Self::with_cause(error)
    }
}

#[derive(Debug)]
struct Message(String);

impl std::fmt::Display for Message {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for Message {}

pub trait IntoRunResult {
    fn into_run_result(self) -> RunResult;
}

impl IntoRunResult for () {
    fn into_run_result(self) -> RunResult {
        Ok(())
    }
}

impl IntoRunResult for RunResult {
    fn into_run_result(self) -> RunResult {
        self
    }
}
