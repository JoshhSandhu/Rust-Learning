fn print_elements(elements : Vec<String>){
    // for i in elements{
    //     println!("{}", i)
    // }

    elements.iter().for_each(|el| println!("{}",el));
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("red"),
        String::from("red")
    ];

    print_elements(colors);
}
