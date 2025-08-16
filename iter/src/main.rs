fn print_elements(elements : &[String]){ //this is slice of a vector
    // for i in elements{
    //     println!("{}", i)
    // }

    elements
    .iter()
    .map(|el| format!("{} {}", el, el))
    .for_each(|el| println!("{}",el));
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("red"),
        String::from("red")
    ];

    print_elements(&colors[1..3]);
}
