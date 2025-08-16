use std::vec;


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
    .collect::<Vec<_>>() //type inference the compiler figures out the value that goes here
    //this is called a trubofish
}

//moves elemts from one vector to another
fn move_elements(Vec_A : Vec<String>, vec_B: &mut Vec<String>){ 
    Vec_A.into_iter().for_each(|el| vec_B.push(el));
}

//turns a Vec<String> into Vec<Vec<String>>
fn explode(elements : &[String]) -> Vec<Vec<String>>{
    elements.iter()
    .map(|el| el.chars().map(|c| c.to_string()).collect()
    )
    .collect()
}

//finds matching color and returns a fallback
fn find_color_or(elements : &[String], search:&str, fallbacks:&str) -> String {
    elements
    .iter()
    .find(|el| el.contains(search))
    .map_or(String::from(fallbacks), |el|el.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green")
    ];

    let mut othercolors= vec![
        String::from("orange"),
        String::from("violet"),
        String::from("yellow")
    ];

    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);
    shortened_strings(&mut colors[1..3]);
    print_elements(&colors);
    
    let expoded = explode(&colors);
    println!("{:#?}", expoded);
    let mut moved = vec![];

    move_elements(colors, &mut moved);
    println!("{:#?}", moved);

    let found_color = find_color_or(
        &othercolors, 
        "re", 
        "no colors");
        println!("{:#?}",found_color);
}
