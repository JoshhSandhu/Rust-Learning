use std::{fs, result};
use std::io::{read_to_string, Error};

fn extract_errors(text: &str) -> Vec<&str>{
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text{
        if line.starts_with("ERROR"){
            results.push(line);
        }
    }

    results
}

fn extract_warnings(warnings:&str)-> Vec<&str>{
    let split_text = warnings.split("\n");

    let mut results = vec![];

    for i in split_text{
        if i.starts_with("WARNING"){
            results.push(i);
        }
    }
    results
}

fn extract_info(information:&str) -> Vec<&str>{
    let split_text = information.split("\n");

    let mut results = vec![];

    for j in split_text{
        if j.starts_with("INFO"){
            results.push(j);
        }
    }
    results
}

fn main() -> Result<(),Error>{
    let info_text = fs::read_to_string("logs.txt")?;
    let info_things = extract_info(info_text.as_str());
    fs::write("info.txt", info_things.join("\n"))?;

    let wt = fs::read_to_string("logs.txt").expect("failed to read file");
    let warning_text = extract_warnings(wt.as_str());
    fs::write("warnings.txt", warning_text.join("\n")).expect("failed to write to text");

    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) =>{
            let error_logs = extract_errors(text_that_was_read.as_str());
            
            match fs::write("errors.txt", error_logs.join("\n")){
                Ok(()) => println!("wrote errors.txt"),
                Err(reason_write_failed) =>{
                    println!("error in writing file:{}", reason_write_failed)
                }
            }
        }
        Err(why_this_failed) =>{
            println!("failed to read file: {}", why_this_failed);
        }
    }
    
    
    fn the_match_statments_for_the_examples(){

        //these were just examples
        // match devide(5.0, 0.0){
        //     Ok(result_of_division) =>{
        //         println!("{}", result_of_division);
        //     }
        //     Err(what_went_wrong) =>{
        //         println!("{}", what_went_wrong)
        //     }
        // }

        // match validate_email(String::from("josh.sandhu@email.com")){
        //     Ok(..) => println!("email is valid"),
        //     Err(er) =>{
        //         println!("{}", er)
        //     }
        // }
    }

    Ok(())
}


fn example_for_erro_handling(){
        // fn validate_email(email : String) -> Result<(), Error>{
        //     if email.contains("@"){
        //         Ok(())
        //     } else {
        //         Err(Error::other("emails should contain @"))
        //     } 
        // }

        // fn devide(a: f64, b: f64) -> Result<f64, Error>{
        //     if b == 0.0{
        //         Err(Error::other("cant devide by 0"))
        //     } else{
        //         Ok(a/b)
        //     }
        // }
}