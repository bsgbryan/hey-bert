use rust_bert::pipelines::ner::Entity;
use tokio::sync::oneshot;

pub type Message = (Vec<String>, oneshot::Sender<Vec<Vec<Entity>>>);
