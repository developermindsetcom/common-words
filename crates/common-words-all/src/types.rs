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

impl Default for Language {
    fn default() -> Self {
        #[cfg(feature = "chinese")]
        let lang = Self::Chinese;
        #[cfg(feature = "english")]
        let lang = Self::English;
        #[cfg(feature = "french")]
        let lang = Self::French;
        #[cfg(feature = "german")]
        let lang = Self::German;
        #[cfg(feature = "hebrew")]
        let lang = Self::Hebrew;
        #[cfg(feature = "italian")]
        let lang = Self::Italian;
        #[cfg(feature = "russian")]
        let lang = Self::Russian;
        #[cfg(feature = "spanish")]
        let lang = Self::Spanish;

        lang
    }
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

impl Default for NgramSize {
    fn default() -> Self {
        #[allow(unused_variables)]
        #[cfg(feature = "one")]
        let size = Self::One;
        
        #[allow(unused_variables)]
        #[cfg(feature = "two")]
        let size = Self::Two;
        
        #[allow(unused_variables)]
        #[cfg(feature = "three")]
        let size = Self::Three;
        
        #[allow(unused_variables)]
        #[cfg(feature = "four")]
        let size = Self::Four;
        
        #[allow(unused_variables)]
        #[cfg(feature = "five")]
        let size = Self::Five;

        size
    }
}