use std::io::stdin;

mod game;



fn main () {
    let mut my_game = game::Game::new();
    let mut hint_animal = "";

    let binding = &my_game.clone().request_hint();
    hint_animal = &binding;
    game::Game::discard_from_hand(&mut my_game, hint_animal.to_string());
    println!("{}",hint_animal);

    while true {

        while !&my_game.clone().is_reached_target_number_of_eliminated_animals() {






            println!("current content of the board ");
            /*
            for el in &my_game.board {
                print!("{} ", el.name);
            }
            */
            println!("");
            println!("choose an animal to eliminate : ");


            // ask the player for an animal
            let mut shoot_candidate = String::new();
            
            let b1 = std::io::stdin().read_line(&mut shoot_candidate).unwrap();

            //println!("{}", shoot_candidate);

            if game::Game::is_animal_on_board(&mut my_game, shoot_candidate.clone()) {

                // shoot the animal chosen
                game::Game::shoot_animal(&mut my_game, shoot_candidate);
                if my_game.is_over() {
                    std::process::exit(0)
                }

            }else{
                println!("this animal is not on the board");
            }
                
            
            //std::process::exit(0)
        }
        if my_game.clone().is_reached_target_number_of_eliminated_animals(){
            let binding = &my_game.clone().request_hint();
            hint_animal = &binding;
            game::Game::discard_from_hand(&mut my_game, hint_animal.to_string());
            println!("{}",hint_animal);
            //println!("{}",my_game.clone().request_hint());
            my_game.pop_target()
        }

        println!("current content of the board ");
        /*
        for el in &my_game.board {
            print!("{} ", el.name);
        }
        */
        println!("");
        println!("choose an animal to eliminate : ");


        // ask the player for an animal
        let mut shoot_candidate = String::new();
        
        let b1 = std::io::stdin().read_line(&mut shoot_candidate).unwrap();

        //println!("{}", shoot_candidate);

        if game::Game::is_animal_on_board(&mut my_game, shoot_candidate.clone()) {

            // shoot the animal chosen
            game::Game::shoot_animal(&mut my_game, shoot_candidate);
            if my_game.is_over() {
                std::process::exit(0)
            }

        }else{
            println!("this animal is not on the board");
        }
    }
}