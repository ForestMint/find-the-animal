
pub enum Class {
    Mammal,
    Bird,
    Reptile
}

pub enum Continent {
    Africa,
    Americas,
    Antarctica,
    Asia,
    Europe,
    Oceania
}

pub enum Diet {
    Carnivore,
    Herbivore,
    Omnivore
}

pub struct Animal {
    pub name: String,
    pub class: Class,
    pub area: Vec<Continent>,
    pub diet : Diet,
}

