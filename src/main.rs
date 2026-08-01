mod levenshtein;
mod hamming;
mod jaccard;

fn main() {
    let distance = levenshtein::levenshtein("kitten", "sitting");

    println!(
        "Distance between 'kitten' and 'sitting' = {}",
        distance
    );
}