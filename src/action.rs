use std::fmt::Display;

use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize)]
pub enum Action {
	ExtractEntities,
	ExtractKeywords,
	ExtractImageEntities,
}

impl Display for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Action::ExtractEntities      => writeln!(f, "Action::ExtractEntities"),
			Action::ExtractKeywords      => writeln!(f, "Action::ExtractKeywords"),
			Action::ExtractImageEntities => writeln!(f, "Action::ExtractImageEntities"),
		}
	}
}
