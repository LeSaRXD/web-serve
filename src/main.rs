use std::{env::args, net::SocketAddr};

use axum::{extract::Path, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	let port: u16 = args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(8080);

	let router = Router::new().route("/{*key}", get(handler));
	let listener = TcpListener::bind(SocketAddr::from(([0u8, 0, 0, 0], port)))
		.await
		.unwrap();

	println!("Server listening on port {port}");
	axum::serve(listener, router).await.unwrap();
}

async fn handler(Path(path): Path<String>) -> String {
	path
}
