// #include <iostream>
// #include <vector>
// #include <map>
// #include <tuple>
// #include <math.h>
// #include "corax/corax.h"
// #include "difficulty.h"

use corax_sys::{
    corax_fasta_load, corax_map_aa, corax_map_nt, corax_msa_destroy, corax_msa_t, corax_split_t,
    corax_utree_create_parsimony, corax_utree_destroy, corax_utree_split_create,
    corax_utree_split_destroy, corax_utree_split_rf_distance, corax_utree_t,
};
use corax_sys::{corax_msa_compute_features, corax_msa_predict_difficulty};
use std::ffi::{CStr, c_void};
use std::{collections::HashMap, ffi::CString};

// std::vector<corax_split_t *> get_pars_splits(corax_msa_t *msa, int n_trees)
// {
/**
 * Infers n_trees parsimony trees for the given MSA using Coraxlib, creates and returns the splits.
 */
fn get_pars_splits(msa: *const corax_msa_t, n_trees: u32) -> Vec<*mut corax_split_t> {
    // SAFETY: i have no idea why they would use i32 for n_trees. the vector allocation below would
    // immediately fail for negative numbers. it is also only ever called with size_t which is
    // unsigned
    //   std::vector<corax_utree_t *> pars_trees(n_trees);
    let mut pars_trees: Vec<*mut corax_utree_t> = vec![std::ptr::null_mut(); n_trees as usize];
    //   std::vector<corax_split_t *> splits(n_trees);
    let mut splits: Vec<*mut corax_split_t> = vec![std::ptr::null_mut(); n_trees as usize];
    //   std::map<std::string, unsigned int> labelToId;
    let mut label_to_id: HashMap<&CStr, u32> = Default::default();
    //   unsigned int score;
    // NOTE: this was previously undefined behavior but rust forces initializaiton
    // before use
    let mut score: u32 = 0;
    //   int n_taxa = msa->count;
    let n_taxa: i32 = unsafe { *msa }.count;
    // all use cases below require a unsiged n_taxa so passing a signed is classic old undefined
    // behavior in C
    let unsigned_n_taxa: u32 = n_taxa
        .try_into()
        .expect("negative n_taxa indicates an error");
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
        pars_trees[i as usize] = unsafe {
            corax_utree_create_parsimony(
                unsigned_n_taxa,
                (*msa).length.try_into().expect("msa length doesnt fit u32"),
                (*msa).label as *const *const i8,
                (*msa).sequence as *const *const i8,
                std::ptr::null(),
                corax_map_aa.as_ptr(),
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
    for i in 0..unsigned_n_taxa {
        //     labelToId.insert({std::string(pars_trees[0]->nodes[i]->label), i});
        let node = unsafe { *(*pars_trees[0]).nodes.offset(i as isize) };
        label_to_id.insert(unsafe { CStr::from_ptr((*node).label) }, i);
        //   }
    }
    dbg!("built label map");
    //   for (corax_utree_t * tree: pars_trees)
    //   {
    for tree in pars_trees.iter().copied() {
        //     for (int i = 0; i < n_taxa; ++i)
        //     {
        for i in 0..unsigned_n_taxa {
            // auto leaf        = tree->nodes[i];
            let leaf = unsafe { *(*tree).nodes.offset(i as isize) };
            // auto id          = labelToId.at(std::string(leaf->label));
            let label = unsafe { CStr::from_ptr((*leaf).label) };
            let Some(id) = label_to_id.get(label).copied() else {
                panic!(
                    "label {} not found in label_to_id map",
                    label.to_string_lossy()
                )
            };
            // leaf->node_index = leaf->clv_index = id;
            unsafe { *leaf }.clv_index = id;
            unsafe { *leaf }.node_index = id;
            //     }
        }
        //   }
    }
    dbg!("assigned tree nodes to ids according to label");
    //   for (int i = 0; i < n_trees; ++i)
    //   {
    for i in 0..n_trees {
        //     splits[i] = corax_utree_split_create(pars_trees[i]->vroot, n_taxa, nullptr);
        splits[i as usize] = unsafe {
            corax_utree_split_create(
                (*pars_trees[i as usize]).vroot,
                unsigned_n_taxa,
                std::ptr::null_mut(),
            )
        };
        //   }
    }
    dbg!("created pars_tree splits");
    //   for (int i = 0; i < n_trees; ++i)
    //   {
    for i in 0..n_trees {
        //     corax_utree_destroy(pars_trees[i], NULL);
        unsafe { corax_utree_destroy(pars_trees[i as usize], None) };
        //   }
    }
    dbg!("destroyed all pars_trees");
    //   return splits;
    splits
    // }
}
// std::tuple<int, double> get_num_unique_and_rel_rfdist(std::vector<corax_split_t *> splits, int n_taxa)
// {
/**
 * Computes the average relative RF distance and the number of unique topologies for the given splits.
 */
fn get_num_unique_and_rel_rfdist(splits: &[*mut corax_split_t], n_taxa: i32) -> (u32, f64) {
    let unsigned_n_taxa: u32 = n_taxa.try_into().expect("negative n_taxa dont make sense");
    //   int num_trees = splits.size();
    let num_trees = splits.len();
    //   int num_unique = 1;
    let mut num_unique = 1u32;
    //   double avg_rrf = 0.0;
    let mut avg_rrf = 0.0f64;
    //   double max_rf = (double)2 * (n_taxa - 3);
    // TODO: should we catch n_taxa below 3?
    // seems like a negative value here would mess with the avg
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
            let rf: f64 =
                unsafe { corax_utree_split_rf_distance(splits[i], splits[j], unsigned_n_taxa) }
                    .into();
            // avg_rrf += ((double)rf) / max_rf;
            dbg!(rf);
            avg_rrf += rf as f64 / max_rf;
            dbg!(avg_rrf);
            // num_pairs++;
            num_pairs += 1;
            // uniq &= (rf > 0);
            uniq &= rf > 0.;
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
    avg_rrf /= num_pairs as f64;
    //   return {num_unique, avg_rrf};
    (num_unique, avg_rrf)
    // }
}

// int main(int argc, char *argv[])
// {
//   const char *filename = "path/to/msa.phy";
pub fn predict_difficulty(filename: &str) -> f64 {
    dbg!(filename);
    let c_filename = CString::new(filename).expect("failed to convert filename to a cstring");
    /*
     * make sure to update this function call based on your MSA:
     * - for MSAs in phylip format use corax_phylip_load and set the interleaved flag accordingly
     * - for MSAs in fasta format use corax_fasta_load
     */
    //   corax_msa_t *msa = corax_phylip_load(filename, CORAX_FALSE);
    let msa = unsafe { corax_fasta_load(c_filename.as_ptr()) };
    if msa.is_null() {
        panic!("loaded msa is null");
    }
    //   size_t _num_trees = 100;
    let _num_trees: u32 = 100;
    //   int n_taxa = msa->count;
    let n_taxa: i32 = unsafe { *msa }.count;
    // std::vector<corax_split_t *> splits = get_pars_splits(msa, _num_trees);
    let splits = get_pars_splits(msa, _num_trees);
    println!("got splits");
    // int num_unique;
    // double avg_rrf;
    // std::tie(num_unique, avg_rrf) = get_num_unique_and_rel_rfdist(splits, n_taxa);
    // note: copy of vec is unnecessary since we only read. Also: rust readonly ref makes
    // this safe since if the function could modify the pointers we would not free them later
    // with corax_utree_split_destroy
    let (num_unique, avg_rrf) = get_num_unique_and_rel_rfdist(&splits, n_taxa);
    println!("got num_unique {num_unique} and avg_rrf {avg_rrf}");
    // corax_msa_features * features = corax_msa_compute_features(msa, 4, corax_map_nt);
    let features = unsafe { corax_msa_compute_features(msa, 4, corax_map_aa.as_ptr()) };
    println!("computed msa features");
    // double out_pred = corax_msa_predict_difficulty(features, avg_rrf, num_unique / _num_trees);
    let out_pred: f64 = unsafe {
        corax_msa_predict_difficulty(
            features,
            avg_rrf,
            num_unique.overflowing_div(_num_trees).0 as f64,
        )
    };
    println!("predicted difficulty {out_pred}");
    // out_pred = round(out_pred * 100.0) / 100.0;
    let out_pred = (out_pred * 100.0).round() / 100.0;
    // std::cout << "The predicted difficulty for MSA " << filename << " is: " << out_pred << "\n";

    // corax_msa_destroy(msa);
    unsafe { corax_msa_destroy(msa) };
    // free(features);
    unsafe { libc::free(features as *mut c_void) };
    // for (auto s : splits) corax_utree_split_destroy(s);
    splits.into_iter().for_each(|s| unsafe {
        corax_utree_split_destroy(s);
    });
    // }
    out_pred
}
