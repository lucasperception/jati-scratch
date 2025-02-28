// #include <iostream>
// #include <vector>
// #include <map>
// #include <tuple>
// #include <math.h>
// #include "corax/corax.h"
// #include "difficulty.h"

use corax_bindings::{
    corax_msa_destroy, corax_msa_t, corax_phylip_load, corax_split_t, corax_utree_create_parsimony,
    corax_utree_destroy, corax_utree_split_create, corax_utree_split_destroy,
    corax_utree_split_rf_distance, corax_utree_t,
};
use cpythia_bindings::{corax_msa_compute_features, corax_msa_predict_difficulty};
use std::{
    collections::HashMap,
    ffi::{self, CString},
};

// std::vector<corax_split_t *> get_pars_splits(corax_msa_t *msa, int n_trees)
// {
fn get_pars_splits(msa: *const corax_msa_t, n_trees: i32) -> Vec<*const corax_split_t> {
    /**
     * Infers n_trees parsimony trees for the given MSA using Coraxlib, creates and returns the splits.
     */
    //   std::vector<corax_utree_t *> pars_trees(n_trees);
    let mut pars_trees: Vec<*const corax_utree_t> = vec![std::ptr::null(); n_trees];
    //   std::vector<corax_split_t *> splits(n_trees);
    let mut splits: Vec<*const corax_split_t> = vec![std::ptr::null(); n_trees];
    //   std::map<std::string, unsigned int> labelToId;
    let mut label_to_id: HashMap<CString, u32> = Default::default();
    //   unsigned int score;
    let mut score: u32;
    //   int n_taxa = msa->count;
    let n_taxa: i32 = unsafe { *msa }.count;
    //   for (int i = 0; i < n_trees; ++i)
    //   {
    for i in 0..n_trees {
        //     pars_trees[i] = corax_utree_create_parsimony(n_taxa,
        //                                                  msa->length,
        //                                                  msa->label,
        //                                                  msa->sequence,
        //                                                  NULL, /* site weights */
        //                                                  corax_map_nt,
        //                                                  4,
        //                                                  0,
        //                                                  i, /* seed */
        //                                                  &score);
        pars_trees[i] = unsafe {
            corax_utree_create_parsimony(
                n_taxa,
                unsafe { *msa }.length,
                unsafe { *msa }.label,
                unsafe { *msa }.sequence,
                std::ptr::null(),
                todo!("corax_map_nt"),
                4,
                0,
                i, // seed
                &mut score,
            )
        };
        //   }
    }
    //   for (int i = 0; i < n_taxa; ++i)
    //   {
    for i in 0..n_taxa {
        //     labelToId.insert({std::string(pars_trees[0]->nodes[i]->label), i});
        let node = unsafe { *(*pars_trees[0]).nodes.offset(i) };
        label_to_id.insert(unsafe { CString::from_raw(unsafe { *node }.label) }, i)
        //   }
    }
    //   for (corax_utree_t * tree: pars_trees)
    //   {
    for tree in pars_trees.iter().copied() {
        //     for (int i = 0; i < n_taxa; ++i)
        //     {
        for i in 0..n_taxa {
            // auto leaf        = tree->nodes[i];
            let leaf = unsafe { *(*tree).nodes.offset(i) };
            // auto id          = labelToId.at(std::string(leaf->label));
            let label = unsafe { CString::from_raw((*leaf).label) };
            let Some(id) = *label_to_id.get(&label) else {
                panic!("label {label} not found in label_to_id map")
            };
            // leaf->node_index = leaf->clv_index = id;
            unsafe { *leaf }.node_index = unsafe { *leaf }.clv_index = id;
            //     }
        }
        //   }
    }
    //   for (int i = 0; i < n_trees; ++i)
    //   {
    for i in 0..n_trees {
        //     splits[i] = corax_utree_split_create(pars_trees[i]->vroot, n_taxa, nullptr);
        splits[i] =
            unsafe { corax_utree_split_create((*pars_trees[i]).vroot, n_taxa, std::ptr::null()) };
        //   }
    }
    //   for (int i = 0; i < n_trees; ++i)
    //   {
    for i in 0..n_trees {
        //     corax_utree_destroy(pars_trees[i], NULL);
        unsafe { corax_utree_destroy(pars_trees[i], std::ptr::null()) };
        //   }
    }
    //   return splits;
    splits
    // }
}
// std::tuple<int, double> get_num_unique_and_rel_rfdist(std::vector<corax_split_t *> splits, int n_taxa)
// {
fn get_num_unique_and_rel_rfdist(splits: Vec<*const corax_split_t>, n_taxa: i32) -> (i32, f64) {
    /**
     * Computes the average relative RF distance and the number of unique topologies for the given splits.
     */
    //   int num_trees = splits.size();
    let num_trees = splits.len();
    //   int num_unique = 1;
    let mut num_unique = 1i32;
    //   double avg_rrf = 0.0;
    let mut avg_rrf = 0.0f64;
    //   double max_rf = (double)2 * (n_taxa - 3);
    let max_rf = (2 * (n_taxa - 3)) as f64;
    //   size_t num_pairs =  0;
    let mut num_pairs = 0usize;
    //
    //   for (int i = 0; i < num_trees - 1; ++i)
    //   {
    for i in 0..(num_trees - 1) {
        // bool uniq = true;
        let mut uniq = true;
        // for (int j = i + 1; j < num_trees; ++j)
        //
        for j in (i + 1)..num_trees {
            // double rf = corax_utree_split_rf_distance(splits[i], splits[j], n_taxa);
            let rf: f64 = unsafe { corax_utree_split_rf_distance(splits[i], splits[j], n_taxa) };
            // avg_rrf += ((double)rf) / max_rf;
            avg_rrf += rf as f64 / max_rf;
            // num_pairs++;
            num_pairs += 1;
            // uniq &= (rf > 0);
            uniq &= (rf > 0);
            //  }
        }
        //     if (uniq)
        if uniq {
            //       num_unique++;
            num_unique += 1;
        }
        //   }
    }
    //   avg_rrf /= num_pairs;
    avg_rrf /= num_pairs;
    //   return {num_unique, avg_rrf};
    (num_unique, avg_rrf)
    // }
}

// int main(int argc, char *argv[])
// {
//   const char *filename = "path/to/msa.phy";
fn predict_difficulty(filename: &str) -> f64 {
    let c_filename = CString::new(filename).expect("failed to convert filename to a cstring");
    /*
     * make sure to update this function call based on your MSA:
     * - for MSAs in phylip format use corax_phylip_load and set the interleaved flag accordingly
     * - for MSAs in fasta format use corax_fasta_load
     */
    //   corax_msa_t *msa = corax_phylip_load(filename, CORAX_FALSE);
    // TODO: corax false
    let msa = unsafe { corax_phylip_load(c_filename.as_ptr(), 0) };
    if msa.is_null() {
        eprintln!("loaded msa is null");
        return;
    }
    //   size_t _num_trees = 100;
    let _num_trees: usize = 100;
    //   int n_taxa = msa->count;
    let n_taxa: i32 = unsafe { *msa }.count;
    // std::vector<corax_split_t *> splits = get_pars_splits(msa, _num_trees);
    let splits = get_pars_splits(msa, _num_trees);
    // int num_unique;
    // double avg_rrf;
    // std::tie(num_unique, avg_rrf) = get_num_unique_and_rel_rfdist(splits, n_taxa);
    let (num_unique, avg_rrf) = get_num_unique_and_rel_rfdist(splits, n_taxa);
    // corax_msa_features * features = corax_msa_compute_features(msa, 4, corax_map_nt);
    let features = unsafe { corax_msa_compute_features(msa, 4, todo!("corax_map_nt")) };
    // double out_pred = corax_msa_predict_difficulty(features, avg_rrf, num_unique / _num_trees);
    let out_pred: f64 =
        unsafe { corax_msa_predict_difficulty(features, avg_rrf, num_unique / _num_trees) };
    // out_pred = round(out_pred * 100.0) / 100.0;
    let out_pred = (out_pred * 100.0).round() / 100.0;
    // std::cout << "The predicted difficulty for MSA " << filename << " is: " << out_pred << "\n";

    // corax_msa_destroy(msa);
    unsafe { corax_msa_destroy(msa) };
    // free(features);
    unsafe { libc::free(features) };
    // for (auto s : splits) corax_utree_split_destroy(s);
    splits.into_iter().for_each(|s| unsafe {
        corax_utree_split_destroy(s);
    });
    // }
    out_pred
}
