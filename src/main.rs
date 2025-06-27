use std::{
	env,
	io::Error,
};

use futures_util::{
	SinkExt,
	StreamExt,
};
use hey_bert::model::FullEntityExtractor;
use tokio::net::{
	TcpListener,
	TcpStream,
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

	if let Ok(addr) 		 = stream.peer_addr() &&
		 let Ok(mut ws_stream) = tokio_tungstenite::accept_async(stream).await
	{
		println!("Connection from peer: {addr}");

		while let Some(msg) = ws_stream.next().await {
			if let Some(msg) = msg.ok() {
	      if msg.is_text() || msg.is_binary() {
					if let Some(body) = msg.to_text().ok() {
						println!("Message: {body}");
						if let Some(entities) = extractor.execute(vec![body.to_owned()]).await {
							println!("Got {entities:#?}");
						}
		        ws_stream.send(msg).await.ok();
					}
	      }
			}
    }
	}
}
