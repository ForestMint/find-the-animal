use std::io::stdin;

mod game;


fn main () {
    let my_game = game::Game::new();

    while !&my_game.is_over() {

        let mut line = String::new();
        println!("Enter your name :");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        println!("Hello , {}", line);
        println!("no of bytes read , {}", b1);





        let shoot_candidate = "walrus".to_string();





        game::Game::shoot_animal(&my_game, shoot_candidate);




    }
}