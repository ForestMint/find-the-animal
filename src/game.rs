mod pool_of_animals;

pub struct Game {
    pool_of_animals: pool_of_animals::PoolOfAnimals,
    is_over: bool,
}

impl Game {

    pub fn new() -> Game {
        Game {pool_of_animals: pool_of_animals::PoolOfAnimals::fill(), is_over: false}
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    pub fn shoot_animal(&self, animal : String) {

    }

}