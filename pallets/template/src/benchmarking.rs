//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}
}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);


// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use crate::mock::{new_test_ext, Test};
// 	use frame_support::assert_ok;

// 	#[test]
// 	fn test_benchmarks() {
// 		new_test_ext().execute_with(|| {
// 			assert_ok!(test_benchmark_do_something::<Test>());
// 		});
// 	}
// }
