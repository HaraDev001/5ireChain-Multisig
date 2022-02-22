use rand::Rng;
use serde::{Deserialize, Serialize};

/// EsgScore wraps the ESG score of a validator. In the mock oracle, we simply return a random integer
/// to mock the ESG score. After the integration of the mock oracle, the score will be generated
/// by a centralized oracle ran by the 5ire team.
#[derive(Serialize, Deserialize)]
pub struct EsgScore {
	pub score: u8,
}

impl EsgScore {
	/// Return a new random ESG score. Future releases of the oracle will need to get an ESG score
	/// per validator.
	pub(crate) fn new() -> EsgScore {
		let mut rng = rand::thread_rng();
		EsgScore { score: rng.gen_range(1..101) }
	}
}
