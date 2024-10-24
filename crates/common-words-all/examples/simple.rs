
use common_words_all::*;

fn main() {
    let max = 10;
    let top = get_top(Language::English, max, NgramSize::One);
    println!("Top {max} ngrams:");
    for (index, ngram) in top.into_iter().enumerate() {
        println!("{index}\t{ngram}");
    }
}