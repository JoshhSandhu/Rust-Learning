
//prints each element one by one
fn print_elements(elements : &[String]){ //this is slice of a vector
    // for i in elements{
    //     println!("{}", i)
    // }

    elements
    .iter()
    .map(|el| format!("{} {}", el, el))
    .for_each(|el| println!("{}",el));
}

//shortens the strings in each vector to 1 char
fn shortened_strings(short_elements: &mut [String]){
    short_elements.iter_mut().for_each(|el | el.truncate(1));
    //print_elements(&short_elements);
}

//return a new vector with each element capitalized
fn to_uppercase(elements: &[String]) -> Vec<String>{
    elements.iter()
    .map(|el| el.to_uppercase())
    .collect()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green")
    ];

    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);
    shortened_strings(&mut colors[1..3]);
    print_elements(&colors);


}
