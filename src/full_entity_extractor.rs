use flume::{
	Receiver,
	Sender,
	unbounded,
};
use rust_bert::pipelines::ner::{
	Entity,
	NERModel,
};
use tokio::{
	sync::oneshot::channel,
	task::{
		JoinHandle,
		self,
	},
};

use crate::message::EntityMessage;

pub struct FullEntityExtractor {
	sender: Sender<EntityMessage>
}

impl FullEntityExtractor {
	pub fn spawn() -> (JoinHandle<()>, FullEntityExtractor) {
		let (sender, receiver) = unbounded();
		let handle = task::spawn_blocking(move || Self::runner(receiver));

		(handle, FullEntityExtractor { sender })
	}

	fn runner(receiver: Receiver<EntityMessage>) {
		if let Ok(model) = NERModel::new(Default::default()) {
			while let Ok((paragraphs, sender)) = receiver.recv() {
  			let mut output = vec![];
				let mut input = paragraphs
					.iter()
					.map(String::as_str)
					.into_iter();

				let initial: Vec<&str> = input.by_ref().take(50).collect();
				output.append(&mut model.predict_full_entities(&initial));

				let leftovers: Vec<&str> = input.collect();
				if leftovers.len() > 0 {
				  output.append(&mut model.predict_full_entities(&leftovers));
				}

				sender.send(output).ok();
			}
		}
	}

	pub async fn execute(&self, paragraphs: Vec<String>) -> Option<Vec<Vec<Entity>>> {
		let (sender, receiver) = channel();

		if self.sender.send((paragraphs, sender)).is_err() {
			None
		}
		else {
			match receiver.await {
				Ok (r) => Some(r),
				Err(_) => None,
			}
		}
	}
}
