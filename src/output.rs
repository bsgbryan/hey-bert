use rust_bert::pipelines::ner::Entity;
use serde::Serialize;

#[derive(Serialize)]
pub struct Output {
	pub uuid: String,
	pub result: Vec<Vec<Entity>>,
}
