use std::io::stdin;

mod game;



fn main () {
    let mut my_game = game::Game::new();

    // as long as the game is not over
    while !&my_game.is_over() {

        println!("current content of the board ");
        for el in &my_game.board {
            print!("{} ", el.name);
        }
        println!("");
        println!("choose an animal to eliminate : ");


        // ask the player for an animal
        let mut shoot_candidate = String::new();
        
        let b1 = std::io::stdin().read_line(&mut shoot_candidate).unwrap();

        //println!("{}", shoot_candidate);

        if game::Game::is_animal_on_board(&mut my_game, shoot_candidate.clone()) {

            // shoot the animal chosen
            game::Game::shoot_animal(&mut my_game, shoot_candidate);
        }else{
            println!("this animal is not on the board");
        }

    }
}