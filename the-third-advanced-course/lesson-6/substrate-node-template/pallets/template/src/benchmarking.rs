//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 1000;
		let caller: T::AccountId = account("caller", 0, 0);
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}
}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
