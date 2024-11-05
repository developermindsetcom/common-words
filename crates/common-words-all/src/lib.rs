pub mod generated;
pub mod types;

use generated::*;
pub use types::*;

pub fn get_array(language: Language, size: NgramSize) -> Vec<Record<'static>> {
    match (language, size) {
        #[cfg(all(feature = "chinese", feature = "one"))]
        (Language::Chinese, NgramSize::One) => cwa_1grams_chinese_simplified.to_vec(),
        #[cfg(all(feature = "chinese", feature = "two"))]
        (Language::Chinese, NgramSize::Two) => cwa_2grams_chinese_simplified.to_vec(),
        #[cfg(all(feature = "chinese", feature = "three"))]
        (Language::Chinese, NgramSize::Three) => cwa_3grams_chinese_simplified.to_vec(),
        #[cfg(all(feature = "chinese", feature = "four"))]
        (Language::Chinese, NgramSize::Four) => cwa_4grams_chinese_simplified.to_vec(),
        #[cfg(all(feature = "chinese", feature = "five"))]
        (Language::Chinese, NgramSize::Five) => cwa_5grams_chinese_simplified.to_vec(),

        #[cfg(all(feature = "english", feature = "one"))]
        (Language::English, NgramSize::One) => cwa_1grams_english.to_vec(),
        #[cfg(all(feature = "english", feature = "two"))]
        (Language::English, NgramSize::Two) => cwa_2grams_english.to_vec(),
        #[cfg(all(feature = "english", feature = "three"))]
        (Language::English, NgramSize::Three) => cwa_3grams_english.to_vec(),
        #[cfg(all(feature = "english", feature = "four"))]
        (Language::English, NgramSize::Four) => cwa_4grams_english.to_vec(),
        #[cfg(all(feature = "english", feature = "five"))]
        (Language::English, NgramSize::Five) => cwa_5grams_english.to_vec(),
        
        #[cfg(all(feature = "french", feature = "one"))]
        (Language::French, NgramSize::One) => cwa_1grams_french.to_vec(),
        #[cfg(all(feature = "french", feature = "two"))]
        (Language::French, NgramSize::Two) => cwa_2grams_french.to_vec(),
        #[cfg(all(feature = "french", feature = "three"))]
        (Language::French, NgramSize::Three) => cwa_3grams_french.to_vec(),
        #[cfg(all(feature = "french", feature = "four"))]
        (Language::French, NgramSize::Four) => cwa_4grams_french.to_vec(),
        #[cfg(all(feature = "french", feature = "five"))]
        (Language::French, NgramSize::Five) => cwa_5grams_french.to_vec(),
        
        #[cfg(all(feature = "german", feature = "one"))]
        (Language::German, NgramSize::One) => cwa_1grams_german.to_vec(),
        #[cfg(all(feature = "german", feature = "two"))]
        (Language::German, NgramSize::Two) => cwa_2grams_german.to_vec(),
        #[cfg(all(feature = "german", feature = "three"))]
        (Language::German, NgramSize::Three) => cwa_3grams_german.to_vec(),
        #[cfg(all(feature = "german", feature = "four"))]
        (Language::German, NgramSize::Four) => cwa_4grams_german.to_vec(),
        #[cfg(all(feature = "german", feature = "five"))]
        (Language::German, NgramSize::Five) => cwa_5grams_german.to_vec(),
        
        #[cfg(all(feature = "hebrew", feature = "one"))]
        (Language::Hebrew, NgramSize::One) => cwa_1grams_hebrew.to_vec(),
        #[cfg(all(feature = "hebrew", feature = "two"))]
        (Language::Hebrew, NgramSize::Two) => cwa_2grams_hebrew.to_vec(),
        #[cfg(all(feature = "hebrew", feature = "three"))]
        (Language::Hebrew, NgramSize::Three) => cwa_3grams_hebrew.to_vec(),
        #[cfg(all(feature = "hebrew", feature = "four"))]
        (Language::Hebrew, NgramSize::Four) => cwa_4grams_hebrew.to_vec(),
        #[cfg(all(feature = "hebrew", feature = "five"))]
        (Language::Hebrew, NgramSize::Five) => cwa_5grams_hebrew.to_vec(),
        
        #[cfg(all(feature = "italian", feature = "one"))]
        (Language::Italian, NgramSize::One) => cwa_1grams_italian.to_vec(),
        #[cfg(all(feature = "italian", feature = "two"))]
        (Language::Italian, NgramSize::Two) => cwa_2grams_italian.to_vec(),
        #[cfg(all(feature = "italian", feature = "three"))]
        (Language::Italian, NgramSize::Three) => cwa_3grams_italian.to_vec(),
        #[cfg(all(feature = "italian", feature = "four"))]
        (Language::Italian, NgramSize::Four) => cwa_4grams_italian.to_vec(),
        #[cfg(all(feature = "italian", feature = "five"))]
        (Language::Italian, NgramSize::Five) => cwa_5grams_italian.to_vec(),
        
        #[cfg(all(feature = "russian", feature = "one"))]
        (Language::Russian, NgramSize::One) => cwa_1grams_russian.to_vec(),
        #[cfg(all(feature = "russian", feature = "two"))]
        (Language::Russian, NgramSize::Two) => cwa_2grams_russian.to_vec(),
        #[cfg(all(feature = "russian", feature = "three"))]
        (Language::Russian, NgramSize::Three) => cwa_3grams_russian.to_vec(),
        #[cfg(all(feature = "russian", feature = "four"))]
        (Language::Russian, NgramSize::Four) => cwa_4grams_russian.to_vec(),
        #[cfg(all(feature = "russian", feature = "five"))]
        (Language::Russian, NgramSize::Five) => cwa_5grams_russian.to_vec(),
        
        #[cfg(all(feature = "spanish", feature = "one"))]
        (Language::Spanish, NgramSize::One) => cwa_1grams_spanish.to_vec(),
        #[cfg(all(feature = "spanish", feature = "two"))]
        (Language::Spanish, NgramSize::Two) => cwa_2grams_spanish.to_vec(),
        #[cfg(all(feature = "spanish", feature = "three"))]
        (Language::Spanish, NgramSize::Three) => cwa_3grams_spanish.to_vec(),
        #[cfg(all(feature = "spanish", feature = "four"))]
        (Language::Spanish, NgramSize::Four) => cwa_4grams_spanish.to_vec(),
        #[cfg(all(feature = "spanish", feature = "five"))]
        (Language::Spanish, NgramSize::Five) => cwa_5grams_spanish.to_vec(),
    }
}

pub fn get_top(language: Language, top: usize, size: NgramSize) -> Vec<&'static str> {
    let array = get_array(language, size);
    let top_ngrams = array.iter().take(top).into_iter().map(|record|record.ngram).collect::<Vec<&str>>();

    top_ngrams
}
