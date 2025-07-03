use serde::Serialize;

use crate::action::Action;

#[derive(Serialize)]
pub struct Output<T> {
	pub action: Action,
	pub uuid: String,
	pub result: Vec<Vec<T>>,
}

impl<T> Output<T> {
	pub fn new(action: Action, uuid: String, result: Vec<Vec<T>>) -> Self {
		Self { action, uuid, result }
	}
}
