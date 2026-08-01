mod levenshtein;
mod hamming;
mod jaccard;
mod sorensen;
mod jaro;
mod jaro_winkler;
mod cosine;
mod damerau;
mod lcs;
mod needleman_wunsch;
mod longest_common_substring;
mod overlap;
mod tversky;
mod bag_distance;



fn main() {
    let distance = levenshtein::levenshtein("kitten", "sitting");

    println!(
        "Distance between 'kitten' and 'sitting' = {}",
        distance
    );
}