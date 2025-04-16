use std::io::stdin;

mod game;


fn main () {
    let my_game = game::Game::new();

    while !&my_game.is_over() {

        let mut shoot_candidate = String::new();
        println!("Animal to eliminate : ");
        let b1 = std::io::stdin().read_line(&mut shoot_candidate).unwrap();


        game::Game::shoot_animal(&my_game, shoot_candidate);




    }
}