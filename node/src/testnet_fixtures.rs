use hex_literal::hex;
use node_5ire_runtime::constants::currency::*;
use node_primitives::{AccountId, Balance};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_network::config::MultiaddrWithPeerId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::UncheckedInto;
use sp_finality_grandpa::AuthorityId as GrandpaId;
/// Testnet root key
pub fn get_testnet_root_key() -> AccountId {
	// fire  sudo key: 5FJ2iwRV3WURZE1DeVi1Zqrk8pzrFeuryMGF4J1kqVpSvF57
	return hex!["8ede66d550c880426b931fdd4cb6705e29602adbb1a3669095e20d5de4285530"].into()
}

/// Testnet bootnodes
pub fn get_fire_bootnodes() -> Vec<MultiaddrWithPeerId> {
	return vec![
		"/dns/ryuk.genesis.5ire.network/tcp/30333/p2p/12D3KooWDeeDyzgd3buign9B8AkMK5cY6sF4vfzSrP7q28NDB43v"
		.parse()
		.unwrap(),
		"/dns/zeus.genesis.5ire.network/tcp/30333/p2p/12D3KooWSbeiiHncSEo4Gj7fH5ZXv2dHAqRMfwUGxAaM75Vb7nWX"
		.parse()
		.unwrap(),
		"/dns/shiva.genesis.5ire.network/tcp/30333/p2p/12D3KooWSHhgMgFWhQ1xMKjPbUruHc6Rg4z9a6xcdjddHvsJ3rTk"
		.parse()
		.unwrap(),
		"/dns/shakti.genesis.5ire.network/tcp/30333/p2p/12D3KooWPfeoC7v4ojiGGsHdiLyPpRQZPMtV5rW5BnKaioxcEarP"
		.parse()
		.unwrap(),
		"/dns/pratik.genesis.5ire.network/tcp/30333/p2p/12D3KooWDihn1mHvRCzzM8EktpCE6yx39jkkJfeDinnqeHvVKhT7"
		.parse()
		.unwrap(),
		"/dns/prateek.genesis.5ire.network/tcp/30333/p2p/12D3KooWMbvukj3bxPbyWoGsrpLSL1JX9GyvDvVrBxbd6ZGQbG8X"
		.parse()
		.unwrap(),
		"/dns/vilma.genesis.5ire.network/tcp/30333/p2p/12D3KooWBKcTuYTD9VtBxjRP4BMkMqgQJUNo13pRSyvd1WgXSXxa"
		.parse()
		.unwrap(),
		"/dns/dung.genesis.5ire.network/tcp/30333/p2p/12D3KooWJJwtwYpSSP3ZHUETPQbfg6kuibsrBqrkz2ELuEGa4pZd"
		.parse()
		.unwrap(),
		"/dns/mikasa.genesis.5ire.network/tcp/30333/p2p/12D3KooWAb5aKjYH5zBHR8xVyfD7bBzogvzRMt3YG9pRB1RvUGgr"
		.parse()
		.unwrap(),
	];
}

/// Split endowment amount for Commonwealth
pub const COMMONWEALTH_ENDOWMENT: Balance = 100_000_000 * FIREB; // 100_000_000 5IRE
/// Split endowment amount for stash
pub const STASH_ENDOWMENT: Balance = 50_000 * FIREB; // 10000 5IRE
/// Split endowment amount for controllers
pub const CONTROLLER_ENDOWMENT: Balance = 1_000 * FIREB; // 1000 5IRE
pub const STAKED_ENDOWMENT: Balance = 500 * FIREB; // 500 5IRE
pub const EVM_ENDOWMENT: Balance = 500_000 * FIREB;
// pub const FAUCET_ENDOWMENT: Balance = 10_000_000 * FIREB;

/// Split endowment amount for root
// pub const ROOT_ENDOWMENT: Balance = 1_199_967_000 * FIREB; // 1.5B - 3*COMMONWEALTH - 3*STASH - 3*CONTROLLER = 1199967000
pub const ROOT_ENDOWMENT: Balance = 1_500_000_000 * FIREB -
	((4 * COMMONWEALTH_ENDOWMENT) +
		(9 * STASH_ENDOWMENT) +
		(9 * CONTROLLER_ENDOWMENT) +
		(2 * EVM_ENDOWMENT));

/// Genesis allocation that will fit into the "balances" module for
/// Commonwealth/Founders
pub fn get_commonwealth_allocation() -> Vec<(AccountId, Balance)> {
	return vec![
		(
			hex!["8ede66d550c880426b931fdd4cb6705e29602adbb1a3669095e20d5de4285530"].into(), /* SUDO */
			ROOT_ENDOWMENT,
		),
		(
			hex!["4a989c4d3bd9772f2b9bc64981d2aa64704edfbe6f0fca9b047aed1c2a074801"].into(), /* Founder 1 */
			COMMONWEALTH_ENDOWMENT,
		),
		(
			hex!["6ce128526122259fa76eb694f0e332130fbc2f61f84a65b660ca9dc369bdde51"].into(), /* Founder 2 */
			COMMONWEALTH_ENDOWMENT,
		),
		(
			hex!["e2ad49dc473acc094714152f2f8a510abb564213368e94673d8285c23b64a846"].into(), /* Founder 3 */
			COMMONWEALTH_ENDOWMENT,
		),
		(
			hex!["08c9015658a40c5ef14465d07c21db6443225c3dcf4b62f20762c4eef3d64777"].into(), /* Team Account */
			COMMONWEALTH_ENDOWMENT,
		),
		(
			hex!["80cf86904f8f57c8de318eedd70617a2e5b0d2e4cba3276c3e923899bc7ea610"].into(), /* Stash account validator 1 */
			STASH_ENDOWMENT,
		),
		(
			hex!["58e8c326d42ffcd22600e5d6e5792ba74d28d7445557de63db132dfeaa85f145"].into(), /* Controller account Validator 1 */
			CONTROLLER_ENDOWMENT,
		),
		(
			hex!["84a19d62208287b6f061b303c0760dfce3ba6d6e0c7e3bcf8c44e94e86917614"].into(), /* Stash account validator 2 */
			STASH_ENDOWMENT,
		),
		(
			hex!["cc39ca76215a08c42f14d4ece5744275575bed4c4ee7619534a0e7cafca96d08"].into(), /* Controller account Validator 2 */
			CONTROLLER_ENDOWMENT,
		),
		(
			hex!["865b4d7a9697c3494f782327c6cd99be199f5b4d25aa4878e166a327b693f20e"].into(), /* Stash account validator 3 */
			STASH_ENDOWMENT,
		),
		(
			hex!["62e30e3554a3323b63e200046371d5b8ae5cd59c049904cd1ea71b31883a5443"].into(), /* Controller account Validator 3 */
			CONTROLLER_ENDOWMENT,
		),
		(
			hex!["14ad12edfe60a9aed26c71daef050c7011581e66bcbb4962e3836eb3c236a82a"].into(), /* Stash account validator 4 */
			STASH_ENDOWMENT,
		),
		(
			hex!["6808a312b2441f39476c6477c19f097d508c7fc1b5b4172baa3c22048ae98344"].into(), /* Controller account Validator 4 */
			CONTROLLER_ENDOWMENT,
		),
		(
			hex!["7a7b097138a82c18ea1cd464e0d5c507225a2808908a1ba7be6d39ff5037e516"].into(), /* Stash account validator 5 */
			STASH_ENDOWMENT,
		),
		(
			hex!["a00a8d38d6acb4777943670ff16644d8a554aba59dca073152034e44e47b5907"].into(), /* Controller account Validator 5 */
			CONTROLLER_ENDOWMENT,
		),
		(
			hex!["c044d7ec5e2447e695ce22539b1140622b2dd0c446aa5c5506eae4d50d2d205a"].into(), /* Stash account validator 6 */
			STASH_ENDOWMENT,
		),
		(
			hex!["9a544cc9ce99d701f0bf12b7797c9f4fc040d6a8d12bdeab0a5d2c95ede40b40"].into(), /* Controller account Validator 6 */
			CONTROLLER_ENDOWMENT,
		),
		(
			hex!["1817ebd0a02fa883d7b0ffcf8d8549acc7af7573dd9e8041fe35057de0062c6b"].into(), /* Stash account validator 7 */
			STASH_ENDOWMENT,
		),
		(
			hex!["2cbdf0282f40ebb92d0f373541440ddafd4f52d767437efca8fbf86da6edef7c"].into(), /* Controller account Validator 7 */
			CONTROLLER_ENDOWMENT,
		),
		(
			hex!["582c8904336f304751896419618a05edb06a6a03653eed49d1a73df62929c774"].into(), /* Stash account validator 8 */
			STASH_ENDOWMENT,
		),
		(
			hex!["2671c43b4a2550da193acfdcce804ba98dfa6746e44bf3159b2418b462d13242"].into(), /* Controller account Validator 8 */
			CONTROLLER_ENDOWMENT,
		),
		(
			hex!["7e095d67ca2c9a07607cffcea46d71f42e64c1bbb247e19c75a6940433e9fb2d"].into(), /* Stash account validator 9 */
			STASH_ENDOWMENT,
		),
		(
			hex!["58f0a23aa855472bda3c555f6295e094e34be94e81c5da1565745298bef7f702"].into(), /* Controller account Validator 9 */
			CONTROLLER_ENDOWMENT,
		),
	]
}

// Fire initial authorities
pub fn get_fire_initial_authorities(
) -> Vec<(AccountId, AccountId, Balance, GrandpaId, AuraId, ImOnlineId, AuthorityDiscoveryId)> {
	// StashId, ControllerId,GrandpaId,Auraid, ImonlineID, AuthorityDiscoveryID
	return vec![
		(
			// Validator 1
			// 5EybdkMchboKNVUNGEZnjh29mVvNaXahYBK954KGDz4DnvKm
			hex!["80cf86904f8f57c8de318eedd70617a2e5b0d2e4cba3276c3e923899bc7ea610"].into(),
			// 5E5HDURF5dzqLrEQHbYdmZVBsqn3rosXyZLs3LGXMkRaMZ4c
			hex!["58e8c326d42ffcd22600e5d6e5792ba74d28d7445557de63db132dfeaa85f145"].into(),
			STAKED_ENDOWMENT,
			// 5EC3DziW47Hme2zzHjJ6SornoC1Hb5LubwSJhEgwLXGE3NtD
			hex!["5e1061dbb97bf47b4012b52dffdec98b67abb7f2dc664fd5c0353542261da130"]
				.unchecked_into(),
			// 5CPNhGdysX99M6cXFau33uNQ3XoxxCwWKGPFDsRUqSEE1TAB
			hex!["0e3d285251e77954cfbb86a74a823a628a57f255c67cd61d95622caf0a7b210a"]
				.unchecked_into(),
			// 5CPNhGdysX99M6cXFau33uNQ3XoxxCwWKGPFDsRUqSEE1TAB
			hex!["0e3d285251e77954cfbb86a74a823a628a57f255c67cd61d95622caf0a7b210a"]
				.unchecked_into(),
			// 5CPNhGdysX99M6cXFau33uNQ3XoxxCwWKGPFDsRUqSEE1TAB
			hex!["0e3d285251e77954cfbb86a74a823a628a57f255c67cd61d95622caf0a7b210a"]
				.unchecked_into(),
		),
		(
			// Validator 2
			// 5F4cBqCczHRwWxoASKH5A1iuWkbys5Jbrq7ncT1nMr58y1ih
			hex!["84a19d62208287b6f061b303c0760dfce3ba6d6e0c7e3bcf8c44e94e86917614"].into(),
			// 5GgUnxHWRfv3YcQMVycvRiEcFy3o7qsDu9cmAKpjcMBFfJWa
			hex!["cc39ca76215a08c42f14d4ece5744275575bed4c4ee7619534a0e7cafca96d08"].into(),
			STAKED_ENDOWMENT,
			// 5EBykkVMYZQSGnPEuMTigf8Jnxs7hzBjbZjtZ8YkQjwncNt8
			hex!["5e04b3b9a7eeaba22193cddc822aea3d08046b0f9fdcaeeb09b0fb5206529edc"]
				.unchecked_into(),
			// 5EToC57L6xUZu45SWXPjBmuHRLPKkMfYwZ1VqAWrLsHMgNaD
			hex!["6a15119fd36f2531169a2714687ff865f01b575497a0c2722e8122a09fafb004"]
				.unchecked_into(),
			// 5EToC57L6xUZu45SWXPjBmuHRLPKkMfYwZ1VqAWrLsHMgNaD
			hex!["6a15119fd36f2531169a2714687ff865f01b575497a0c2722e8122a09fafb004"]
				.unchecked_into(),
			// 5EToC57L6xUZu45SWXPjBmuHRLPKkMfYwZ1VqAWrLsHMgNaD
			hex!["6a15119fd36f2531169a2714687ff865f01b575497a0c2722e8122a09fafb004"]
				.unchecked_into(),
		),
		(
			// Validator 3
			// 5F6sPwHTQy2dVeiShASP42dyUfYqwoiy1SyqYVPoxv6rotYG
			hex!["865b4d7a9697c3494f782327c6cd99be199f5b4d25aa4878e166a327b693f20e"].into(),
			// 5EJMzvME9BGgRDJ4etmtiYYn8BgBPa6UsiLjALt4xVJTNCfe
			hex!["62e30e3554a3323b63e200046371d5b8ae5cd59c049904cd1ea71b31883a5443"].into(),
			STAKED_ENDOWMENT,
			// 5D3CMYHMmZ7ms8jjR3kd15kz6UxU2CpZ46AJqZAcZutWDdS4
			hex!["2b15acbb4f040d38ceeda2901876e9249843b55d3b26c01a321a63ec6d05d1f1"]
				.unchecked_into(),
			// 5EFrvEvjzVYFiJAcwQHXA1i2s6VtE4VvNVmakFhhj8apkhCw
			hex!["60faab9e31f01708694a2c74f883d010c2d9fc92f7485b88978f66c7b3c9db22"]
				.unchecked_into(),
			// 5EFrvEvjzVYFiJAcwQHXA1i2s6VtE4VvNVmakFhhj8apkhCw
			hex!["60faab9e31f01708694a2c74f883d010c2d9fc92f7485b88978f66c7b3c9db22"]
				.unchecked_into(),
			// 5EFrvEvjzVYFiJAcwQHXA1i2s6VtE4VvNVmakFhhj8apkhCw
			hex!["60faab9e31f01708694a2c74f883d010c2d9fc92f7485b88978f66c7b3c9db22"]
				.unchecked_into(),
		),
		(
			// Validator 4
			// 5CXpED7V1X3BvhRbN5YEurUyuLngunMnsXpfBDXWW3Roq4ko
			hex!["14ad12edfe60a9aed26c71daef050c7011581e66bcbb4962e3836eb3c236a82a"].into(),
			// 5ER7QKbTb7kzgTq93rCpXEjsP5wQJeq3BpU88xu8RnCLASGR
			hex!["6808a312b2441f39476c6477c19f097d508c7fc1b5b4172baa3c22048ae98344"].into(),
			STAKED_ENDOWMENT,
			// 5D6WE9fqXRgkaSPdYXQ2ff2NxdL7SbuGAsNeyG3kSm9SVBtY
			hex!["2d9b93502b85c543b7a8f2b3dd5b6b027841a8c47dc9ff133a99a864bba57878"]
				.unchecked_into(),
			// 5FX5ycCYJajjS4kW6P3RRdKa2PH2DpBeLnqAmRTdMXkihHT4
			hex!["98d38ace230ccb6a2cd414d86a744013ace393691b6b19c6ca690dffd2d1f864"]
				.unchecked_into(),
			// 5FX5ycCYJajjS4kW6P3RRdKa2PH2DpBeLnqAmRTdMXkihHT4
			hex!["98d38ace230ccb6a2cd414d86a744013ace393691b6b19c6ca690dffd2d1f864"]
				.unchecked_into(),
			// 5FX5ycCYJajjS4kW6P3RRdKa2PH2DpBeLnqAmRTdMXkihHT4
			hex!["98d38ace230ccb6a2cd414d86a744013ace393691b6b19c6ca690dffd2d1f864"]
				.unchecked_into(),
		),
		(
			// Validator 5
			// 5EqJFNpk3L8GP52XtTJxo4ZhUub3vBHGUo4uHianiv78f93b
			hex!["7a7b097138a82c18ea1cd464e0d5c507225a2808908a1ba7be6d39ff5037e516"].into(),
			// 5FgYeqeGgkvkcgi8AJbW84rLD7J6VSafv1U8vtUkFacNBp64
			hex!["a00a8d38d6acb4777943670ff16644d8a554aba59dca073152034e44e47b5907"].into(),
			STAKED_ENDOWMENT,
			// 5D3gRBD3wVZXsQkrb2zCUw8Uceu1m3aa5DCjMySUuWSEBbeT
			hex!["2b7424678eb0c5638f981418334418bd44bc6a7736b40da3ab53140e4062a64d"]
				.unchecked_into(),
			// 5H1M1of5eEe77WFVr3wqG5kXSuMRu7KZKAcvFrkAhyLT3s8N
			hex!["da9d44df1387389c715a5d1c6d28a5b0ad9af2ebeb245e74022e9b739a351015"]
				.unchecked_into(),
			// 5H1M1of5eEe77WFVr3wqG5kXSuMRu7KZKAcvFrkAhyLT3s8N
			hex!["da9d44df1387389c715a5d1c6d28a5b0ad9af2ebeb245e74022e9b739a351015"]
				.unchecked_into(),
			// 5H1M1of5eEe77WFVr3wqG5kXSuMRu7KZKAcvFrkAhyLT3s8N
			hex!["da9d44df1387389c715a5d1c6d28a5b0ad9af2ebeb245e74022e9b739a351015"]
				.unchecked_into(),
		),
		(
			// Validator 6
			// 5GQoW4JtXN5h1m6RznqZtx8qVyJCzRDD1tFyCefTVyqULewJ
			hex!["c044d7ec5e2447e695ce22539b1140622b2dd0c446aa5c5506eae4d50d2d205a"].into(),
			// 5FZ4Gq48Dai8fmEePLh9Lfyaa5q4NhRccDt9GgLvxx6EfPQp
			hex!["9a544cc9ce99d701f0bf12b7797c9f4fc040d6a8d12bdeab0a5d2c95ede40b40"].into(),
			STAKED_ENDOWMENT,
			// 5CmXX5gzXUFsqReAg3NLz7LremYRmdMEqkoFsSXkwdsVVHBA
			hex!["1f22430216727733ab383001d3c7ad624f073af0fd7bb515e808b5b327f6e2ec"]
				.unchecked_into(),
			// 5HLCTRSLMvqAAk7SPbVqzPwfvMxx5ScUzgkAqUMmatAfTZcX
			hex!["e8fe1f2ec16b6d803e6486b1c29b72d1de3679dae19732d0a6d9f1ce919c6e54"]
				.unchecked_into(),
			// 5HLCTRSLMvqAAk7SPbVqzPwfvMxx5ScUzgkAqUMmatAfTZcX
			hex!["e8fe1f2ec16b6d803e6486b1c29b72d1de3679dae19732d0a6d9f1ce919c6e54"]
				.unchecked_into(),
			// 5HLCTRSLMvqAAk7SPbVqzPwfvMxx5ScUzgkAqUMmatAfTZcX
			hex!["e8fe1f2ec16b6d803e6486b1c29b72d1de3679dae19732d0a6d9f1ce919c6e54"]
				.unchecked_into(),
		),
		(
			// Validator 7
			// 5CcJ7UZufk6XdxG6h28fqPGSa9MWyLEVcNEgFjgq3tkzRULc
			hex!["1817ebd0a02fa883d7b0ffcf8d8549acc7af7573dd9e8041fe35057de0062c6b"].into(),
			// 5D5NPRn7PBYWuHu5DcVVqzHUAciGNfiesVv79BRFjrDKLi4T
			hex!["2cbdf0282f40ebb92d0f373541440ddafd4f52d767437efca8fbf86da6edef7c"].into(),
			STAKED_ENDOWMENT,
			// 5EkrXYv7wXZpHQWntEjwPXzC6PSKZBK2wFXZ1GfMBkswsjR7
			hex!["7717785eef4b8fb43beb6ee386e869a79a7fbd819c48c071638d70b35718f8da"]
				.unchecked_into(),
			// 5Ev6Mhx5Uk44MsLDACqRtenc1qd3d6g8ZKryfRbzeRLnoJFq
			hex!["7e233c55ac30f6b0c205f824526f7a19c2a7ea7a24ba674ec637464a6a402073"]
				.unchecked_into(),
			// 5Ev6Mhx5Uk44MsLDACqRtenc1qd3d6g8ZKryfRbzeRLnoJFq
			hex!["7e233c55ac30f6b0c205f824526f7a19c2a7ea7a24ba674ec637464a6a402073"]
				.unchecked_into(),
			// 5Ev6Mhx5Uk44MsLDACqRtenc1qd3d6g8ZKryfRbzeRLnoJFq
			hex!["7e233c55ac30f6b0c205f824526f7a19c2a7ea7a24ba674ec637464a6a402073"]
				.unchecked_into(),
		),
		(
			// Validator 8
			// 5E4KJPwJ1gk7hByyiFi9efPCBnwqznMhM8M5hPnKXdS9TjTJ
			hex!["582c8904336f304751896419618a05edb06a6a03653eed49d1a73df62929c774"].into(),
			// 5Cw7UMjcoyS8dxmUGcxAKGxfmEsqnynnBd7a2idMBGduUwWV
			hex!["2671c43b4a2550da193acfdcce804ba98dfa6746e44bf3159b2418b462d13242"].into(),
			STAKED_ENDOWMENT,
			// 5DGLxwJ8YMVWdub81pMDqueCWSdXvwwpWTXPKAHxDJcC8d2T
			hex!["351cd98cfac65e56a4fb98b1b90b7b182fdb89f20f0a673c5ce47259843fe9da"]
				.unchecked_into(),
			// 5Fhn8iBTWYB2efrnbjp38SqYR26yYVtVFdZyTbQjreP6wcSK
			hex!["a0fb2d35739a962baf26a031b1fa106bdeee58a69d7527af50048a9c9550ce71"]
				.unchecked_into(),
			// 5Fhn8iBTWYB2efrnbjp38SqYR26yYVtVFdZyTbQjreP6wcSK
			hex!["a0fb2d35739a962baf26a031b1fa106bdeee58a69d7527af50048a9c9550ce71"]
				.unchecked_into(),
			// 5Fhn8iBTWYB2efrnbjp38SqYR26yYVtVFdZyTbQjreP6wcSK
			hex!["a0fb2d35739a962baf26a031b1fa106bdeee58a69d7527af50048a9c9550ce71"]
				.unchecked_into(),
		),
		(
			// Validator 9
			// 5EuxfxovEXNHfQhmMFbUy96hQ8G9VSM3T8UbNVUUBetn8fGP
			hex!["7e095d67ca2c9a07607cffcea46d71f42e64c1bbb247e19c75a6940433e9fb2d"].into(),
			// 5E5KZ6SHqigJF2ZcLCsRosYJYWxBqUZbP3ovbFC2XxDG8iRJ
			hex!["58f0a23aa855472bda3c555f6295e094e34be94e81c5da1565745298bef7f702"].into(),
			STAKED_ENDOWMENT,
			// 5FU7dAEMrxDmpMt8y7yqaobtkbDK6veUAxC45zXXgSJf8t8Y
			hex!["968f5a1093f17a36480d294f83b532bd38ffc611a3f793d9071a99ffd013991f"]
				.unchecked_into(),
			// 5G1FCvbLLC7XSUSQKYoZ4TGGA6rfuGb3tpZeTJRnnB6nGqKu
			hex!["ae4d7bafd63b6bfb7da29fb85776b2d2fdcb4d8834af3def01c168641549af5b"]
				.unchecked_into(),
			// 5G1FCvbLLC7XSUSQKYoZ4TGGA6rfuGb3tpZeTJRnnB6nGqKu
			hex!["ae4d7bafd63b6bfb7da29fb85776b2d2fdcb4d8834af3def01c168641549af5b"]
				.unchecked_into(),
			// 5G1FCvbLLC7XSUSQKYoZ4TGGA6rfuGb3tpZeTJRnnB6nGqKu
			hex!["ae4d7bafd63b6bfb7da29fb85776b2d2fdcb4d8834af3def01c168641549af5b"]
				.unchecked_into(),
		),
	]
}
