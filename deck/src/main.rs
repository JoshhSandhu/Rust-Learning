#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let deck: Deck = Deck { cards: vec![] };

    println!("this is your deck of cards: {:?}", deck);
}

//another way to type an empty vector is cards: vec :: new()