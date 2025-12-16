use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    /// Map of irregular past participles (200+ common verbs)
    pub static ref IRREGULAR_PAST_PARTICIPLES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        
        // Most common irregular verbs
        set.extend(&[
            "been", "done", "gone", "seen", "known", "given", "taken", "made",
            "come", "become", "written", "spoken", "broken", "chosen", "driven",
            "eaten", "fallen", "forgotten", "forgiven", "frozen", "gotten",
            "hidden", "ridden", "risen", "shaken", "shown", "stolen", "sworn",
            "torn", "thrown", "worn", "beaten", "bitten", "blown", "drawn",
            "flown", "grown", "withdrawn",
        ]);
        
        // Additional irregular forms
        set.extend(&[
            "begun", "drunk", "rung", "shrunk", "sunk", "sprung", "stunk",
            "sung", "swum", "spun", "won", "hung", "struck", "stuck",
            "swung", "slung", "clung", "flung", "stung", "strung", "wrung",
        ]);
        
        // Verbs with -en endings
        set.extend(&[
            "arisen", "awoken", "borne", "beaten", "begotten", "bidden",
            "bitten", "broken", "chosen", "driven", "eaten", "fallen",
            "forbidden", "forgotten", "forgiven", "forsaken", "frozen",
            "given", "hewn", "hidden", "lain", "laden", "mistaken",
            "proven", "ridden", "risen", "shaken", "shown", "spoken",
            "stolen", "stricken", "stridden", "striven", "sworn", "taken",
            "thriven", "thrown", "trodden", "waken", "waxen", "woven",
            "written",
        ]);
        
        // Common -ed irregular forms that could be confused
        set.extend(&[
            "said", "paid", "laid", "made", "heard", "sold", "told",
            "held", "left", "kept", "slept", "wept", "swept", "felt",
            "dealt", "meant", "sent", "spent", "bent", "lent", "built",
            "burnt", "learnt", "spelt", "spoilt", "dwelt",
        ]);
        
        // Less common but important
        set.extend(&[
            "abode", "awoke", "bore", "bound", "bred", "brought", "built",
            "burst", "bought", "cast", "caught", "crept", "dealt", "dug",
            "fed", "felt", "fought", "found", "fled", "flung", "forbade",
            "forecast", "forgot", "forsook", "fought", "froze", "got",
            "ground", "grew", "heard", "hid", "hit", "hurt", "kept",
            "knelt", "knew", "laid", "led", "left", "lent", "let",
            "lit", "lost", "meant", "met", "overcome", "overthrown",
            "paid", "put", "quit", "read", "rid", "rang", "ran",
            "said", "saw", "sought", "sent", "set", "sewed", "shaken",
            "shed", "shone", "shot", "shrunk", "shut", "slain", "slept",
            "slid", "slit", "sown", "sped", "spelt", "spent", "spilt",
            "split", "spread", "stood", "strewn", "strode", "strove",
            "stuck", "stung", "stunk", "swept", "swelled", "swore",
            "swum", "swung", "taught", "thought", "threw", "thrust",
            "told", "took", "tore", "underwent", "understood", "undone",
            "upset", "woken", "wore", "wound", "wove", "wrought",
        ]);
        
        set
    };

    /// Words that end in -ed/-en but are adjectives, not passive voice
    pub static ref ADJECTIVE_EXCEPTIONS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.extend(&[
            "tired", "excited", "interested", "bored", "confused", "worried",
            "scared", "frightened", "amazed", "surprised", "shocked", "pleased",
            "satisfied", "disappointed", "frustrated", "embarrassed", "ashamed",
            "annoyed", "delighted", "thrilled", "stunned", "overwhelmed",
            "talented", "gifted", "blessed", "cursed", "aged", "beloved",
            "learned", "skilled", "experienced", "advanced", "supposed",
            "alleged", "concerned", "determined", "devoted", "distinguished",
            "educated", "enlightened", "equipped", "established", "esteemed",
            "experienced", "extended", "informed", "inspired", "involved",
            "limited", "marked", "mixed", "organized", "packed", "prepared",
            "pronounced", "qualified", "refined", "relaxed", "relieved",
            "renowned", "reserved", "respected", "retired", "scared",
            "skilled", "sophisticated", "trained", "troubled", "united",
            "unmarried", "used", "varied", "wasted", "wicked", "worried",
            "wounded",
        ]);
        set
    };

    /// Linking verbs that might be confused with passive voice auxiliaries
    pub static ref LINKING_VERBS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.extend(&[
            "seem", "seems", "seemed", "seeming",
            "appear", "appears", "appeared", "appearing",
            "become", "becomes", "became", "becoming",
            "feel", "feels", "felt", "feeling",
            "look", "looks", "looked", "looking",
            "remain", "remains", "remained", "remaining",
            "stay", "stays", "stayed", "staying",
            "sound", "sounds", "sounded", "sounding",
            "smell", "smells", "smelled", "smelling",
            "taste", "tastes", "tasted", "tasting",
        ]);
        set
    };
}

/// Check if a word is an irregular past participle
pub fn is_irregular_past_participle(word: &str) -> bool {
    IRREGULAR_PAST_PARTICIPLES.contains(word.to_lowercase().as_str())
}

/// Check if a word is likely an adjective exception
pub fn is_adjective_exception(word: &str) -> bool {
    ADJECTIVE_EXCEPTIONS.contains(word.to_lowercase().as_str())
}

/// Check if a word is a linking verb
pub fn is_linking_verb(word: &str) -> bool {
    LINKING_VERBS.contains(word.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irregular_participles() {
        assert!(is_irregular_past_participle("written"));
        assert!(is_irregular_past_participle("done"));
        assert!(is_irregular_past_participle("seen"));
        assert!(is_irregular_past_participle("broken"));
        assert!(!is_irregular_past_participle("walked"));
    }

    #[test]
    fn test_adjective_exceptions() {
        assert!(is_adjective_exception("tired"));
        assert!(is_adjective_exception("excited"));
        assert!(is_adjective_exception("interested"));
        assert!(!is_adjective_exception("completed"));
    }

    #[test]
    fn test_linking_verbs() {
        assert!(is_linking_verb("seems"));
        assert!(is_linking_verb("appears"));
        assert!(!is_linking_verb("runs"));
    }
}
