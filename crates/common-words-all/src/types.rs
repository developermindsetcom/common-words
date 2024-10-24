#[allow(dead_code)]

#[derive(Clone)]
pub struct Record<'a> {
    pub ngram: &'a str,
    pub freq: f64,
    pub cumshare: Option<f64>,
    pub en: Option<&'a str>,
}

pub enum Language {
    #[cfg(feature = "chinese")]
    Chinese,
    #[cfg(feature = "english")]
    English,
    #[cfg(feature = "french")]
    French,
    #[cfg(feature = "german")]
    German,
    #[cfg(feature = "hebrew")]
    Hebrew,
    #[cfg(feature = "italian")]
    Italian,
    #[cfg(feature = "russian")]
    Russian,
    #[cfg(feature = "spanish")]
    Spanish,
}

pub enum NgramSize {
    #[cfg(feature = "one")]
    One,
    #[cfg(feature = "two")]
    Two,
    #[cfg(feature = "three")]
    Three,
    #[cfg(feature = "four")]
    Four,
    #[cfg(feature = "five")]
    Five,
}