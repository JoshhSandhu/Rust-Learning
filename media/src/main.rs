#[derive(Debug)]
enum media{
    book{title: String, author: String},
    movie{title: String, director: String},
    Audiobook{title: String}
}

impl media {
    fn description(&self) -> String {
        //we dont know if self is an audiobook, movie or an book
        // if let media::book{ title, author} = self {
        //     format!("Book:{} {}", title, author)
        // }
        // else if let media::movie{title,director} = self{
        //     format!("Movie:{} {}", title, director)
        // }
        // else if let media::Audiobook{title}=self{
        //     format!("Audiobook: {}", title)
        // }
        // else{
        //     String::from("media Description")
        // }

        match self{
            media::book { title, author } =>{
                format!("Book:{} {}", title, author)
            }
            media::movie { title, director } =>{
                format!("Movie:{} {}", title, director)
            }
            media::Audiobook { title } =>{
                format!("Audiobook: {}", title)
            }
        }
    }
}

#[derive(Debug)]
struct Calalog{
    item: Vec<media>
}

impl Calalog {
    fn new() -> Self{
        Calalog{ item: vec![]}
    }

    fn add(&mut self, media:media){
        self.item.push(media);
    }
}

fn print_media(media:media){
    println!("{:#?}", media)
}

fn main() {
    println!("Enums in Rust");
    
    let audiobook = media::Audiobook { 
        title: String::from("Syntax") 
    };
    
    let goodmovie = media::movie { 
        title: String::from("interstller"), 
        director: String::from("Christopher Nolan") 
    };

    let niceBook = media::book { 
        title: String::from("harry potter"), 
        author: String::from("JK rowling") 
    };

    let mut calalog = Calalog::new();

    calalog.add(audiobook);
    calalog.add(goodmovie);
    calalog.add(niceBook);

    println!("{:#?}", calalog);

    // println!("{}",audiobook.description());
    // println!("{}",goodmovie.description());
    // println!("{}",niceBook.description());

    // print_media(audiobook);
    // print_media(goodmovie);
    // print_media(niceBook);
}
