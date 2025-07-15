use std::{
	collections::HashMap,
	env,
	io::Error,
};

use futures_util::{
	SinkExt,
	StreamExt,
};
use rust_bert::pipelines::ner::Entity;
use serde_json::Value;
use tokio::net::{
	TcpListener,
	TcpStream,
};
use tokio_tungstenite::tungstenite::Message;

use hey_bert::{
	action::Action,
	full_entity_extractor::FullEntityExtractor,
	input::{
	  Input,
	  ExtractImageEntityInput,
	},
	keyword_extractor::KeywordExtractor,
	out::log,
	output::{
	  ImageOutput,
		ArticleOutput,
	},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let addr = env::args().nth(1).expect("Please specify an IP address/port to listen on");

  if let Ok(listener) = TcpListener::bind(&addr).await {
	  println!("Listening on: {addr}");

	  while let Ok((stream, _)) = listener.accept().await {
	    tokio::spawn(handle_connection(stream));
	  }
  }

  Ok(())
}

async fn handle_connection(stream: TcpStream) {
	let (_handle,  entity_extractor) = FullEntityExtractor::spawn();
	let (_handle, keyword_extractor) = 		KeywordExtractor::spawn();

	if let Ok(		addr		 ) = stream.peer_addr() &&
		 let Ok(mut ws_stream) = tokio_tungstenite::accept_async(stream).await
	{
		println!("Connection from peer: {addr}");

		while let Some(msg) = ws_stream.next().await {
			if let Some(msg) = msg.ok() 				 &&
	      (msg.is_text() || msg.is_binary()) &&
				 let Some(body ) = msg.to_text().ok()
			{
			  if let Ok(v) = serde_json::from_str::<Value>(body) {
					match &v["action"].as_str() {
						Some("ExtractEntities") => {
							log("🍔");
							if let Ok  (input   ) = serde_json::from_str::<Input>(body) &&
							   let Some(entities) = entity_extractor.execute(input.split()).await
							{
								let output = ArticleOutput::new(Action::ExtractEntities, input.uuid, entities);
								if let Some(out) = serde_json::to_string(&output).ok() {
									match ws_stream.send(Message::Text(out.into())).await {
									  Ok (_) => log("🍟"),
										Err(e) => eprintln!("Got error extracting entities: {e:#?}"),
									}
								}
							}
						}
						Some("ExtractKeywords") => {
  						log("🥓");
							if let Ok  (input   ) = serde_json::from_str::<Input>(body) &&
							   let Some(keywords) = keyword_extractor.execute(input.split()).await
							{
								let output = ArticleOutput::new(Action::ExtractKeywords, input.uuid, keywords);
								if let Some(out) = serde_json::to_string(&output).ok() {
									match ws_stream.send(Message::Text(out.into())).await {
									  Ok (_) => log("🍳"),
										Err(e) => eprintln!("Got error extracting keywords: {e:#?}")
									}
								}
							}
						}
						Some("ExtractImageEntities") => {
							log("☕");
							if let Ok  (input   ) = serde_json::from_str::<ExtractImageEntityInput>(body) &&
							   let Some(entities) = entity_extractor.execute(input.split()).await
							{
							  let mut result: HashMap<String, Vec<Entity>> = HashMap::new();
								for (i, k) in input.hrefs().iter_mut().enumerate() {
    						  result.insert(k.to_owned(), entities[i].to_owned());
								}

								let output = ImageOutput::new(result);
								if let Some(out) = serde_json::to_string(&output).ok() {
									match ws_stream.send(Message::Text(out.into())).await {
     							  Ok (_) => log("🥯"),
                    Err(e) => eprintln!("Got error extracting image entities: {e:#?}"),
									}
								}
							}
						}
						Some(_) => {}
						None    => {}
					}
				}
			}
    }
	}
}
