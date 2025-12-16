use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    pub static ref GLUE_WORDS: HashSet<&'static str> = {
        let words = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
            "of", "with", "by", "from", "up", "about", "into", "through", "during",
            "that", "this", "these", "those", "it", "its", "is", "are", "was", "were",
            "be", "been", "being", "have", "has", "had", "do", "does", "did", "will",
            "would", "should", "could", "may", "might", "must", "can", "which", "who",
            "when", "where", "why", "how", "if", "than", "then", "as", "so"
        ];
        words.iter().copied().collect()
    };

    pub static ref TRANSITION_WORDS: HashSet<&'static str> = {
        let words = [
            "however", "therefore", "thus", "consequently", "nevertheless", "moreover",
            "furthermore", "additionally", "meanwhile", "instead", "otherwise",
            "similarly", "likewise", "conversely", "nonetheless", "hence", "accordingly",
            "subsequently", "indeed", "specifically", "particularly", "especially"
        ];
        words.iter().copied().collect()
    };

    pub static ref TRANSITION_PHRASES: HashSet<&'static str> = {
        let phrases = [
            "for example", "for instance", "in addition", "in contrast", "on the other hand",
            "as a result", "in conclusion", "in summary", "to summarize", "finally"
        ];
        phrases.iter().copied().collect()
    };

    pub static ref VAGUE_WORDS: HashSet<&'static str> = {
        let words = [
            "thing", "things", "stuff", "nice", "good", "bad", "great", "terrible",
            "amazing", "awesome", "interesting", "very", "really", "quite", "rather",
            "somewhat", "pretty", "fairly"
        ];
        words.iter().copied().collect()
    };

    pub static ref VAGUE_PHRASES: HashSet<&'static str> = {
        let phrases = ["kind of", "sort of", "a bit"];
        phrases.iter().copied().collect()
    };

    pub static ref BUSINESS_JARGON: HashSet<&'static str> = {
        let jargon = [
            "synergy", "leverage", "paradigm", "disrupt", "innovative", "streamline",
            "optimization", "scalable", "bandwidth", "win-win", "game changer",
            "best practice", "core competency", "value-added", "going forward",
            "deep dive", "reach out"
        ];
        jargon.iter().copied().collect()
    };

    pub static ref BUSINESS_JARGON_PHRASES: HashSet<&'static str> = {
        let phrases = [
            "circle back", "touch base", "low-hanging fruit", "move the needle",
            "drink the kool-aid", "boil the ocean", "think outside the box",
            "at the end of the day", "take it offline", "drill down"
        ];
        phrases.iter().copied().collect()
    };

    pub static ref CLICHES: HashSet<&'static str> = {
        let cliches = [
            "avoid it like the plague", "beat around the bush", "better late than never",
            "bite the bullet", "break the ice", "bring to the table", "call it a day",
            "cut to the chase", "easy as pie", "get the ball rolling", "hit the nail on the head",
            "in the nick of time", "it goes without saying", "jump on the bandwagon",
            "keep your eyes peeled", "let the cat out of the bag", "piece of cake",
            "raining cats and dogs", "the best of both worlds", "throw in the towel",
            "time flies", "under the weather", "when pigs fly", "whole nine yards",
            "a blessing in disguise", "a dime a dozen", "actions speak louder than words",
            "add insult to injury", "at the drop of a hat", "back to square one",
            "barking up the wrong tree", "bent out of shape", "bite off more than you can chew",
            "break a leg", "burning the midnight oil", "caught between a rock and a hard place",
            "costs an arm and a leg", "cry over spilled milk", "curiosity killed the cat",
            "devil's advocate", "don't count your chickens", "every cloud has a silver lining"
        ];
        cliches.iter().copied().collect()
    };

    pub static ref SENSORY_WORDS: HashMap<&'static str, HashSet<&'static str>> = {
        let mut map = HashMap::new();
        
        map.insert("sight", [
            "see", "saw", "seen", "look", "looked", "looking", "watch", "watched",
            "bright", "dark", "light", "shadow", "color", "colorful", "shiny", "dull",
            "vivid", "brilliant", "gleaming", "glowing", "sparkling", "shimmering",
            "transparent", "opaque", "visible", "invisible", "appearance", "view",
            "glimpse", "glance", "stare", "gaze", "observe", "notice", "spot"
        ].iter().copied().collect());

        map.insert("sound", [
            "hear", "heard", "listen", "listened", "sound", "noise", "loud", "quiet",
            "silent", "whisper", "shout", "scream", "yell", "murmur", "mumble",
            "echo", "ring", "buzz", "hum", "bang", "crash", "thump", "click",
            "rustle", "crackle", "pop", "snap", "sizzle", "hiss", "roar", "howl",
            "musical", "melodious", "harmonious", "deafening", "piercing"
        ].iter().copied().collect());

        map.insert("touch", [
            "feel", "felt", "touch", "touched", "soft", "hard", "smooth", "rough",
            "texture", "cold", "hot", "warm", "cool", "freezing", "burning", "icy",
            "sticky", "slippery", "dry", "wet", "moist", "damp", "sharp", "dull",
            "coarse", "silky", "velvety", "grainy", "bumpy", "prickly", "tender",
            "firm", "solid", "squishy", "fluffy", "crisp", "brittle"
        ].iter().copied().collect());

        map.insert("smell", [
            "smell", "smelled", "smelling", "scent", "odor", "aroma", "fragrance",
            "perfume", "stink", "stench", "whiff", "sniff", "fragrant", "aromatic",
            "pungent", "acrid", "musty", "moldy", "fresh", "stale", "rancid",
            "sweet", "sour", "spicy", "floral", "earthy", "smoky", "putrid"
        ].iter().copied().collect());

        map.insert("taste", [
            "taste", "tasted", "tasting", "flavor", "flavored", "sweet", "sour",
            "bitter", "salty", "savory", "spicy", "tangy", "tart", "bland", "mild",
            "delicious", "tasty", "appetizing", "mouthwatering", "scrumptious",
            "palatable", "flavorful", "zesty", "peppery", "sugary", "acidic"
        ].iter().copied().collect());

        map
    };

    pub static ref HIDDEN_VERBS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("decision", "decide");
        map.insert("conclusion", "conclude");
        map.insert("assumption", "assume");
        map.insert("observation", "observe");
        map.insert("consideration", "consider");
        map.insert("implementation", "implement");
        map.insert("investigation", "investigate");
        map.insert("examination", "examine");
        map.insert("explanation", "explain");
        map.insert("discussion", "discuss");
        map.insert("analysis", "analyze");
        map.insert("recommendation", "recommend");
        map.insert("suggestion", "suggest");
        map.insert("description", "describe");
        map
    };

    pub static ref CONJUNCTIONS: HashSet<&'static str> = {
        ["and", "but", "or", "so", "yet", "for", "nor"].iter().copied().collect()
    };

    pub static ref US_UK_PAIRS: Vec<(&'static str, &'static str)> = {
        vec![
            ("color", "colour"), ("favor", "favour"), ("honor", "honour"),
            ("labor", "labour"), ("neighbor", "neighbour"), ("center", "centre"),
            ("meter", "metre"), ("fiber", "fibre"), ("organize", "organise"),
            ("recognize", "recognise"), ("analyze", "analyse"), ("defense", "defence"),
            ("license", "licence"), ("traveling", "travelling"), ("canceled", "cancelled")
        ]
    };

    pub static ref HYPHEN_PATTERNS: Vec<(&'static str, &'static str)> = {
        vec![
            ("email", "e-mail"),
            ("online", "on-line"),
            ("website", "web-site"),
            ("today", "to-day"),
            ("cooperate", "co-operate"),
            ("coordinate", "co-ordinate")
        ]
    };
}
