
mod animal;

use crate::game::animal::Class::*;
use crate::game::animal::Continent::*;
use crate::game::animal::Diet::*;

use crate::game::animal::Animal;

use rand::thread_rng;
use rand::seq::SliceRandom;

use rand::Rng;

#[derive(Clone)]
pub struct Game {
    deck: Vec<Animal>,
    pub board: Vec<Animal>,
    hand: Vec<Animal>,
    name_of_secret_animal: String,
    is_over: bool,
    unreached_targets: Vec<usize>,
}

pub fn average(numbers: Vec<i32>) -> i32 {
    // Remember how many numbers we were passed.
    let nnumbers = numbers.len() as i32;
    let mut sum = 0;
    // This will consume the numbers.
    for n in numbers {
        sum += n;
    }
    // Average (arithmetic mean) is sum divided by count.
    sum / nnumbers
}

pub fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

pub fn get_similarity_score (animal_1: Animal, animal_2: Animal) -> i32 {
    let mut similarity_score: i32 = 0;

    if animal_1.class == animal_2.class {
        similarity_score += 50;
    }

    if animal_1.has_wings && animal_2.has_wings {
        similarity_score += 25;
    }

    if animal_1.can_fly && animal_2.can_fly {
        similarity_score += 25;
    }

    /*
    if animal_1.diet == animal_2.diet {
        similarity_score += 20;
    }

    if animal_1.area == animal_2.area {
        similarity_score += 15;
    }
    */



    similarity_score
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

        let mut my_pool = vec![lion,tiger,cobra,chameleon,elephant, zebra, hippopotamus, rhinoceros, lama, colibri, panda, koala, komodo_dragon, seagull, penguin, walrus, iguana, crocodile, ostrich, greater_flamingo, toucan];
        my_pool.shuffle(&mut thread_rng());

        // Get a slice from index 0 to 4 (excluding index 5)
        let my_hand_of_animals = my_pool[0..5].to_vec();

        // Get a slice from index 5 to 16 (excluding index 17)
        let mut my_board_of_animals = my_pool[5..17].to_vec();

        let name_of_my_secret_animal = my_board_of_animals[0].name.to_string();
        my_board_of_animals.shuffle(&mut thread_rng());


        // Get a slice from index 17 to 20 (excluding index 21)
        let my_deck_of_animals = my_pool[17..21].to_vec();


        /*
        let my_hand_of_animals = vec![my_pool[0].clone(), my_pool[1].clone(), &my_pool[2].clone(), &my_pool[3].clone(),&my_pool[4].clone()];
        let my_board_of_animals = vec![my_pool[5], my_pool[6], my_pool[7],my_pool[8], my_pool[9], my_pool[10], my_pool[11],my_pool[12], my_pool[13], my_pool[14], my_pool[15], my_pool[16]];
        let my_deck_of_animals = vec![my_pool[17], my_pool[18], my_pool[19],my_pool[20], my_pool[21]];
        */

        /*
        let my_deck_of_animals = vec![lion, tiger, cobra, chameleon];
        let my_board_of_animals = vec![elephant, zebra, hippopotamus, rhinoceros, lama, colibri, panda, koala, komodo_dragon, seagull, penguin, walrus];
        let my_hand_of_animals = vec![iguana, crocodile, ostrich, greater_flamingo, toucan];
        */



        


        Game {
            deck: my_deck_of_animals, 
            board: my_board_of_animals.clone(),
            hand: my_hand_of_animals,
            name_of_secret_animal: name_of_my_secret_animal,
            unreached_targets: vec![1,2,6,9,11],
            //name_of_secret_animal: "walrus".to_string(),
            is_over: false
        }
    }



    pub fn is_over(&self) -> bool {
        if self.is_secret_animal_shot() {
            println!("secret animal was shot");
            return true ;
        }
        else {
            
            if (&self.board).len()==1{
                println!("only the secret animal left.");
                return true ;
            }
        }
        
        return false;

    }

    pub fn shoot_animal(&mut self, animal_name : String) {
        let mut index = 0;
        for animal in &self.board {
            if animal.name.trim() == animal_name.trim() {
                break;
                
            }else {
                index +=1;
            }
        }
        //println!("animal being shot : ");
        //println!("{}",&self.board[index].name);
        self.board.remove(index);
    }

    pub fn is_reached_target_number_of_eliminated_animals(&self) -> bool {
        //et unreached_targets = vec![1,2,6,9,11];
        //println!("{}",&self.board.len());
        self.unreached_targets.contains(&self.board.len())
        
    }

    pub fn pop_target(&mut self) {
        //self.unreached_targets.pop(&self.board.len());

        if let Some(index) = self.unreached_targets.iter().position(|value| *value == self.board.len()) {
            self.unreached_targets.swap_remove(index);
        }
    }

    pub fn request_hint(self) -> String {
        /*
        let mut rng = rand::thread_rng();
        let res = rng.gen_range(0..2);
        let mut mem = "";
        if res == 0 {
            mem = "(+)";
        }else {
            mem = "(-)";
        }
        mem.to_owned()+"iguana"
        */

        //let secret_animal: Animal;
        let mut secret_animal = &self.board[0].clone();
        for animal in &self.board {
            if animal.name == self.name_of_secret_animal{
                //secret_animal = animal.clone();
                secret_animal = animal;
            }
        }
        let mut similarity_between_hand_animals_and_secret_animal: Vec<i32> = Vec::new();
        let mut similarity_between_deck_animals_and_secret_animal: Vec<i32> = Vec::new();
        let mut abs_avg_diff_1: Vec<i32> = Vec::new();
        let mut abs_avg_diff_2: Vec<i32> = Vec::new();



        for animal in &self.hand {
            similarity_between_hand_animals_and_secret_animal.push(get_similarity_score(animal.clone(),secret_animal.clone()))
        }

        
        for animal in &self.deck {
            similarity_between_deck_animals_and_secret_animal.push(get_similarity_score(animal.clone(),secret_animal.clone()))
        }

        let average_hand = average(similarity_between_hand_animals_and_secret_animal.clone());
        let average_deck = average(similarity_between_deck_animals_and_secret_animal.clone());


        for value in &similarity_between_hand_animals_and_secret_animal {
            abs_avg_diff_1.push((value-average_hand).abs())
        }

        for value in &similarity_between_deck_animals_and_secret_animal {
            abs_avg_diff_2.push((value-average_deck).abs())
        }


        println!("{:?}",abs_avg_diff_1);

        println!("{:?}",abs_avg_diff_2);
        
        "".to_string()
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
        for animal in &self.board {
            //println!("{}", animal.name);
            //println!("{}", self.name_of_secret_animal);

            if animal.name.trim() == self.name_of_secret_animal.trim() {
                //println!("secret animal not shot yet");
                return false;
            }
        }
            //println!("secret animal shot yet");
        return true;
    }
    


}