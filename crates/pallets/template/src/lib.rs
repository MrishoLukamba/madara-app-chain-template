#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod utils;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use utils::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_candidate)]
	pub type Candidates<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Candidate<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_certificate)]
	pub type Certificates<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Certificate<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_verifiable_link)]
	pub type VerifiableLinks<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, VerifiableLink<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_institution)]
	pub type Institutions<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Institution<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a new candidate is added to the protocol
		CandidateAdded { candidate: T::AccountId },
		/// Event emitted when a new link from a candidate is generated to be verified by the target institute
		VerifyLinkGenerated { verify_link: Vec<u8> },
		/// Event emitted when a link is confirmed by the target institute
		VerifyLinkConfirmed { verify_link: Vec<u8> },
		/// Event emitted when a link is rejected by the target institute
		VerifyLinkRejected { verify_link: Vec<u8> },
        /// Event emitted when a new institution is added to the protocol
		InsitutionAdded { institution: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Not registered candidate
		NotRegisteredCandidate,
		/// invalid link
		InvalidLink,
		/// Not registered institution
		NotRegisteredInstitution,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_candidate(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn register_institution(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Ok(())
        }

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn register_certificate(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn generate_verifiable_link(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			Ok(())
		}

		/// Verify the proof of the verified link by the institute
		/// This function should be unsigned transaction as anyone can submit the proof to be verified onchain
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn verify_link_proof(origin: OriginFor<T>,link_proof:Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			// we gonna use other people's project ( Bathlomeo and Pia) cairo vm verifier to verify the proof
			Ok(())
		}
	}
}
