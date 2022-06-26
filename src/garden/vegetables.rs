use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Plant {
    pub name: String
}


impl Display for Plant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Plant: {})", self.name)
    }
}