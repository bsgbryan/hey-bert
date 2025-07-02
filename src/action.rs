use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize)]
pub enum Action {
	Extract(Extract),
}

#[derive(Serialize, Deserialize)]
pub enum Extract {
	Entities,
	Keywords,
}
