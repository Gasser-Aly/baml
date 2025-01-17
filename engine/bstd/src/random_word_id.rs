use rand::{seq::SliceRandom, Rng};

const TAILS: &[&str] = &[
    "orca",
    "shark",
    "capybara",
    "fox",
    "turtle",
    "fish",
    "dolphin",
    "armadillo",
    "hedgehog",
    "dhole",
    "dog",
    "cat",
    "jaguar",
    "cheetah",
    "leopard",
    "cougar",
    "lion",
    "tapir",
    "anteater",
    "monkey",
    "snake",
    "scorpion",
    "jerboa",
    "opossum",
    "stingray",
    "colobus",
    "euplectes",
    "jay",
    "finch",
    "hawk",
    "beaver",
    "mouse",
    "moose",
    "alligator",
    "salamander",
    "tadpole",
    "astrapia",
    "pug",
    "greyhound",
    "foxhound",
    "bushbaby",
    "aardvark",
    "aardwolf",
    "pangolin",
    "porcupine",
    "genet",
    "springhare",
    "kangaroo",
    "koala",
    "ostrich",
    "dingo",
    "platypus",
    "camel",
    "horse",
    "echidna",
    "wombat",
    "crocodile",
    "whale",
    "narwhal",
    "humpback",
    "marmoset",
    "tucuxi",
    "beluga",
    "porpoise",
    "sparrow",
    "pigeon",
    "owl",
    "hummingbird",
    "robin",
    "starling",
    "manakin",
    "warbler",
    "penguin",
    "snowfinch",
    "broadbill",
    "thrush",
    "bishop",
    "swallow",
    "bittern",
    "caracara",
    "manx",
    "possum",
    "lemur",
    "deer",
    "peacock",
    "ratfish",
    "vulture",
    "rat",
    "takaya",
    "skunk",
    "tuxedo",
    "turkey",
    "elephant",
    "civet",
    "ainu",
    "husky",
    "akita",
    "alpaca",
    "spaniel",
    "terrier",
    "hare",
    "auroch",
    "axolotl",
    "bandicoot",
    "beagle",
    "beago",
    "collie",
    "tiger",
    "liger",
    "rhino",
    "bobcat",
    "corgi",
    "dachshund",
    "giraffe",
    "bonobo",
    "cetacean",
    "bear",
    "hyena",
    "burmese",
    "caracal",
    "goat",
    "chameleon",
    "chihuahua",
    "chimp",
    "cow",
    "cuscus",
    "pinscher",
    "dunker",
    "mau",
    "ermine",
    "feist",
    "spitz",
    "squirrel",
    "gerbil",
    "hamster",
    "panda",
    "gibbon",
    "flyingfox",
    "jackal",
    "macaque",
    "kinkajou",
    "lynx",
    "manatee",
    "mole",
    "ocelot",
    "otter",
    "panther",
    "pig",
    "prairiedog",
    "puma",
    "hippo",
    "quokka",
    "quoll",
    "bunny",
    "raccoon",
    "tamarin",
    "reindeer",
    "hyrax",
    "saiga",
    "sable",
    "serval",
    "spanador",
    "springbok",
    "tuatara",
    "dropbear",
    "vaquita",
    "wallaby",
    "walrus",
    "weasel",
    "wolf",
    "zebra",
    "zonkey",
    "donkey",
    "mule",
    "zebu",
    "cuttlefish",
    "unicorn",
    "meerkat",
    "jackalope",
    "heron",
    "fawn",
    "warthog",
    "drake",
    "badger",
    "zapus",
    "yak",
    "werewolf",
    "tahr",
    "fossa",
    "xerus",
    "centaur",
    "raptor",
    "long",
    "sheep",
    "quetzal",
    "wildebeest",
    "motmot",
    "coati",
    "drongo",
    "boston",
    "myth",
    "saga",
    "fable",
    "folk",
    "fairy",
    "hound",
    "risk",
    "coin",
    "tyrannosaurus",
    "velociraptor",
    "siren",
    "mudpuppy",
    "ferret",
    "roborovski",
    "bee",
    "dragon",
    "bearded",
    "beardie",
    "pogona",
    "chicken",
    "hen",
    "rooster",
    "quail",
    "grouse",
    "comet",
    "halley",
    "hale",
    "bopp",
    "sungrazing",
    "sungrazer",
    "exocomet",
    "hartley",
    "borrelly",
    "wild",
    "ison",
    "encke",
    "atlas",
    "hyakutake",
    "tempel",
    "west",
    "osiris",
    "basilisk",
    "ctesias",
    "equalacorn",
    "wyvern",
    "hippogriff",
    "hippogryph",
    "griffin",
    "hydra",
    "chimera",
    "phoenix",
    "faun",
    "minotaur",
    "amarok",
    "gorgon",
    "cerberus",
    "orthrus",
    "sphinx",
    "ladon",
    "pegasus",
    "scylla",
    "mermaid",
    "nessie",
    "hippocampus",
    "warg",
    "fenrir",
    "typhon",
    "wampus",
    "bicorn",
    "manticore",
    "qilin",
    "fluffy",
    "bun",
    "mercat",
    "coyote",
    "alpine",
    "lamancha",
    "angora",
    "beetal",
    "girgentana",
    "jamnapari",
    "kamori",
    "kiko",
    "oberhasli",
    "pygmy",
    "pygora",
    "rove",
    "saanen",
    "bilberry",
    "adal",
    "alai",
    "afrino",
    "alpines",
    "ancon",
    "blue",
    "boreray",
    "border",
    "cheviot",
    "churra",
    "cormo",
    "dolly",
    "dala",
    "dorper",
    "dohne",
    "gute",
    "han",
    "jacob",
    "kooka",
    "latxa",
    "lleyn",
    "lonk",
    "merino",
    "nilgiri",
    "orkhon",
    "rya",
    "soay",
    "xalda",
    "ewe",
    "lamb",
    "great",
    "emperor",
    "king",
    "adelie",
    "chinstrap",
    "gentoo",
    "little",
    "banded",
    "magellanic",
    "humboldt",
    "crested",
    "fiordland",
    "snares",
    "rockhopper",
    "royal",
    "macaroni",
    "bison",
    "buffalo",
    "gaur",
    "saola",
    "anoa",
    "banteng",
    "beefalo",
    "yakalo",
    "yattle",
    "tigon",
    "jaglion",
    "leopon",
    "savannah",
    "toyger",
    "cheetoh",
    "grolar",
    "zebroid",
    "coywolf",
    "wholphin",
    "narluga",
    "cama",
    "dzo",
    "zubron",
    "geep",
    "hinny",
    "mulard",
    "zeedonk",
    "zorse",
    "pumapard",
    "pizzly",
    "coydog",
    "whydah",
    "pheasant",
    "lyrebird",
    "peafowl",
    "tayra",
    "zorilla",
    "mara",
    "galago",
    "tenrec",
    "bettong",
    "tamandua",
    "cusimanse",
    "polecat",
    "degu",
    "coatimundi",
    "diplodocus",
    "stegosaurus",
    "zuul",
    "allosaurus",
    "baryonyx",
    "edmontosaurus",
    "iguanodon",
    "minmi",
    "triceratops",
    "troodon",
    "trex",
    "shetland",
    "pinto",
    "appaloosa",
    "auxois",
    "shire",
    "brumby",
    "fell",
    "java",
    "pony",
    "welara",
    "mammoth",
    "burro",
    "poitou",
    "spotted",
    "snowy",
    "barn",
    "barred",
    "boreal",
    "elf",
    "barking",
    "buru",
    "chaco",
    "chestnut",
    "cinnebar",
    "cloudforest",
    "dusky",
    "tawny",
    "berylline",
    "calliope",
    "rufous",
    "xantu",
    "violetear",
    "mango",
    "bumblebee",
    "emerald",
    "cinnamon",
    "golden",
    "pumpkinseed",
    "ruffe",
    "walleye",
    "perch",
    "bowfin",
    "burbot",
    "goldeye",
    "mooneye",
    "dace",
    "quillback",
    "stonecat",
    "albacore",
    "alewife",
    "amberjack",
    "sole",
    "codlet",
    "char",
    "searobin",
    "arowana",
    "bonito",
    "saury",
    "ayu",
    "silverside",
    "banjo",
    "barb",
    "barbel",
    "bangus",
    "banfish",
    "ray",
    "danio",
    "betta",
    "bigeye",
    "bicolor",
    "bitterling",
    "bleak",
    "blenny",
    "boga",
    "duck",
    "brill",
    "brotula",
    "buri",
    "goby",
    "catla",
    "chimaera",
    "cobia",
    "dab",
    "darter",
    "discus",
    "duckbill",
    "drum",
    "elver",
    "featherback",
    "garibaldi",
    "ghost",
    "ghoul",
    "dojo",
    "hake",
    "halfmoon",
    "halfbeak",
    "hamlet",
    "halibut",
    "halosaur",
    "hoki",
    "huchen",
    "ide",
    "inanga",
    "ilish",
    "inconnu",
    "dory",
    "koi",
    "kanyu",
    "kokanue",
    "lenok",
    "ling",
    "manta",
    "marlin",
    "mora",
    "mulley",
    "stargazer",
    "nase",
    "neon",
    "daggertooth",
    "noodlefish",
    "notothen",
    "tetra",
    "orfe",
    "opah",
    "opaleye",
    "pancake",
    "panga",
    "paradise",
    "parore",
    "pirarucu",
    "pirate",
    "platy",
    "pleco",
    "powan",
    "pomano",
    "paridae",
    "porgy",
    "rohu",
    "rudd",
    "skate",
    "squeaker",
    "tailor",
    "uaru",
    "vimba",
    "wahoo",
    "zebra",
    "coelacanth",
    "gila",
    "monster",
    "sun",
    "cirius",
    "canopus",
    "arcturus",
    "vega",
    "capella",
    "rigel",
    "procyon",
    "achernar",
    "betelgeuse",
    "hadar",
    "altair",
    "acrux",
    "aldebaran",
    "antares",
    "spica",
    "pollux",
    "fomalhaut",
    "deneb",
    "mimosa",
    "regulus",
    "adhara",
    "shaula",
    "castor",
    "gacrux",
    "bellatrix",
    "elnath",
    "miaplacidus",
    "alnilam",
    "alnair",
    "alnitak",
    "alioth",
    "dubhe",
    "mirfak",
    "wezen",
    "sargas",
    "avior",
    "alkaid",
    "menkalinan",
    "atria",
    "alhena",
    "peacock",
    "alsephina",
    "mirzam",
    "alphard",
    "polaris",
    "hamal",
    "algieba",
    "diphda",
    "mizar",
    "nunki",
    "menkent",
    "mirach",
    "alpheratz",
    "kochab",
    "saiph",
    "denebola",
    "algol",
    "tiaki",
    "mintaka",
    "draconis",
    "centauri",
    "becrux",
    "godzilla",
    "sirius",
    "vector",
    "cherimoya",
];

const SCALES: &[&str] = &[
    "lizard",
    "crocodile",
    "snake",
    "dragon",
    "catfish",
    "bass",
    "salmon",
    "tuna",
    "hammerhead",
    "eel",
    "carp",
    "trout",
    "mahi",
    "snapper",
    "bluegill",
    "sole",
    "cod",
    "triceratops",
    "edmontosaurus",
    "saurolophus",
    "liberty",
    "justice",
    "pangolin",
    "turtle",
    "tortoise",
    "alligator",
    "butterfly",
    "iguana",
    "pineapplefish",
    "pinecone",
    "anaconda",
    "puffin",
    "cardassian",
    "cloud",
    "nominal",
    "ordinal",
    "interval",
    "ratio",
    "vernier",
    "lime",
    "scala",
    "boa",
    "cobra",
    "richter",
    "kelvin",
    "celsius",
    "decibel",
    "beaufort",
    "mohs",
    "pauling",
    "python",
    "mamba",
    "alkaline",
    "climb",
    "moth",
    "eagle",
    "woodpecker",
    "morpho",
    "tautara",
    "anoles",
    "theropod",
    "owl",
    "frog",
    "lionfish",
    "morray",
    "clownfish",
    "ostrich",
    "stork",
    "egret",
    "map",
    "grue",
    "tiyanki",
    "broadnose",
    "basking",
    "goblin",
    "porbeagle",
    "chuckwalla",
    "tawny",
    "bramble",
    "kitefin",
    "agama",
    "komodo",
    "bull",
    "monitor",
    "chameleon",
    "tegus",
    "gecko",
    "micro",
    "gray",
    "allosaurus",
    "glyptodon",
    "basilisk",
    "cordylus",
    "tegu",
    "sailfin",
    "mountain",
    "dinosaur",
    "goanna",
    "herring",
    "minnow",
    "perch",
    "coho",
    "lake",
    "arctic",
    "pumpkinseed",
    "gopher",
    "chickadee",
    "toad",
    "shark",
    "roach",
    "tyrannosaurus",
    "velociraptor",
    "bee",
    "bearded",
    "beardie",
    "pogona",
    "chicken",
    "hen",
    "rooster",
    "quail",
    "grouse",
    "universe",
    "galaxy",
    "atlas",
    "wyvern",
    "hydra",
    "gorgon",
    "chimera",
    "ladon",
    "scylla",
    "mermaid",
    "nessie",
    "hippocampus",
    "typhon",
    "qilin",
    "acoustic",
    "alpha",
    "altered",
    "augmented",
    "bebop",
    "beta",
    "blues",
    "bushi",
    "byzantine",
    "chromatic",
    "delta",
    "diatonic",
    "diminished",
    "dominant",
    "solfege",
    "solfeggio",
    "dorian",
    "enigmatic",
    "freygish",
    "gamma",
    "harmonic",
    "heptatonic",
    "hexatonic",
    "hirajoshi",
    "in",
    "insen",
    "istrian",
    "iwato",
    "jazz",
    "locrian",
    "major",
    "minor",
    "mixolydian",
    "musical",
    "octatonic",
    "pentatonic",
    "phrygian",
    "pierce",
    "prometheus",
    "pythagorean",
    "symmetric",
    "tet",
    "tone",
    "tritone",
    "yo",
    "beaver",
    "hops",
    "salak",
    "mercat",
    "great",
    "emperor",
    "adelie",
    "chinstrap",
    "gentoo",
    "little",
    "banded",
    "magellanic",
    "humboldt",
    "crested",
    "fiordland",
    "snares",
    "rockhopper",
    "royal",
    "macaroni",
    "vibe",
    "vibes",
    "banana",
    "ulmer",
    "bortle",
    "palermo",
    "torino",
    "forel",
    "ule",
    "pyruvate",
    "kardashev",
    "ionian",
    "aeolian",
    "sunfish",
    "gar",
    "pike",
    "muskellunge",
    "pickerel",
    "ruffe",
    "walleye",
    "bowfin",
    "burbot",
    "goldeye",
    "mooneye",
    "dace",
    "quillback",
    "stonecat",
    "albacore",
    "alewife",
    "amberjack",
    "codlet",
    "char",
    "searobin",
    "arowana",
    "bonito",
    "saury",
    "ayu",
    "silverside",
    "banjo",
    "barb",
    "barbel",
    "bangus",
    "banfish",
    "ray",
    "danio",
    "betta",
    "bigeye",
    "bicolor",
    "bitterling",
    "bleak",
    "blenny",
    "boga",
    "duck",
    "brill",
    "brotula",
    "buri",
    "goby",
    "catla",
    "chimaera",
    "cobia",
    "dab",
    "darter",
    "discus",
    "duckbill",
    "drum",
    "elver",
    "featherback",
    "garibaldi",
    "ghost",
    "ghoul",
    "dojo",
    "hake",
    "halfmoon",
    "halfbeak",
    "hamlet",
    "halibut",
    "halosaur",
    "hoki",
    "huchen",
    "ide",
    "inanga",
    "ilish",
    "inconnu",
    "dory",
    "koi",
    "kanyu",
    "kokanue",
    "lenok",
    "ling",
    "manta",
    "marlin",
    "mora",
    "mulley",
    "stargazer",
    "nase",
    "neon",
    "daggertooth",
    "noodlefish",
    "notothen",
    "tetra",
    "orfe",
    "opah",
    "opaleye",
    "pancake",
    "panga",
    "paradise",
    "parore",
    "pirarucu",
    "pirate",
    "platy",
    "pleco",
    "powan",
    "pomano",
    "paridae",
    "porgy",
    "rohu",
    "rudd",
    "skate",
    "squeaker",
    "tailor",
    "uaru",
    "vimba",
    "wahoo",
    "zebra",
    "coelacanth",
    "gila",
    "monster",
    "sun",
    "cirius",
    "canopus",
    "arcturus",
    "vega",
    "capella",
    "rigel",
    "procyon",
    "achernar",
    "betelgeuse",
    "hadar",
    "altair",
    "acrux",
    "aldebaran",
    "antares",
    "spica",
    "pollux",
    "fomalhaut",
    "deneb",
    "mimosa",
    "regulus",
    "adhara",
    "shaula",
    "castor",
    "gacrux",
    "bellatrix",
    "elnath",
    "miaplacidus",
    "alnilam",
    "alnair",
    "alnitak",
    "alioth",
    "dubhe",
    "mirfak",
    "wezen",
    "sargas",
    "avior",
    "alkaid",
    "menkalinan",
    "atria",
    "alhena",
    "peacock",
    "alsephina",
    "mirzam",
    "alphard",
    "polaris",
    "hamal",
    "algieba",
    "diphda",
    "mizar",
    "nunki",
    "menkent",
    "mirach",
    "alpheratz",
    "kochab",
    "saiph",
    "denebola",
    "algol",
    "tiaki",
    "mintaka",
    "draconis",
    "centauri",
    "becrux",
    "godzilla",
    "sirius",
    "vector",
    "cherimoya",
];

/// Generate a random identifier in the format of <tail>-<scale>-<3 digit number>
///
/// Taken from Tailscale's wordlist: https://github.com/tailscale/tailscale/blob/v1.76.3/words/tails.txt
///
/// Criteria I evaluated:
///
///   - needs sufficient entropy
///   - should be easy to copy/paste, slightly easy to read
///   - should be SFW and not have profanity
///   - comfortably unoffensive, e.g. "angry-woman-172" is a terrible ID that wordlists allow
pub fn random_word_id() -> String {
    let mut rng = rand::thread_rng();

    format!(
        "{}-{}-{}",
        TAILS.choose(&mut rng).unwrap_or(&"leaping"),
        SCALES.choose(&mut rng).unwrap_or(&"llama"),
        rng.gen_range(100..1000)
    )
}
