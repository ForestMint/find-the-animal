use crate::game::pool_of_animals::animal::Class::*;
use crate::game::pool_of_animals::animal::Continent::*;
use crate::game::pool_of_animals::animal::Diet::*;

mod animal;

pub struct PoolOfAnimals {
    pub vector_of_animals : Vec<animal::Animal>,
}

impl PoolOfAnimals {

    pub fn fill() -> PoolOfAnimals {

        let lion = animal::Animal {
            name: "lion".to_string(),
            class: Mammal,
            area: vec![Africa],
            diet: Carnivore,
            has_wings: false,
            can_fly: false,
        };
    
        let tiger = animal::Animal {
            name: "tiger".to_string(),
            class: Mammal,
            area: vec![Asia],
            diet: Carnivore,
            has_wings: false,
            can_fly: false,
        };
    
        let cobra = animal::Animal {
            name: "cobra".to_string(),
            class: Reptile,
            area: vec![Asia],
            diet: Carnivore,
            has_wings: false,
            can_fly: false,
        };
    
        let chameleon = animal::Animal {
            name: "chameleon".to_string(),
            class: Reptile,
            area: vec![Americas],
            diet: Carnivore,
            has_wings: false,
            can_fly: false,
        };
    
        let elephant = animal::Animal {
            name: "elephant".to_string(),
            class: Mammal,
            area: vec![Africa, Asia],
            diet: Herbivore,
            has_wings: false,
            can_fly: false,
        };
    
        let zebra = animal::Animal {
            name: "zebra".to_string(),
            class: Mammal,
            area: vec![Africa],
            diet: Herbivore,
            has_wings: false,
            can_fly: false,
        };
    
        let hippopotamus = animal::Animal {
            name: "hippopotamus".to_string(),
            class: Mammal,
            area: vec![Africa],
            diet: Herbivore,
            has_wings: false,
            can_fly: false,
        };
    
        let rhinoceros = animal::Animal {
            name: "rhinoceros".to_string(),
            class: Mammal,
            area: vec![Africa],
            diet: Herbivore,
            has_wings: false,
            can_fly: false,
        };
    
        let lama = animal::Animal {
            name: "lama".to_string(),
            class: Mammal,
            area: vec![Americas],
            diet: Herbivore,
            has_wings: false,
            can_fly: false,
        };
    
        let colibri = animal::Animal {
            name: "colibri".to_string(),
            class: Bird,
            area: vec![Americas],
            diet: Omnivore,
            has_wings: true,
            can_fly: true,
        };
    
        let panda = animal::Animal {
            name: "panda".to_string(),
            class: Mammal,
            area: vec![Asia],
            diet: Herbivore,
            has_wings: false,
            can_fly: false,
        };
    
        let koala = animal::Animal {
            name: "koala".to_string(),
            class: Mammal,
            area: vec![Oceania],
            diet: Herbivore,
            has_wings: false,
            can_fly: false,
        };
    
        let komodo_dragon = animal::Animal {
            name: "komodo_dragon".to_string(),
            class: Reptile,
            area: vec![Asia],
            diet: Carnivore,
            has_wings: false,
            can_fly: false,
        };
    
        let seagull = animal::Animal {
            name: "seagull".to_string(),
            class: Bird,
            area: vec![Africa, Asia, Americas, Europe, Oceania],
            diet: Carnivore,
            has_wings: true,
            can_fly: true,
        };
    
        let penguin = animal::Animal {
            name: "penguin".to_string(),
            class: Bird,
            area: vec![Antarctica],
            diet: Carnivore,
            has_wings: true,
            can_fly: false,
        };
    
        let walrus = animal::Animal {
            name: "walrus".to_string(),
            class: Mammal,
            area: vec![Antarctica],
            diet: Carnivore,
            has_wings: false,
            can_fly: false,
        };

        let iguana = animal::Animal {
            name: "iguana".to_string(),
            class: Reptile,
            area: vec![Americas],
            diet: Carnivore,
            has_wings: false,
            can_fly: false,
        };

        let crocodile = animal::Animal {
            name: "crocodile".to_string(),
            class: Reptile,
            area: vec![Africa, Americas],
            diet: Carnivore,
            has_wings: false,
            can_fly: false,
        };

        let ostrich = animal::Animal {
            name: "ostrich".to_string(),
            class: Bird,
            area: vec![Africa],
            diet: Omnivore,
            has_wings: true,
            can_fly: false,
        };

        let my_vector_of_animals = vec![lion, tiger];

        PoolOfAnimals { vector_of_animals: my_vector_of_animals }

    }
}