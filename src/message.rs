use rust_bert::pipelines::{
	keywords_extraction::Keyword,
	ner::Entity,
};
use tokio::sync::oneshot;

pub type EntityMessage 	= (Vec<String>, oneshot::Sender<Vec<Vec<Entity >>>);
pub type KeywordMessage = (Vec<String>, oneshot::Sender<Vec<Vec<Keyword>>>);
