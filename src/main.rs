mod levenshtein;
mod hamming;
mod jaccard;
mod sorensen;
mod jaro;

fn main() {
    let distance = levenshtein::levenshtein("kitten", "sitting");

    println!(
        "Distance between 'kitten' and 'sitting' = {}",
        distance
    );
}