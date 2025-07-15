use std::collections::HashMap;

use serde::Serialize;

use crate::action::Action;

#[derive(Serialize)]
pub struct ArticleOutput<T> {
	pub action: Action,
	pub uuid: String,
	pub result: Vec<Vec<T>>,
}

impl<T> ArticleOutput<T> {
	pub fn new(action: Action, uuid: String, result: Vec<Vec<T>>) -> Self {
		Self { action, uuid, result }
	}
}

#[derive(Serialize)]
pub struct ImageOutput<T> {
	pub action: Action,
	pub result: HashMap<String, Vec<T>>,
}

impl<T> ImageOutput<T> {
	pub fn new(result: HashMap<String, Vec<T>>) -> Self {
		Self { action: Action::ExtractImageEntities, result }
	}
}
