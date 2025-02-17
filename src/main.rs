use std::{
	env,
	fs::{self, read_to_string},
	net::SocketAddr,
	path::PathBuf,
};

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
		.unwrap_or(0);

	let router = Router::new()
		.route("/", get(root))
		.route("/favicon.ico", get(favicon))
		.route("/{*path}", get(handler));
	let listener = match TcpListener::bind(SocketAddr::from(([0u8, 0, 0, 0], port))).await {
		Ok(l) => l,
		Err(e) => {
			eprintln!("Unable to bind to 0.0.0.0:{port}: {e}");
			return;
		}
	};

	println!(
		"Server listening: http://127.0.0.1:{}/",
		listener
			.local_addr()
			.expect("Expected a local address")
			.port()
	);
	if let Err(e) = axum::serve(listener, router).await {
		eprintln!("Could not start axum server: {e}");
	}
}

async fn root() -> &'static str {
	"Hello from web-serve!"
}

async fn favicon() -> Result<impl IntoResponse, impl IntoResponse> {
	match fs::read("favicon.ico") {
		Ok(bytes) => Ok((
			StatusCode::OK,
			[(header::CONTENT_TYPE, "image/x-icon")],
			bytes,
		)),
		Err(_) => Err(const {
			(
				StatusCode::OK,
				[(header::CONTENT_TYPE, "image/x-icon")],
				include_bytes!("favicon.ico"),
			)
		}),
	}
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
