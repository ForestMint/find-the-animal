
mod animal;

use crate::game::animal::Class::*;
use crate::game::animal::Continent::*;
use crate::game::animal::Diet::*;

use crate::game::animal::Animal;

pub struct Game {
    deck: Vec<Animal>,
    board: Vec<Animal>,
    hand: Vec<Animal>,
    name_of_secret_animal: String,
    is_over: bool,
}

pub fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

impl Game {

    pub fn new() -> Game {



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

        let greater_flamingo = animal::Animal {
            name: "greater_flamingo".to_string(),
            class: Bird,
            area: vec![Africa, Asia, Europe],
            diet: Carnivore,
            has_wings: true,
            can_fly: true,
        };


        let toucan = animal::Animal {
            name: "toucan".to_string(),
            class: Bird,
            area: vec![Americas],
            diet: Herbivore,
            has_wings: true,
            can_fly: true,
        };

        let my_deck_of_animals = vec![lion, tiger, cobra, chameleon];
        let my_board_of_animals = vec![elephant, zebra, hippopotamus, rhinoceros, lama, colibri, panda, koala, komodo_dragon, seagull, penguin, walrus];
        let my_hand_of_animals = vec![iguana, crocodile, ostrich, greater_flamingo, toucan];




        


        Game {
            deck: my_deck_of_animals, 
            board: my_board_of_animals,
            hand: my_hand_of_animals,
            name_of_secret_animal: "walrus".to_string(),
            is_over: false
        }
    }

    pub fn is_over(&self) -> bool {
        if self.is_secret_animal_shot() {
            return true ;
        }
        else {
            
            if (&self.board).len()==1{
                return true ;
            }
        }
        
        return false;

    }

    pub fn shoot_animal(&mut self, animal_name : String) {
        let mut index = 0;
        for animal in &self.board {
            if (animal.name == animal_name) {
                break;
                
            }else {
                index +=1;
            }
        }

        self.board.remove(index-1);
    }



    pub fn is_animal_on_board(&mut self, animal_name : String) -> bool {
        for animal in &self.board {
            
            //print_type(&animal.name);
            //print_type(&animal_name);
            //println!("{}",animal.name);
            //println!("{}",animal_name);
            //println!("{}", animal.name == animal_name);
            //println!("{}", "walrus" == "walrus");
            
            
            //if 1== 1{

            if animal.name.trim() == animal_name.trim() {
                return true;
            }
            
        }
        return false;
    }


    pub fn is_secret_animal_shot(&self) -> bool {
        let mut secret_animal_found = false;
        for animal in &self.board {
            if (animal.name.trim() == self.name_of_secret_animal.trim()) {
                return false;
            }
        }
        return true;
    }
    


}