mod game;

fn request_shoot() -> String {
    "walrus".to_string()
}

fn main () {
    let my_game = game::Game::new();

    while !my_game.is_over() {
        let shoot_candidate = request_shoot();
        my_game.shoot_animal(shoot_candidate);
    }
}