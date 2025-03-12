#!/bin/env -S cargo +nightly -Zscript

// ---cargo
// [package]
// edition = "2024"
// ---
use std::ops::RangeInclusive;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Alphabet {
    Nucleotide,
    Protein,
}
#[derive(Clone, Debug)]
struct DataSetDimensions {
    n_seqs: usize,
    seq_len: RangeInclusive<usize>,
    alphabet: Alphabet,
}

#[derive(Clone, Debug)]
struct DataSet {
    path: &'static str,
    dimensions: DataSetDimensions,
    difficulty: Option<f64>,
}

fn filter_fn(dataset: &DataSet) -> bool {
    dataset.dimensions.alphabet == Alphabet::Nucleotide
        && dataset.dimensions.seq_len.start() == dataset.dimensions.seq_len.end()
}
fn sort_key(dataset: &DataSet) -> (usize, usize) {
    (
        dataset.dimensions.n_seqs,
        *dataset.dimensions.seq_len.start(),
    )
}

fn main() {
    use Alphabet::*;
    let mut datasets = [
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_DNA2_unaligned.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 4,
                seq_len: 1..=4,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_DNA3_2seqs.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 2,
                seq_len: 2..=3,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_DNA4_unaligned.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 4,
                seq_len: 1..=4,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_diff_branch_lengths_1.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 2,
                seq_len: 2..=4,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_diff_branch_lengths_2.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 4,
                seq_len: 2..=5,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_diff_branch_lengths_3.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 4,
                seq_len: 2..=5,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_empty.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 0,
                seq_len: 0..=0,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_fill_matrix_gap_adjustment_1.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 4,
                seq_len: 2..=4,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_fill_matrix_gap_adjustment_2.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 4,
                seq_len: 1..=3,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_fill_matrix_gap_adjustment_3.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 4,
                seq_len: 2..=5,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/JATI_data/sequences_long.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 11,
                seq_len: 241..=246,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/XiD4/Cluster5461.nt.aln",
            dimensions: DataSetDimensions {
                n_seqs: 36,
                seq_len: 312..=312,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/XiD4/Cluster2421.nt.aln",
            dimensions: DataSetDimensions {
                n_seqs: 40,
                seq_len: 687..=687,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/XiD4/Cluster3339.nt.aln",
            dimensions: DataSetDimensions {
                n_seqs: 33,
                seq_len: 375..=375,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_9604.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 262..=262,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_11137.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 340..=340,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13408.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 46,
                seq_len: 164..=164,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_11225.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 364..=364,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_12890.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 674..=674,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13148.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 43,
                seq_len: 220..=220,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_8414.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 372..=372,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_10262.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 424..=424,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_11660.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 43,
                seq_len: 306..=306,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_9529.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 366..=366,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13974.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 45,
                seq_len: 296..=296,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13895.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 46,
                seq_len: 946..=946,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_6730.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 45,
                seq_len: 754..=754,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_14594.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 168..=168,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13079.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 262..=262,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_14261.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 174..=174,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13235.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 264..=264,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13561.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 144..=144,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_12064.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 104..=104,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13931.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 43,
                seq_len: 128..=128,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_2021.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 376..=376,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_3118.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 332..=332,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_14622.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 45,
                seq_len: 80..=80,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_10092.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 46,
                seq_len: 544..=544,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13111.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 84..=84,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_12476.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 108..=108,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_10611.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 46,
                seq_len: 104..=104,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_9374.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 46,
                seq_len: 518..=518,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_11268.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 938..=938,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_11540.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 696..=696,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13585.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 628..=628,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_10946.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 704..=704,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_7674.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 45,
                seq_len: 116..=116,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_8917.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 146..=146,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13783.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 214..=214,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_11923.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 236..=236,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13633.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 66..=66,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_7184.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 46,
                seq_len: 288..=288,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_11255.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 492..=492,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_11506.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 46,
                seq_len: 662..=662,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_12933.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 46,
                seq_len: 514..=514,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_14577.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 226..=226,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_8380.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 312..=312,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_6446.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 44,
                seq_len: 144..=144,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_8007.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 268..=268,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13773.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 118..=118,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_10128.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 42,
                seq_len: 1634..=1634,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_13038.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 46,
                seq_len: 446..=446,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_9839.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 268..=268,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_10732.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 47,
                seq_len: 152..=152,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path:
                "/mnt/shared/datasets/Xiaofan_Zhou/single-gene_alignments/JarvD5a/exon_10090.fasta",
            dimensions: DataSetDimensions {
                n_seqs: 48,
                seq_len: 490..=490,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/ASTER/genetrees.tre_1.fas",
            dimensions: DataSetDimensions {
                n_seqs: 26,
                seq_len: 500..=500,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/ASTER/genetrees.tre_2.fas",
            dimensions: DataSetDimensions {
                n_seqs: 26,
                seq_len: 500..=500,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
        DataSet {
            path: "/mnt/shared/datasets/ASTER/genetrees.tre_3.fas",
            dimensions: DataSetDimensions {
                n_seqs: 26,
                seq_len: 500..=500,
                alphabet: Nucleotide,
            },
            difficulty: None,
        },
    ]
    .into_iter()
    .filter(filter_fn)
    .collect::<Vec<_>>();
    datasets.sort_unstable_by_key(sort_key);
    println!("{datasets:#?}");
}
