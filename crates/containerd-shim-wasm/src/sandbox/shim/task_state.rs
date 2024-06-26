use tracing::{instrument, Span};

use crate::sandbox::Error::FailedPrecondition;
use crate::sandbox::Result;

#[derive(Debug, Clone, Copy)]
pub(super) enum TaskState {
    Created,
    Starting,
    Started,
    Exited,
    Deleting,
}

impl TaskState {
    #[instrument(skip_all, parent = Span::current(), level= "Info")]
    pub fn start(&mut self) -> Result<()> {
        *self = match self {
            Self::Created => Ok(Self::Starting),
            _ => state_transition_error(*self, Self::Starting),
        }?;
        Ok(())
    }

    #[instrument(skip_all, parent = Span::current(), level= "Info")]
    pub fn kill(&mut self) -> Result<()> {
        *self = match self {
            Self::Started => Ok(Self::Started),
            _ => state_transition_error(*self, "Killing"),
        }?;
        Ok(())
    }

    #[instrument(skip_all, parent = Span::current(), level= "Info")]
    pub fn delete(&mut self) -> Result<()> {
        *self = match self {
            Self::Created | Self::Exited => Ok(Self::Deleting),
            _ => state_transition_error(*self, Self::Deleting),
        }?;
        Ok(())
    }

    #[instrument(skip_all, parent = Span::current(), level= "Info")]
    pub fn started(&mut self) -> Result<()> {
        *self = match self {
            Self::Starting => Ok(Self::Started),
            _ => state_transition_error(*self, Self::Started),
        }?;
        Ok(())
    }

    #[instrument(skip_all, parent = Span::current(), level= "Info")]
    pub fn stop(&mut self) -> Result<()> {
        *self = match self {
            Self::Started | Self::Starting => Ok(Self::Exited),
            // This is for potential failure cases where we want delete to be able to be retried.
            Self::Deleting => Ok(Self::Exited),
            _ => state_transition_error(*self, Self::Exited),
        }?;
        Ok(())
    }
}

fn state_transition_error<T>(from: impl std::fmt::Debug, to: impl std::fmt::Debug) -> Result<T> {
    Err(FailedPrecondition(format!(
        "invalid state transition: {from:?} => {to:?}"
    )))
}
