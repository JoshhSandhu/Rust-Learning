use rand::{thread_rng, seq::SliceRandom};
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Deck {
        //list of suits: hearts spades clubs and diamonds
        let suits:[&'static str; 4]= ["Spades", "Hearts", "Clubs", "Diamonds"];
        
        //list of values: Ace two three etc
        let values:[&'static str; 13]= ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "jack", "queen", " king"]; 


        let mut cards:Vec<String> = vec![];
        //we add mut to the binding to make it mutable

        //Double nested for loop
        for suit in suits{
            for val in values{
                let card = format!("{} of {}", val, suit);
                cards.push(card); 
            }
        }

        return Deck { cards }
    }

    fn shuffle(&mut self){
        //we use the mut here cause the rng is calling the thread_rng fn that will change the data in some way
         let mut rng = thread_rng();
         self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("this is your deck of cards: {:#?}", deck);
    // we use this {:#?} to display the output in a more ledgible manner.
}

//another way to type an empty vector is cards: vec :: new()