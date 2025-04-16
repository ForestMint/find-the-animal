mod animal;

use crate::animal::Class::*;
use crate::animal::Continent::*;
use crate::animal::Diet::*;


fn main () {

    /*
    let lion = animal::Animal();
    let tiger = animal::Animal();
    let bat = animal::Animal();
    let seagull = animal::Animal();
    */

    let lion = animal::Animal {
        name: "lion".to_string(),
        class: Mammal,
        area: Africa,
        diet: Carnivore,
    };

    let tiger = animal::Animal {
        name: "tiger".to_string(),
        class: Mammal,
        area: Asia,
        diet: Carnivore,
    };

    let cobra = animal::Animal {
        name: "cobra".to_string(),
        class: Reptile,
        area: Asia,
        diet: Carnivore,
    };

    let chameleon = animal::Animal {
        name: "chameleon".to_string(),
        class: Reptile,
        area: Americas,
        diet: Carnivore,
    };

    let elephant = animal::Animal {
        name: "elephant".to_string(),
        class: Mammal,
        area: Africa,
        diet: Herbivore,
    };

    let zebra = animal::Animal {
        name: "zebra".to_string(),
        class: Mammal,
        area: Africa,
        diet: Herbivore,
    };

    let hippopotamus = animal::Animal {
        name: "hippopotamus".to_string(),
        class: Mammal,
        area: Africa,
        diet: Herbivore,
    };

    let rhinoceros = animal::Animal {
        name: "rhinoceros".to_string(),
        class: Mammal,
        area: Africa,
        diet: Herbivore,
    };

    let lama = animal::Animal {
        name: "lama".to_string(),
        class: Mammal,
        area: Americas,
        diet: Herbivore,
    };

    let colibri = animal::Animal {
        name: "colibri".to_string(),
        class: Bird,
        area: Americas,
        diet: Omnivore,
    };

    let panda = animal::Animal {
        name: "panda".to_string(),
        class: Mammal,
        area: Asia,
        diet: Herbivore,
    };

    let koala = animal::Animal {
        name: "koala".to_string(),
        class: Mammal,
        area: Oceania,
        diet: Herbivore,
    };


}