pub mod abbreviations;
pub mod irregular_verbs;
pub mod syllable_dict;

pub use abbreviations::{is_abbreviation, ends_with_abbreviation, ABBREVIATIONS};
pub use irregular_verbs::{
    is_irregular_past_participle, is_adjective_exception, is_linking_verb,
    IRREGULAR_PAST_PARTICIPLES, ADJECTIVE_EXCEPTIONS, LINKING_VERBS,
};
pub use syllable_dict::{count_syllables, lookup_syllables, estimate_syllables, SYLLABLE_DICT};
