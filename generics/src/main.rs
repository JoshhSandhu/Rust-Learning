use num_traits::{Float, ToPrimitive};


fn solve<T: Float>(a: T, b: T) -> f64 {
    
    let a_f64 = a.to_f64().unwrap(); // use this crate to convert numbers
    let b_f64 = b.to_f64().unwrap(); // use this crate to convert numbers

    (a_f64.powi(1) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.45;
    let b: f32 = 6.54;

    println!("{}", solve::<f32>(a, b));
}


//we cant do arithmetic btw diff types of numbers like we cant do f32 + f64 and all other nuumbers