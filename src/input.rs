use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Input {
	pub uuid: String,

	content: String,
}

impl Input {
	pub fn split(&self) -> Vec<String> {
		self.content
			.split("\n\n")
			.map(|i| i.to_owned())
			.collect()
	}
}
