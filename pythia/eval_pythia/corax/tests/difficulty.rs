#[cfg(test)]
mod test {
    use std::{ffi::CString, str::FromStr};

    use corax::predict_difficulty;
    use corax_sys::{
        CORAX_TRUE, corax_map_nt, corax_msa_compute_features, corax_msa_predict_difficulty,
        corax_phylip_load,
    };

    #[test]
    fn dna_msas() {
        // std::list<PredAttr> dna_msas = {
        //         std::make_tuple(std::string(STRINGIFY(DATAPATH/DNA_1.phy)), 0.70, 1.0, 0.73),
        //         std::make_tuple(std::string(STRINGIFY(DATAPATH/DNA_2.phy)), 0.33, 1.0, 0.03),
        //         std::make_tuple(std::string(STRINGIFY(DATAPATH/DNA_3.phy)), 0.0, 0.04, 0.02)
        // };
        let dna_msas = [
            (1, 0.70, 1., 0.73),
            (2, 0.33, 1., 0.03),
            (3, 0., 0.04, 0.02),
        ];

        for (i, rf_dist, prop_unique_topos, expected_diff) in dna_msas {
            let filename = format!(
                "{}/bindings/coraxlib/lib/difficulty_prediction/test/unit/data/DNA_{i}.phy",
                env!("CARGO_MANIFEST_DIR")
            );
            let received = predict_difficulty(&filename, corax::SequenceType::DNA);
            let diff_from_test_data = unsafe {
                let msa = corax_phylip_load(
                    CString::from_str(&filename).unwrap().as_ptr(),
                    CORAX_TRUE as i32,
                );
                let msa_features = corax_msa_compute_features(msa, 4, corax_map_nt.as_ptr());
                corax_msa_predict_difficulty(msa_features, rf_dist, prop_unique_topos)
            };
            assert_eq!(expected_diff, diff_from_test_data);
            assert_eq!(rf_dist, received.avg_rrf);
            assert_eq!(prop_unique_topos, received.prop_unique_topos);
            assert_eq!(expected_diff, received.difficulty);
        }
    }
}
