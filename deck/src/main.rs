struct Deck {
    cards: Vec<Card>,
}
//the name of the struct is always capitalized
//inside the struct we add feilds like cards in our case
//initialize card as a vector, vectors are arrays thta can grow and shrink
//arrays in rust have fixed length

fn main() {
    println!("Hello, world!");
}
// main is the most imp function in rust and println is used to print lines in the console
//the ! means this is a macro according to rust, they are similar to plain functions
//in rsut stings are enclosed in double quotes single quotes only enclose a char