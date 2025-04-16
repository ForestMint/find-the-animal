mod pool_of_animals;

pub struct Game {
    pool_of_animals: pool_of_animals::PoolOfAnimals,
}

impl Game {

    pub fn new() -> Game {
        Game {pool_of_animals: pool_of_animals::PoolOfAnimals::fill()}
    }
    
}