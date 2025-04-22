use std::io::stdin;

mod game;



fn main () {
    let mut my_game = game::Game::new();

    while !my_game.is_over() {

        println!("current content of the board ");
        for el in &my_game.board {
            print!("{} ", el.name);
        }

        if my_game.is_reached_target_number_of_eliminated_animals() {

            println!("{}",my_game.clone().request_hint());
        }

        
        let mut candidate = "NULL";
        let mut shoot_candidate = String::new();

        while !game::Game::is_animal_on_board(&mut my_game, candidate.to_string()) {
            // ask the player for an animal
            
            
            let b1 = std::io::stdin().read_line(&mut shoot_candidate).unwrap();   
            
            let candidate = &shoot_candidate.clone();
        }

        game::Game::shoot_animal(&mut my_game, shoot_candidate);


        /*
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
        */
    }

    /*

    println!("{}",my_game.clone().request_hint());

    while true {

        while !my_game.is_reached_target_number_of_eliminated_animals() {






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
                if my_game.is_over() {
                    std::process::exit(0)
                }

            }else{
                println!("this animal is not on the board");
            }
                
            
            //std::process::exit(0)
        }
        println!("{}",my_game.clone().request_hint());

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
            if my_game.is_over() {
                std::process::exit(0)
            }

        }else{
            println!("this animal is not on the board");
        }
    }

    */
}