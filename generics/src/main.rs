use num_traits::{Float, ToPrimitive};

//when we have only T that means we have only one generic float type
//when we add another one U that means we have 2 seperate generic types
fn solve<T: Float, U:Float>(a: T, b: U) -> f64 {
    
    let a_f64 = a.to_f64().unwrap(); // use this crate to convert numbers
    let b_f64 = b.to_f64().unwrap(); // use this crate to convert numbers

    (a_f64.powi(1) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.45;
    let b: f64 = 6.54;

    println!("{}", solve(a, b));
}


//we cant do arithmetic btw diff types of numbers like we cant do f32 + f64 and all other nuumbers