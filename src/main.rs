use std::{env, fs::read_to_string, net::SocketAddr, path::PathBuf};

use axum::{
	extract::Path as PathExt,
	http::{header, StatusCode},
	response::IntoResponse,
	routing::get,
	Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	let port: u16 = env::args()
		.nth(1)
		.and_then(|s| s.parse().ok())
		.or_else(|| env::var("SERVE_PORT").ok()?.parse().ok())
		.unwrap_or(8080);

	let router = Router::new()
		.route("/{*path}", get(handler))
		.route("/", get(root));
	let listener = TcpListener::bind(SocketAddr::from(([0u8, 0, 0, 0], port)))
		.await
		.unwrap();

	println!("Server listening on port {port}");
	axum::serve(listener, router).await.unwrap();
}

async fn root() -> &'static str {
	"Hello from web-serve!"
}

async fn handler(PathExt(path): PathExt<PathBuf>) -> Result<impl IntoResponse, impl IntoResponse> {
	match read_to_string(&path) {
		Ok(file) => Ok((
			StatusCode::OK,
			[(
				header::CONTENT_TYPE,
				mime_guess::from_path(&path)
					.first()
					.map_or_else(|| "application/json".to_owned(), |m| m.to_string()),
			)],
			file,
		)),
		Err(err) => {
			eprintln!("Error reading file: {err}");
			Err(not_found())
		}
	}
}

fn not_found() -> impl IntoResponse {
	(
		StatusCode::NOT_FOUND,
		[(header::CONTENT_TYPE, "text/plain")],
		StatusCode::NOT_FOUND.to_string(),
	)
}
