
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
fn shortened_strings(short_elements: &mut Vec<String>){
    short_elements.iter_mut().for_each(|el | el.truncate(1));
    //print_elements(&short_elements);
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("red"),
        String::from("red")
    ];
    shortened_strings(&mut colors);
    print_elements(&colors);
}
