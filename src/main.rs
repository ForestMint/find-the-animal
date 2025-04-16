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


}