#[derive(Clone, PartialEq)]
pub enum Class {
    Mammal,
    Bird,
    Reptile
}

#[derive(Clone, PartialEq)]
pub enum Continent {
    Africa,
    Americas,
    Antarctica,
    Asia,
    Europe,
    Oceania
}

#[derive(Clone, PartialEq)]
pub enum Diet {
    Carnivore,
    Herbivore,
    Omnivore
}

#[derive(Clone)]
pub struct Animal {
    pub name: String,
    pub class: Class,
    pub area: Vec<Continent>,
    pub diet : Diet,
    pub has_wings: bool,
    pub can_fly: bool,
}