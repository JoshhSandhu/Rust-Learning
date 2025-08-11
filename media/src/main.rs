//use std::{collections::hash_map::Values, num::FpCategory};

mod content;
use content::media::Media;
use content::catalog::Calalog;

fn print_media(Media:Media){
    println!("{:#?}", Media)
}

fn main() {
    println!("Enums in Rust");
    
    let audiobook = Media::Audiobook { 
        title: String::from("Syntax") 
    };
    
    let goodmovie = Media::movie { 
        title: String::from("interstller"), 
        director: String::from("Christopher Nolan") 
    };

    let niceBook = Media::book { 
        title: String::from("harry potter"), 
        author: String::from("JK rowling") 
    };

    let podcats = Media::podcast((10));

    let Placeholder = Media::placeholder;

    let mut calalog = Calalog::new();

    calalog.add(audiobook);
    calalog.add(goodmovie);
    calalog.add(niceBook);
    calalog.add(podcats);
    calalog.add(Placeholder);
    
    let item = calalog.get_by_index(40);
    let holder = Media::placeholder;
    
    println!("{:#?}", item.unwrap_or(&holder));

    //-------------------<>--------------------------------------
    // match calalog.get_by_index(100){
    //     Some(value) => {
    //         println!("item : {:#?}", value)
    //     }
    //     None =>{
    //         println!("no value here")
    //     }
    // }

    //we can also use pattern matching for this case
    // if let mighHaveaValue::thereIsAvalue(value) = calalog.get_by_index(0){
    //     println!("item in patten match: {:#?}", value)
    // } else {
    //     println!("there is no value of such!!!!!")
    // }

    // println!("{:#?}", item );

    // match calalog.item.get(100){
    //     Some(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     None =>{
    //         println!("nothing at this value");
    //     }
    // }

    //println!("{:#?}", calalog.item.get(0));

    // println!("{}",audiobook.description());
    // println!("{}",goodmovie.description());
    // println!("{}",niceBook.description());

    // print_Media(audiobook);
    // print_Media(goodmovie);
    // print_Media(niceBook);
}
