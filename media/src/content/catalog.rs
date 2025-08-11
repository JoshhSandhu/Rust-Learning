use super::media::Media;

#[derive(Debug)]
pub struct Calalog{
    item: Vec<Media>
}

impl Calalog {
    pub fn new() -> Self{
        Calalog{ item: vec![]}
    }

    pub fn add(&mut self, media:Media){
        self.item.push(media);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media>{
        if self.item.len() > index{
            Some((&self.item[index]))
        }
        else{
            None
        }
        
    }
}

//this enum that we made is use to show the working of the default option enum
// enum  mighHaveaValue<'a > {
//     thereIsAvalue(&'a media),
//     novalueAvalaible
// }