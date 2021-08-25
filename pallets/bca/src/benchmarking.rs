#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Bca;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin as SystemOrigin;
use sp_std::{convert::TryInto, prelude::*};

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

benchmarks! {
    create_collection {
        let s in 12 .. 32;
        let caller: T::AccountId = whitelisted_caller();
    }: _(
        SystemOrigin::Signed(caller.clone()),
        Default::default(),
        Edition{proofs: u8::MAX, prints: u16::MAX},
        vec![s as u8; s.try_into().unwrap()],
        u8::MAX,
        vec![s as u8; s.try_into().unwrap()]
    )
    verify {
        assert_last_event::<T>(Event::CollectionCreated(caller.clone(), 1).into());
    }

    create_print {
        let caller: T::AccountId = whitelisted_caller();
        Bca::<T>::create_collection(
            SystemOrigin::Signed(caller.clone()).into(),
            Default::default(),
            Edition{proofs: u8::MAX, prints: u16::MAX},
            Default::default(),
            u8::MAX,
            Default::default(),
        )?;
    }: _(SystemOrigin::Signed(caller.clone()), 1, true, caller.clone())
    verify {
        assert_last_event::<T>(Event::PrintCreated(caller, 1, 1).into());
    }

    transfer_print {
        let caller: T::AccountId = whitelisted_caller();
        let dest: T::AccountId = account::<T::AccountId>("dest", 0, 0);
        Bca::<T>::create_collection(
            SystemOrigin::Signed(caller.clone()).into(),
            Default::default(),
            Edition{proofs: u8::MAX, prints: u16::MAX},
            Default::default(),
            u8::MAX,
            Default::default(),
        )?;
        Bca::<T>::create_print(SystemOrigin::Signed(caller.clone()).into(), 1, false, caller.clone())?;
    }: _(SystemOrigin::Signed(caller.clone()), 1, 1, dest.clone())
    verify {
        assert_last_event::<T>(Event::PrintTransferred(dest, 1, 1).into());
    }
}

impl_benchmark_test_suite!(Bca, crate::mock::new_test_ext(), crate::mock::Test);
