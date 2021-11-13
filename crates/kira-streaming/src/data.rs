use kira::sound::{Sound, SoundData};
use ringbuf::RingBuffer;

use crate::StreamingSoundHandle;

use super::{settings::StreamingSoundSettings, sound::StreamingSound, Decoder};

const COMMAND_BUFFER_CAPACITY: usize = 8;
const ERROR_BUFFER_CAPACITY: usize = 8;

pub struct StreamingSoundData<E: Send + Sync + 'static> {
	pub decoder: Box<dyn Decoder<Error = E>>,
	pub settings: StreamingSoundSettings,
}

impl<E: Send + Sync + 'static> StreamingSoundData<E> {
	pub fn new(
		decoder: impl Decoder<Error = E> + 'static,
		settings: StreamingSoundSettings,
	) -> Self {
		Self {
			decoder: Box::new(decoder),
			settings,
		}
	}
}

impl<E: Send + Sync + 'static> SoundData for StreamingSoundData<E> {
	type Error = E;

	type Handle = StreamingSoundHandle<E>;

	fn into_sound(self) -> Result<(Box<dyn Sound>, Self::Handle), Self::Error> {
		let (command_producer, command_consumer) = RingBuffer::new(COMMAND_BUFFER_CAPACITY).split();
		let (error_producer, error_consumer) = RingBuffer::new(ERROR_BUFFER_CAPACITY).split();
		let sound = StreamingSound::new(self, command_consumer, error_producer)?;
		let shared = sound.shared();
		Ok((
			Box::new(sound),
			StreamingSoundHandle {
				shared,
				command_producer,
				error_consumer,
			},
		))
	}
}
