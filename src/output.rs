use serde::Serialize;

use crate::action::{
	Action,
	Extract,
};

#[derive(Serialize)]
pub struct Output<'a, T> {
	pub action: Action,
	pub uuid: &'a str,
	pub result: Vec<Vec<T>>,
}

impl<'a, T> Output<'a, T> {
	pub fn new(action: Extract, uuid: &'a str, result: Vec<Vec<T>>) -> Self {
		Self { action: Action::Extract(action), uuid, result }
	}
}
