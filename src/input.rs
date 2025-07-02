use serde::{
	Deserialize,
	Serialize,
};

use crate::action::Action;

#[derive(Serialize, Deserialize)]
pub struct Input<'a> {
	pub action: Action,
	pub uuid: &'a str,

	content: &'a str,
}

impl<'a> Input<'a> {
	pub fn split(&self) -> Vec<String> {
		self.content
			.split("\n\n")
			.map(|i| i.to_owned())
			.collect()
	}
}
