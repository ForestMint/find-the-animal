enum Class {
    Mammal,
    Bird,
    Reptile
}

enum Continent {
    Africa,
    Americas,
    Asia,
    Europe,
    Oceania
}

enum Diet {
    Carnivore,
    Herbivore,
    Omnivore
}

pub struct Animal {
    name: String,
    class: Class,
    area: Continent,
    diet : Diet,
}

