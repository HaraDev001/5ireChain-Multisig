// use frame_system::limits::{BlockLength, BlockWeights};
use super::{RuntimeBlockLength, RuntimeBlockWeights};
use frame_election_provider_support::{SortedListProvider, VoteWeight};
use frame_support::{
	parameter_types,
	weights::{constants::BlockExecutionWeight, DispatchClass, Weight},
};
use sp_runtime::Perbill;
use sp_std::{boxed::Box, convert::From, marker::PhantomData};

parameter_types! {
	/// A limit for off-chain phragmen unsigned solution submission.
	///
	/// We want to keep it as high as possible, but can't risk having it reject,
	/// so we always subtract the base block execution weight.
	pub OffchainSolutionWeightLimit: Weight = RuntimeBlockWeights::get()
		.get(DispatchClass::Normal)
		.max_extrinsic
		.expect("Normal extrinsics have weight limit configured by default; qed")
		.saturating_sub(BlockExecutionWeight::get());

	/// A limit for off-chain phragmen unsigned solution length.
	///
	/// We allow up to 90% of the block's size to be consumed by the solution.
	pub OffchainSolutionLengthLimit: u32 = Perbill::from_rational(90_u32, 100) *
		*RuntimeBlockLength::get()
		.max
		.get(DispatchClass::Normal);
}

/// The accuracy type used for genesis election provider;
pub type OnOnChainAccuracy = sp_runtime::Perbill;

/// The election provider of the genesis
pub type GenesisElectionOf<T> =
	frame_election_provider_support::onchain::OnChainSequentialPhragmen<T>;

/// Maximum number of iterations for balancing that will be executed in the embedded miner of
/// pallet-election-provider-multi-phase.
pub const MINER_MAX_ITERATIONS: u32 = 10;

/// A source of random balance for the NPoS Solver, which is meant to be run by the off-chain worker
/// election miner.
pub struct OffchainRandomBalancing;
impl frame_support::pallet_prelude::Get<Option<(usize, sp_npos_elections::ExtendedBalance)>>
	for OffchainRandomBalancing
{
	fn get() -> Option<(usize, sp_npos_elections::ExtendedBalance)> {
		use sp_runtime::{codec::Decode, traits::TrailingZeroInput};
		let iters = match MINER_MAX_ITERATIONS {
			0 => 0,
			max => {
				let seed = sp_io::offchain::random_seed();
				let random = <u32>::decode(&mut TrailingZeroInput::new(&seed))
					.expect("input is padded with zeroes; qed") %
					max.saturating_add(1);
				random as usize
			},
		};

		Some((iters, 0))
	}
}

/// Implementation of `frame_election_provider_support::SortedListProvider` that updates the
/// bags-list but uses [`pallet_staking::Nominators`] for `iter`. This is meant to be a transitionary
/// implementation for runtimes to "test" out the bags-list by keeping it up to date, but not yet
/// using it for snapshot generation. In contrast, a  "complete" implementation would use bags-list
/// for `iter`.
pub struct UseNominatorsAndUpdateBagsList<T>(PhantomData<T>);
impl<T: pallet_bags_list::Config + pallet_staking::Config> SortedListProvider<T::AccountId>
	for UseNominatorsAndUpdateBagsList<T>
{
	type Error = pallet_bags_list::Error;

	fn iter() -> Box<dyn Iterator<Item = T::AccountId>> {
		Box::new(pallet_staking::Nominators::<T>::iter().map(|(n, _)| n))
	}

	fn count() -> u32 {
		pallet_bags_list::Pallet::<T>::count()
	}

	fn contains(id: &T::AccountId) -> bool {
		pallet_bags_list::Pallet::<T>::contains(id)
	}

	fn on_insert(id: T::AccountId, weight: VoteWeight) -> Result<(), Self::Error> {
		pallet_bags_list::Pallet::<T>::on_insert(id, weight)
	}

	fn on_update(id: &T::AccountId, new_weight: VoteWeight) {
		pallet_bags_list::Pallet::<T>::on_update(id, new_weight);
	}

	fn on_remove(id: &T::AccountId) {
		pallet_bags_list::Pallet::<T>::on_remove(id);
	}

	fn regenerate(
		all: impl IntoIterator<Item = T::AccountId>,
		weight_of: Box<dyn Fn(&T::AccountId) -> VoteWeight>,
	) -> u32 {
		pallet_bags_list::Pallet::<T>::regenerate(all, weight_of)
	}

	fn sanity_check() -> Result<(), &'static str> {
		pallet_bags_list::Pallet::<T>::sanity_check()
	}

	fn clear(count: Option<u32>) -> u32 {
		pallet_bags_list::Pallet::<T>::clear(count)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn weight_update_worst_case(who: &T::AccountId, is_increase: bool) -> VoteWeight {
		pallet_bags_list::Pallet::<T>::weight_update_worst_case(who, is_increase)
	}
}
