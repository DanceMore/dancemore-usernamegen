use std::env;

use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn fun_name(username_seed: &str) -> String {
    let adjectives = vec![
    		"Adorable", "Adventurous", "Aggressive", "Agreeable", "Alert", "Alive",
		"Amused", "Angry", "Annoyed", "Annoying", "Anxious", "Arrogant",
		"Ashamed", "Attractive", "Average", "Awful", "Bad", "Beautiful",
		"Better", "Bewildered", "Black", "Bloody", "Blue",
		"Blushing", "Bored", "Brainy", "Brave", "Breakable", "Bright",
		"Busy", "Calm", "Careful", "Cautious", "Charming", "Cheerful", "Clean",
		"Clear", "Clever", "Cloudy", "Clumsy", "Colorful", "Combative", "Comfortable",
		"Concerned", "Condemned", "Confused", "Cooperative", "Courageous", "Crazy",
		"Creepy", "Crowded", "Cruel", "Curious", "Cute", "Dangerous", "Dark",
		"Dead", "Defeated", "Defiant", "Delightful", "Depressed", "Determined",
		"Different", "Difficult", "Disgusted", "Distinct", "Disturbed", "Dizzy",
		"Doubtful", "Drab", "Dull", "Eager", "Easy", "Elated", "Elegant",
		"Embarrassed", "Enchanting", "Encouraging", "Energetic", "Enthusiastic",
		"Envious", "Evil", "Excited", "Expensive", "Exuberant", "Fair", "Faithful",
		"Famous", "Fancy", "Fantastic", "Fierce", "Filthy", "Fine", "Foolish", "Fragile",
		"Frail", "Frantic", "Friendly", "Frightened", "Funny", "Gentle", "Gifted",
		"Glamorous", "Gleaming", "Glorious", "Good", "Gorgeous", "Graceful", "Grieving",
		"Grotesque", "Grumpy", "Handsome", "Happy", "Healthy", "Helpful", "Helpless",
		"Hilarious", "Homeless", "Homely", "Horrible", "Hungry", "Hurt", "Ill",
		"Important", "Impossible", "Inexpensive", "Innocent", "Inquisitive", "Itchy",
		"Jealous", "Jittery", "Jolly", "Joyous", "Kind", "Lazy", "Light", "Lively",
		"Lonely", "Long", "Lovely", "Lucky", "Magnificent", "Misty", "Modern",
		"Motionless", "Muddy", "Mushy", "Mysterious", "Nasty", "Naughty", "Nervous",
		"Nice", "Nutty", "Obedient", "Obnoxious", "Odd", "Old-fashioned", "Open",
		"Outrageous", "Outstanding", "Panicky", "Perfect", "Plain", "Pleasant",
		"Poised", "Poor", "Powerful", "Precious", "Prickly", "Proud", "Putrid", "Puzzled",
		"Quaint", "Real", "Relieved", "Repulsive", "Rich", "Scary", "Selfish", "Shiny",
		"Shy", "Silly", "Sleepy", "Smiling", "Smoggy", "Sore", "Sparkling", "Splendid",
		"Spotless", "Stormy", "Strange", "Stupid", "Successful", "Super", "Talented", "Tame",
		"Tasty", "Tender", "Tense", "Terrible", "Thankful", "Thoughtful", "Thoughtless",
		"Tired", "Tough", "Troubled", "Ugliest", "Ugly", "Uninterested", "Unsightly",
		"Unusual", "Upset", "Uptight", "Vast", "Victorious", "Vivacious", "Wandering",
		"Weary", "Wicked", "Wide-eyed", "Wild", "Witty", "Worried", "Worrisome", "Wrong",
		"Zany", "Zealous",
		"Abundant", "Acrobatic", "Afraid", "Agile", "Airy", "Alarming", "Alerted", "Aloof",
		"Ambitious", "Ancient", "Angry", "Antsy", "Anxious", "Appreciative", "Artificial",
		"Ashen", "Astonished", "Athletic", "Attentive", "Authentic", "Awesome", "Awkward",
		"Baffled", "Balanced", "Bashful", "Beautiful", "Belligerent", "Beloved", "Beneficial", "Bent",
		"Bitter", "Bizarre", "Black", "Blissful", "Blushing", "Boastful", "Bold", "Bored", "Bouncy",
		"Brave", "Bright", "Brilliant", "Broad", "Broken", "Bumpy", "Burly", "Busy", "Calm", "Candid",
		"Canny", "Capable", "Careful", "Careless", "Caring", "Cautious", "Celestial", "Challenging",
		"Charming", "Cheerful", "Chilly", "Chosen", "Chubby", "Circular", "Classic", "Clean", "Clear",
		"Clever", "Clumsy", "Coarse", "Cold", "Colorful", "Colossal", "Comfortable", "Common", "Compassionate",
		"Competent", "Complete", "Complex", "Complicated", "Concerned", "Condemned", "Confident", "Confused",
		"Conscientious", "Constant", "Content", "Cool", "Cooperative", "Cordial", "Correct", "Cosmic", "Courageous",
		"Cowardly", "Cozy", "Crazy", "Creative", "Credible", "Creepy", "Crooked", "Crowded", "Cruel", "Crushing",
		"Cuddly", "Cultivated", "Cultured", "Curious", "Curly", "Curvy", "Cute", "Dainty", "Damp", "Dancing",
		"Dapper", "Daring", "Dark", "Darling", "Dazzling", "Dead", "Deadly", "Deafening", "Dear", "Debonair",
		"Decisive", "Decrepit", "Deep", "Defeated", "Defiant", "Delicate", "Delicious", "Delightful", "Dependent",
		"Depressing", "Deserted", "Desirable", "Desperate", "Detailed", "Determined", "Devoted", "Dim",
		"Dimpled", "Direct", "Dirty", "Disastrous", "Discreet", "Disgusting", "Distinct", "Distracted",
		"Disturbing", "Divine", "Dizzy", "Domineering", "Dopey", "Doting", "Double", "Doubtful", "Dramatic",
		"Dreamy", "Dreary", "Dry", "Dull", "Dusty", "Dynamic", "Eager", "Early", "Earnest", "Earthy", "Easy",
		"Easygoing", "Eclectic", "Economic", "Educated", "Efficient", "Elaborate", "Elastic", "Elderly", "Electric",
		"Elegant", "Elemental", "Elevated", "Elfin", "Elite", "Embarrassed", "Emotional", "Empathetic", "Empowered",
		"Empty", "Enchanted", "Endearing", "Endless", "Energetic", "Enormous", "Enthusiastic", "Entire", "Envious",
		"Equal", "Ethereal", "Ethical", "Euphoric", "Even", "Everlasting", "Evil", "Exalted", "Exasperating",
		"Excellent", "Excited", "Exciting", "Exclusive", "Exotic", "Expensive",
    ];

    let animals = vec![
		"Alligator", "Alpaca", "Antalope", "Axolotl", "Badger", "Bee",
		"Bison", "Buffalo", "Camel", "Canary", "Caracal", "Carp", "Cat",
		"Cattle", "Cheetah", "Chicken", "Chinchilla", "Chipmunk", "Cobra",
		"Cockatoo", "Cockroach", "Cricket", "Deer", "Dingo", "Dog", "Donkey",
		"Dove", "Dragon", "Duck", "Eagle", "Elephant", "Elk", "Emu", "Falcon",
		"Ferret", "Finch", "Fish", "Flea", "Fly", "Fox", "Frog", "Gazelle",
		"Gecko", "Gerbil", "Giraffe", "Goat", "Goldfish", "Goose", "Guineafowl",
		"Guppy", "Hamster", "Hawk", "Hedgehog", "Horse", "Hyena", "Ibex", "Iguana",
		"Koi", "Kudu", "Lemming", "Llama", "Lynx", "Manatee", "Mink", "Mongoose",
		"Monkey", "Moose", "Mouse", "Newt", "Ostrich", "Otter", "Owl", "Parakeet",
		"Parrot", "Pheasant", "Pig", "Pigeon", "Piranha", "Porcupine", "Python",
		"Quail", "Rabbit", "Raccoon", "Rat", "Raven", "Reindeer", "Rhino", "Robin",
		"Salmon", "Serval", "Sheep", "Silkmoth", "Skunk", "Snail", "Snake", "Sparrow",
		"Springbok", "Stoat", "Swan", "Trout", "Tuna", "Turkey", "Turtle", "Wallaby",
		"Weasel", "Wolf", "Yak", "Zebu",
		"Albatross", "Ant", "Armadillo", "Baboon", "Bat", "Bear", "Beaver", "Beetle", "Blackbird", "Boar",
                "Bobcat", "Butterfly", "Camelopard", "Capybara", "Caribou", "Caterpillar", "Chamois", "Cobra",
                "Condor", "Cougar", "Coyote", "Crane", "Crocodile", "Crow", "Dolphin", "Dormouse",
                "Echidna", "Eel", "Falcon", "Firefly",
                "Flamingo", "Fossa", "Gazelle", "Gibbon", "Gnu", "Gorilla", "Grasshopper",
                "Groundhog", "Hare", "Harpy", "Hedgehog", "Hippopotamus", "Hornbill", "Hummingbird",
                "Hydromedusa", "Ibis", "Jaguar", "Jellyfish", "Kangaroo", "Kingfisher", "Kinkajou", "Kiwi",
                "Koala", "Krill", "Ladybug", "Lancelet", "Lark", "Lemur", "Leopard", "Lion", "Lizard", "Lobster",
                "Locust", "Loris", "Louse", "Lynx", "Maggot", "Magpie", "Mallard", "Mammoth", "Manta", "Mantis",
                "Marmoset", "Marsupial", "Meerkat", "Megalodon", "Millipede", "Mole", "Mollusk",
                "Mongoose", "Moth", "Goat", "Mouse", "Mule", "Musk", "Narwhal", "Nautilus",
                "Nightingale", "Numbat", "Nuthatch", "Ocelot", "Octopus", "Okapi", "Opossum", "Orangutan",
		"Orca", "Oryx", "Ox", "Panda", "Panther", "Peacock", "Pelican", "Penguin", "Pheasant", "Piglet",
                "Pinniped", "Platypus", "Puma", "Pupa", "Python", "Quokka", "Raven", "Rhea", "Robin",
                "Rooster", "Salamander", "Scorpion", "Seagull", "Shark", "Shrimp", "Siamang", "Sloth", "Slug",
                "Smelt", "Starfish", "Stork", "Sturgeon", "Swallow", "Swift", "Swordfish", "Tapir", "Termite",
                "Thrush", "Tiger", "Toad", "Toucan", "Vicuna", "Viper", "Vole", "Vulture", "Walrus", "Warthog",
                "Whale", "Wildebeest", "Wolverine", "Wombat", "Woodchuck", "Woodpecker", "Wren", "Xerus", "Yabby",
                "Yeti", "Yak", "Zander", "Zebra", "Zebu", "Zorilla", "Zokor", "Zebrafish", "Zooplankton"
    ];

    let mut hasher = DefaultHasher::new();
    hasher.write(username_seed.as_bytes());
    let seed = hasher.finish();

    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let adj = rng.gen_range(0..adjectives.len());
    let animal = rng.gen_range(0..animals.len());

    let result = format!("{}{}", adjectives[adj], animals[animal]);
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut debug = false;
    let mut seed_word = String::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--debug" {
            debug = true;
        } else if args[i] == "--seed" {
            i += 1;
            seed_word = args[i].clone();
        }
        i += 1;
    }

    if debug {
        println!("[-] seed-word is: {}", seed_word);
    }

    let result = if seed_word.is_empty() {
        if debug {
            println!("[!] seed not specified, generating...");
        }
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(0..1000000000);
        fun_name(&rand_num.to_string())
    } else {
        fun_name(&seed_word)
    };

    println!("{}", result);
}
