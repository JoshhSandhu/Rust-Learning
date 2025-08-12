use std::fs;
use std::io::Error;
fn main() {

    // let text = fs::read_to_string("logs.txt");
    // println!("{:#?}", text);

    match devide(5.0, 0.0){
        Ok(result_of_division) =>{
            println!("{}", result_of_division);
        }
        Err(what_went_wrong) =>{
            println!("{}", what_went_wrong)
        }
    }

    match validate_email(String::from("josh.sandhu@email.com")){
        Ok(..) => println!("email is valid"),
        Err(er) =>{
            println!("{}", er)
        }
    }
}

fn validate_email(email : String) -> Result<(), Error>{
    if email.contains("@"){
        Ok(())
    } else {
        Err(Error::other("emails should contain @"))
    } 
}

fn devide(a: f64, b: f64) -> Result<f64, Error>{
    if b == 0.0{
        Err(Error::other("cant devide by 0"))
    } else{
        Ok(a/b)
    }
}