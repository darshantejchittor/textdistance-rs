mod levenshtein;
mod hamming;
mod jaccard;
mod sorensen;
mod jaro;
mod jaro_winkler;
mod cosine;

fn main() {
    let distance = levenshtein::levenshtein("kitten", "sitting");

    println!(
        "Distance between 'kitten' and 'sitting' = {}",
        distance
    );
}