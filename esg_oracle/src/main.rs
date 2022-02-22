use crate::oracle::EsgScore;
use env_logger::{Builder, Target};
use log::info;
use std::net::SocketAddr;
use warp::{reply::Json, Filter};

mod oracle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut builder = Builder::from_default_env();
	builder.target(Target::Stdout);
	builder.init();

	let default_route = warp::any()
		.and(warp::get())
		.map(warp::reply)
		.with(warp::log(""))
		.map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT));

	let address: SocketAddr = "[::]:8080".parse()?;

	warp::serve(esg().or(default_route)).run(address).await;
	Ok(())
}

fn esg() -> impl Filter<Extract = (Json,), Error = warp::Rejection> + Copy {
	warp::path("oracle")
		.and(warp::get())
		.map(warp::reply)
		.with(warp::log(""))
		.map(|_reply| {
			let score = EsgScore::new();
			info!("returning esg_score: {:?}", score.score);
			warp::reply::json(&score)
		})
}

#[cfg(test)]
mod test {
	use super::*;

	#[tokio::test]
	async fn returns_a_score() {
		let filter = esg();

		let score = warp::test::request().path("/score").filter(&filter).await.unwrap();
		assert!(score.score > 0);
		assert!(score.score < 101);
	}
}
