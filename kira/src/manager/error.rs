//! Things that can go wrong when using an [`AudioManager`](super::AudioManager).

use cpal::{BuildStreamError, DefaultStreamConfigError, PlayStreamError};
use thiserror::Error;

/// Things that can go wrong when creating an `AudioManager`.
#[derive(Debug, Error)]
pub enum SetupError {
	/// A default audio output device could not be determined.
	#[error("Cannot find the default audio output device")]
	NoDefaultOutputDevice,

	/// An error occurred when getting the default output configuration.
	#[error("{0}")]
	DefaultStreamConfigError(#[from] DefaultStreamConfigError),

	/// An error occured when building the audio stream.
	#[error("{0}")]
	BuildStreamError(#[from] BuildStreamError),

	/// An error occured when starting the audio stream.
	#[error("{0}")]
	PlayStreamError(#[from] PlayStreamError),
}

#[derive(Debug, Error)]
pub enum AddSoundError {
	#[error("Could not add a sound because the maximum number of sounds has been reached.")]
	SoundLimitReached,
	#[error("Could not add a sound because the command queue is full.")]
	CommandQueueFull,
}
