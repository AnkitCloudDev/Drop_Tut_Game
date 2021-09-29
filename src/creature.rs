use std::fmt::{Display, Formatter, Error};
use core::fmt;

pub struct Creature{
    name: String
}

impl Creature{
    pub fn new(name: &str) -> Creature{
        println!("Creature called {} enters the game",name);
        return Creature{name: name.into()}
    }

}

impl Drop for Creature{
    fn drop(&mut self) {
        println!("{} is dead",self.name);
    }
}
// impl ToString for Creature{
//     fn to_string(&self) -> String {
//         self.name.into()
//     }
// }
impl fmt::Display for Creature{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
        write!(f, "{}", self.name)
    }
}
