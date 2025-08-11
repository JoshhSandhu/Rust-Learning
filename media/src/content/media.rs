#[derive(Debug)]
pub enum Media{
    book{title: String, author: String},
    movie{title: String, director: String},
    Audiobook{title: String},
    podcast(u32),
    placeholder
}
//how would we work with a variant with raw data assigned to it and one with no data assigned

impl Media {
    pub fn description(&self) -> String {

        //pattern matching
        //we dont know if self is an audiobook, movie or an book
        // if let Media::book{ title, author} = self {
        //     format!("Book:{} {}", title, author)
        // }
        // else if let Media::movie{title,director} = self{
        //     format!("Movie:{} {}", title, director)
        // }
        // else if let Media::Audiobook{title}=self{
        //     format!("Audiobook: {}", title)
        // }
        // else{
        //     String::from("Media Description")
        // }

        match self{
            Media::book { title, author } =>{
                format!("Book:{} {}", title, author)
            }
            Media::movie { title, director } =>{
                format!("Movie:{} {}", title, director)
            }
            Media::Audiobook { title } =>{
                format!("Audiobook: {}", title)
            }
            Media::podcast(ep_num) => {
                format!("Podcast:{}", ep_num)
            }
            Media::placeholder =>{
                format!("placeholder")
            }
        }
    }
}