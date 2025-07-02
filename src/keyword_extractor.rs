use flume::{
	Receiver,
	Sender,
	unbounded,
};
use rust_bert::pipelines::keywords_extraction::{
	Keyword,
	KeywordExtractionModel,
};
use tokio::{
	sync::oneshot::channel,
	task::{
		JoinHandle,
		self,
	},
};

use crate::message::KeywordMessage;

pub struct KeywordExtractor {
	sender: Sender<KeywordMessage>
}

impl KeywordExtractor {
	pub fn spawn() -> (JoinHandle<()>, KeywordExtractor) {
		let (sender, receiver) = unbounded();
		let handle = task::spawn_blocking(move || Self::runner(receiver));

		(handle, KeywordExtractor { sender })
	}

	fn runner(receiver: Receiver<KeywordMessage>) {
		if let Ok(model) = KeywordExtractionModel::new(Default::default()) {
			while let Ok((paragraphs, sender)) = receiver.recv() {
				let input: Vec<&str> = paragraphs
					.iter()
					.map(String::as_str)
					.collect();

				if let Some(keywords) = model.predict(&input).ok() {
					sender.send(keywords).ok();
				}
			}
		}
	}

	pub async fn execute(&self, paragraphs: Vec<String>) -> Option<Vec<Vec<Keyword>>> {
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
