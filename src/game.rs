mod pool_of_animals;
//use crate pool_of_animals::PoolOfAnimals;

pub struct Game {
    pool_of_animals: pool_of_animals::PoolOfAnimals,
}

impl Game {
    fn value(&self) -> &pool_of_animals::PoolOfAnimals {
        &self.pool_of_animals
    }

    pub fn new() -> Game {
        Game {pool_of_animals: pool_of_animals::PoolOfAnimals::fill()}
    }
}