mod game;

use game::{Move, Pokemon};

fn main() {
    let tackle = Move {
        name: String::from("Tackle"),
        move_type: String::from("Normal"),
        power: 40,
        accuracy: 100,
        pp: 35,
    };

    let ember = Move {
        name: String::from("Ember"),
        move_type: String::from("Fire"),
        power: 50,
        accuracy: 100,
        pp: 15,
    };

    let scary_face = Move {
        name: String::from("Scary Face"),
        move_type: String::from("Normal"),
        power: 0,
        accuracy: 100,
        pp: 5,
    };

    let growl = Move {
        name: String::from("Growl"),
        move_type: String::from("Normal"),
        power: 0,
        accuracy: 100,
        pp: 35,
    };

    let charmander = Pokemon {
        species: String::from("Charmander"),
        level: 5,
        hp: 20,
        attack: 10,
        defense: 9,
        speed: 12,
        current_hp: 20,
        moves: [tackle, ember, scary_face, growl],
    };

    println!(
        "{} (Lv. {}) - HP {}/{}",
        charmander.species, charmander.level, charmander.current_hp, charmander.hp
    );
}
