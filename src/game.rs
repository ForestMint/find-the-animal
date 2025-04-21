mod pool_of_animals;

mod animal;

pub struct Game {
    deck: pool_of_animals::PoolOfAnimals,
    board: pool_of_animals::PoolOfAnimals,
    hand: pool_of_animals::PoolOfAnimals,
    name_of_secret_animal: String,
    is_over: bool,
}

impl Game {

    pub fn new() -> Game {

        let my_deck_of_animals = pool_of_animals::PoolOfAnimals::fill();
        let my_board_of_animals = pool_of_animals::PoolOfAnimals::pick(&my_deck_of_animals, 12);
        let my_hand_of_animals = pool_of_animals::PoolOfAnimals::pick(&my_deck_of_animals, 5);

        


        Game {
            deck: my_deck_of_animals, 
            board: my_board_of_animals,
            hand: my_hand_of_animals,
            name_of_secret_animal: "walrus".to_string(),
            is_over: false
        }
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    pub fn shoot_animal(&self, animal : String) {

    }

    pub fn is_secret_animal_shot(&self) -> bool {
        let mut secret_animal_found = false;
        for animal in &self.board.vector_of_animals {
            if (animal.name == self.name_of_secret_animal) {
                secret_animal_found = true
            }
        }
        !secret_animal_found
    }


}