use std::collections::HashMap;

use serde::{
	Deserialize,
	Serialize,
};

use crate::action::Action;

#[derive(Serialize, Deserialize)]
pub struct Input {
	pub action: Action,
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

#[derive(Serialize, Deserialize)]
pub struct ExtractImageEntityInput {
  pub action: Action,

  images: HashMap<String, String>,
}

impl ExtractImageEntityInput {
	pub fn split(&self) -> Vec<String> {
		self.images
		  .values()
      .map(|v| v.to_owned())
			.collect()
	}

	pub fn hrefs(&self) -> Vec<String> {
	self.images
    .keys()
    .map(|k| k.to_owned())
    .collect()
	}
}
