use std::{
	env,
	io::Error,
};

use futures_util::{
	SinkExt,
	StreamExt,
};
use tokio::net::{
	TcpListener,
	TcpStream,
};
use tokio_tungstenite::tungstenite::Message;

use hey_bert::{
	input::Input,
	model::FullEntityExtractor,
	output::Output,
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
	let (_handle, extractor) = FullEntityExtractor::spawn();

	if let Ok(addr) 		 		 = stream.peer_addr() &&
		 let Ok(mut ws_stream) = tokio_tungstenite::accept_async(stream).await
	{
		println!("Connection from peer: {addr}");

		while let Some(msg) = ws_stream.next().await {
			if let Some(msg) = msg.ok() 				&&
	      (msg.is_text() || msg.is_binary())&&
				 let Some(body) 		= msg.to_text().ok()
			{
				match serde_json::from_str::<Input>(body) {
					Ok(input) => {
						if let Some(entities) = extractor.execute(input.split()).await {
							let output = Output { uuid: input.uuid, result: entities };
			        if let Some(out) = serde_json::to_string(&output).ok() &&
			        	 let Err(e) = ws_stream.send(Message::Text(out.into())).await
			        { eprintln!("Got error: {e:#?}"); }
						}
					}
					Err(e) => eprintln!("Got error {e:#?}")
				}
			}
    }
	}
}
