#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests {
    use ark_ed_on_bls12_377::{Fq, FqConfig};
    use ark_ff::{MontConfig, PrimeField, Zero};
    use poseidon_paramgen::v1::generate;
    use poseidon_permutation::Instance;
    use proptest::prelude::*;

    #[test]
    fn check_optimized_impl_vs_sage() {
        let params_2_to_1 = generate(128, 3, FqConfig::MODULUS, true);
        let mut our_instance = Instance::new(&params_2_to_1);
        let hash_output =
            our_instance.n_to_1_fixed_hash(vec![Fq::zero(), Fq::from(1u64), Fq::from(2u64)]);
        let output_words = our_instance.output_words();
        assert_eq!(hash_output, output_words[1]);
        let expected_output_words = [
            ark_ff::MontFp!(
                "6368779772888548211318735707249600947486536081021109980085678920065117075165"
            ),
            ark_ff::MontFp!(
                "546637332213889354237126997303352456465330747031466737868777261691321847212"
            ),
            ark_ff::MontFp!(
                "1488544471679348337017344865262529731114801536476862121626711131361325263279"
            ),
        ];
        for (a, b) in expected_output_words.iter().zip(output_words.iter()) {
            assert_eq!(*a, *b);
        }
    }

    #[test]
    fn check_unoptimized_impl_vs_sage() {
        let params_2_to_1 = generate(128, 3, FqConfig::MODULUS, true);
        let mut our_instance = Instance::new(&params_2_to_1);
        let hash_output = our_instance.unoptimized_n_to_1_fixed_hash(vec![
            Fq::zero(),
            Fq::from(1u64),
            Fq::from(2u64),
        ]);
        let output_words = our_instance.output_words();
        assert_eq!(hash_output, output_words[1]);
        let expected_output_words = [
            ark_ff::MontFp!(
                "6368779772888548211318735707249600947486536081021109980085678920065117075165"
            ),
            ark_ff::MontFp!(
                "546637332213889354237126997303352456465330747031466737868777261691321847212"
            ),
            ark_ff::MontFp!(
                "1488544471679348337017344865262529731114801536476862121626711131361325263279"
            ),
        ];
        for (a, b) in expected_output_words.iter().zip(output_words.iter()) {
            assert_eq!(*a, *b);
        }
    }

    fn fq_strategy() -> BoxedStrategy<Fq> {
        any::<[u8; 32]>()
            .prop_map(|bytes| Fq::from_le_bytes_mod_order(&bytes[..]))
            .boxed()
    }

    proptest! {
        #[test]
        fn optimized_and_unoptimized_permutation_consistent(elem_1 in fq_strategy(), elem_2 in fq_strategy(), elem_3 in fq_strategy(), elem_4 in fq_strategy(), elem_5 in fq_strategy()) {
            let t = 5;
            let params_4_to_1 = generate(128, t, FqConfig::MODULUS, true);

            let mut our_instance = Instance::new(&params_4_to_1);
            let our_result = our_instance.n_to_1_fixed_hash(vec![elem_1, elem_2, elem_3, elem_4, elem_5]);

            let mut unoptimized_instance = Instance::new(&params_4_to_1);
            let unoptimized_result =
                unoptimized_instance.unoptimized_n_to_1_fixed_hash(vec![elem_1, elem_2, elem_3, elem_4, elem_5]);

            assert_eq!(unoptimized_result, our_result);
        }
    }
}
