#[derive(Clone)]
pub enum Class {
    Mammal,
    Bird,
    Reptile
}

#[derive(Clone)]
pub enum Continent {
    Africa,
    Americas,
    Antarctica,
    Asia,
    Europe,
    Oceania
}

#[derive(Clone)]
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