#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
pub mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

use frame_support::{
    dispatch::DispatchError,
    ensure,
    traits::tokens::nonfungibles::{Create, Inspect, Mutate},
};
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::tokens::nonfungibles::Transfer;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
    #[derive(
        Clone,
        Copy,
        Default,
        Eq,
        Ord,
        PartialEq,
        PartialOrd,
        codec::Decode,
        codec::Encode,
        sp_runtime::RuntimeDebug,
    )]
    pub struct Edition {
        pub proofs: u8,
        pub prints: u16,
    }

    #[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
    #[derive(
        Clone,
        Copy,
        Default,
        Eq,
        Ord,
        PartialEq,
        PartialOrd,
        codec::Decode,
        codec::Encode,
        sp_runtime::RuntimeDebug,
    )]
    pub struct CollectionDetails {
        pub edition: Edition,
        pub next_proof: u8,
        pub next_print: u16,
        pub next_id: u32,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Weights: WeightInfo;
        type NFT: Inspect<<Self as frame_system::Config>::AccountId, ClassId = u32, InstanceId = u32>
            + Create<<Self as frame_system::Config>::AccountId>
            + Mutate<<Self as frame_system::Config>::AccountId>
            + Transfer<<Self as frame_system::Config>::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::type_value]
    pub(super) fn FirstCollectionId<T: Config>() -> u32 {
        1u32
    }
    #[pallet::storage]
    #[pallet::getter(fn next_collection_id)]
    pub(super) type NextCollectionId<T> = StorageValue<_, u32, ValueQuery, FirstCollectionId<T>>;

    #[pallet::storage]
    #[pallet::getter(fn some_map)]
    pub(super) type Collections<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, u32, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub prints: Vec<(
            T::AccountId,
            Vec<(Vec<u8>, Edition, Vec<(bool, T::AccountId)>)>,
        )>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { prints: vec![] }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (who, collections) in &self.prints {
                for (metadata, edition, prints) in collections {
                    let collection_id =
                        match Pallet::<T>::do_create_collection(&who, &metadata, *edition) {
                            Ok(id) => id,
                            Err(err) => panic!("{:?}", err),
                        };

                    for (proof, owner) in prints {
                        match Pallet::<T>::do_create_print(&collection_id, *proof, owner) {
                            Ok(_) => (),
                            Err(err) => panic!("{:?}", err),
                        }
                    }
                }
            }
        }
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CollectionCreated(T::AccountId, u32),
        PrintCreated(T::AccountId, u32, u32),
        PrintTransferred(T::AccountId, u32, u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        CollectionUnavailable,
        CollectionNotFound,
        PrintUnavailable,
        PrintNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::Weights::create_collection())]
        pub fn create_collection(
            origin: OriginFor<T>,
            metadata: Vec<u8>,
            edition: Edition,
            artist: Vec<u8>,
            year: u8,
            name: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let collection_id = Self::do_create_collection(&who, &metadata, edition)?;
            T::NFT::set_typed_class_attribute(&collection_id, b"artist", &artist)?;
            T::NFT::set_typed_class_attribute(&collection_id, b"year", &year)?;
            T::NFT::set_typed_class_attribute(&collection_id, b"name", &name)?;

            Self::deposit_event(Event::CollectionCreated(who, collection_id));
            Ok(())
        }

        #[pallet::weight(T::Weights::create_print())]
        pub fn create_print(
            origin: OriginFor<T>,
            collection_id: u32,
            proof: bool,
            owner: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                T::NFT::class_owner(&collection_id).unwrap_or_default() == who,
                Error::<T>::CollectionNotFound
            );

            let print_id = Self::do_create_print(&collection_id, proof, &owner)?;

            Self::deposit_event(Event::PrintCreated(owner, collection_id, print_id));
            Ok(())
        }

        #[pallet::weight(T::Weights::transfer_print())]
        pub fn transfer_print(
            origin: OriginFor<T>,
            collection_id: u32,
            print_id: u32,
            dest: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                T::NFT::owner(&collection_id, &print_id).unwrap_or(Default::default()) == who,
                Error::<T>::PrintNotFound
            );

            T::NFT::transfer(&collection_id, &print_id, &dest)?;

            Self::deposit_event(Event::PrintTransferred(dest, collection_id, print_id));
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    fn do_create_collection(
        who: &T::AccountId,
        metadata: &Vec<u8>,
        edition: Edition,
    ) -> Result<u32, DispatchError> {
        ensure!(
            !Collections::<T>::contains_key(metadata),
            Error::<T>::CollectionUnavailable
        );

        let collection_id = NextCollectionId::<T>::get();

        T::NFT::create_class(&collection_id, who, who)?;
        T::NFT::set_typed_class_attribute(
            &collection_id,
            b"bca",
            &CollectionDetails {
                edition,
                next_proof: 1,
                next_print: 1,
                next_id: 1,
            },
        )?;

        Collections::<T>::insert(metadata, collection_id);
        NextCollectionId::<T>::set(collection_id + 1);

        Ok(collection_id)
    }

    fn do_create_print(
        collection_id: &u32,
        proof: bool,
        owner: &T::AccountId,
    ) -> Result<u32, DispatchError> {
        let mut details: CollectionDetails =
            match T::NFT::typed_class_attribute(&collection_id, b"bca") {
                Some(details) => details,
                None => Err(Error::<T>::CollectionNotFound)?,
            };

        let print_id = details.next_id;
        if proof {
            let max = details.edition.proofs;
            let actual = details.next_proof;
            ensure!(max >= actual, Error::<T>::PrintUnavailable);
            details.next_proof = actual + 1;
            T::NFT::mint_into(&collection_id, &print_id, owner)?;
            T::NFT::set_typed_attribute(&collection_id, &print_id, b"proof", &true)?;
        } else {
            let max = details.edition.prints;
            let actual = details.next_print;
            ensure!(max >= actual, Error::<T>::PrintUnavailable);
            details.next_print = actual + 1;
            T::NFT::mint_into(&collection_id, &print_id, owner)?;
        }

        details.next_id = print_id + 1;
        T::NFT::set_typed_class_attribute(&collection_id, b"bca", &details)?;
        Ok(print_id)
    }
}
