# dancemore-usernamegen

Rust port of a golang tool that generates usernames based on the $Adjective $Animal format.

golang version from old, maybe I'll CICD it too... no sense in just deleting it tho.

```golang
package main

import (
	"flag"
	"fmt"
	"hash/fnv"
	"math/rand"
	"strconv"
	"time"
)

func FunName(username_seed string) (string) {
	adjectives := []string {
		"Adorable", "Adventurous", "Aggressive", "Agreeable", "Alert", "Alive",
		"Amused", "Angry", "Annoyed", "Annoying", "Anxious", "Arrogant",
		"Ashamed", "Attractive", "Average", "Awful", "Bad", "Beautiful",
		"Better", "Bewildered", "Black", "Bloody", "Blue", "Blue-eyed",
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
	}

	animals := []string {
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
	}

	hash := fnv.New64a()
	hash.Write([]byte(username_seed))
	seed := hash.Sum64()

	rand.Seed( int64(seed) )

	adj    := adjectives[rand.Intn(len(adjectives))]
	animal := animals[rand.Intn(len(animals))]

	return adj + animal
}

func main() {
	debugPtr := flag.Bool("debug", false, "output debug logs.")
	seedPtr  := flag.String("seed", "", "username seed (optional)")
	flag.Parse()

	// set the overall randomness...
        rand.Seed(time.Now().UTC().UnixNano())

	var seed_word string

	if (*seedPtr == "") {
		if (*debugPtr == true) {
			fmt.Printf("[!] seed not specified, generating...\n")
		}
		seed_word = strconv.Itoa( rand.Int() )
	} else {
		seed_word = *seedPtr
	}
	if (*debugPtr == true) {
		fmt.Printf("[-] seed-word is: %s\n", seed_word)
	}

	result := FunName(seed_word)
	fmt.Printf("%s\n", result)
}
```
