

//finds the given language and returns the next one
fn next_language<'a>(language: &'a [String], current: &str) -> &'a str{
    let mut foound = false;
    for i in language{
        if foound{
            return i;
        }
        if i == current{
            foound = true;
        }
    }

    language.last().unwrap()
}

//retuns the last element of the vector
fn last_language(language: &[String]) ->&str { 
    language.last().unwrap()
}//in this the rust compiler fiugures out itself the lifetime annotations and the relation btw elements.

//returns the longer of the two languages
fn longest_language<'a>(lang_a: &'a str,lang_b: &'a str)->&'a str{
    if lang_a.len() >= lang_b.len(){
        lang_a
    }else{
        lang_b
    }
}

fn main() {
    let languages =vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript")
    ];

    let resut_is = next_language(&languages, "go");
    println!("{:#?}", resut_is);

    let the_last = last_language(&languages);
    println!("{:#?}", the_last);

    let longest = longest_language("go","typescript" );
    println!("{:#?}", longest);
}
