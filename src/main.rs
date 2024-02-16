#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
#[cfg(test)] mod tests;
mod utils;

use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use error::MyError;
use tracing::info;

async fn hello_world() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[derive(serde::Serialize)]
struct Response {
  elf: usize,
  #[serde(rename(serialize = "elf on a shelf"))]
  count_elf_on_a_shelf: usize,
  #[serde(rename(serialize = "shelf with no elf on it"))]
  count_shelf_with_no_elf_on_it: usize,
}

/// an endpoint that takes a POST request with a raw string as input and count how many times the
/// substring "elf" appears.
// use String for content type `text/plain`
async fn elf_regex(elf_string: String) -> impl IntoResponse {
  // let elf = elf_string.to_lowercase();
  // let elf = elf_string.matches("elf").count();
  // let count_elf_on_a_shelf = elf_string.matches("elf on a shelf").count();

  // // the number of occurences of "shelf" such that "elf on a " does not occur before it
  // let count_shelf = elf_string.matches("shelf").count();
  // let count_shelf_with_no_elf_on_it = count_shelf - count_elf_on_a_shelf;

  // info!("prompt: {}", elf_string);
  // info!(
  //   "elf: {elf}, shelf: {count_shelf}, elf on a shelf: {}, shelf with no elf on it: {}",
  //   count_elf_on_a_shelf, count_shelf_with_no_elf_on_it
  // );
  // let response = Response { elf, count_elf_on_a_shelf, count_shelf_with_no_elf_on_it };
		let elf = elf_string.matches("elf").count();
		let count_elf_on_a_shelf = fancy_regex::Regex::new("elf(?= on a shelf)")
			.expect("Could not make regex")
			.captures_iter(&elf_string)
			.count();
		let count_shelf = elf_string.matches("shelf").count();
		let count_shelf_with_no_elf_on_it = count_shelf - count_elf_on_a_shelf;
			// num::CheckedSub::checked_sub(&shelf, &elf_on_a_shelf).expect("Arithmetic Failure");

  let response = Response { elf, count_elf_on_a_shelf, count_shelf_with_no_elf_on_it };
  Json(response)
}

#[shuttle_runtime::main]
async fn main(
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();

  info!("hello thor");

  let router = Router::new()
    .route("/", get(hello_world))
    .route("/6", post(elf_regex))
    .route("/-1/error", get(error_handler))
    .route("/-1/health", get(|| async { StatusCode::OK }));

  Ok(router.into())
}

async fn elf_no_regex(elf_string: String) -> impl IntoResponse {
  let elf = elf_string.to_lowercase();
  let elf = elf_string.matches("elf").count();
  let elf_on_a_shelf = elf_string.matches("elf on a shelf").count();

  // the number of occurences of "shelf" such that "elf on a " does not occur before it
  let shelf = elf_string.matches("shelf").count();
  let shelf_with_no_elf_on_it = shelf - elf_on_a_shelf;

  info!(
    "elf: {}, elf on a shelf: {}, shelf with no elf on it: {}",
    elf, elf_on_a_shelf, shelf_with_no_elf_on_it
  );
  let response = Response {
    elf,
    count_elf_on_a_shelf: elf_on_a_shelf,
    count_shelf_with_no_elf_on_it: shelf_with_no_elf_on_it,
  };
  Json(response)
}
