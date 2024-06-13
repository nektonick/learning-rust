use std::fs::File;
use std::io::{BufReader, BufWriter};

use dialoguer::Confirm;
use serde::{Deserialize, Serialize};

use crate::render;
use crate::render::Render;

type Damage = i32;

#[derive(Serialize, Deserialize)]
pub struct Character {
    name: String,
    hp: i32,
    attack: Damage,
    available_actions_count: u32,
}

impl Character {
    pub fn new(name: String) -> Character {
        let default_hp = 5;
        let default_attack = 1;
        let default_available_actions_count = 1;
        Character {
            name,
            hp: default_hp,
            attack: default_attack,
            available_actions_count: default_available_actions_count,
        }
    }

    // Note: I split attack and take_damage
    // because of self.attack(self) borrowing issue
    pub fn attack(&mut self) -> Damage {
        if self.available_actions_count == 0 {
            println!("No actions left!");
            return 0;
        }
        self.available_actions_count -= 1;
        self.attack
    }

    pub fn take_damage(&mut self, damage: Damage) {
        self.hp -= damage;
    }
}

impl render::Render for Character {
    fn render(&self) {
        let name = &self.name;
        let hp = &self.hp;
        let actions = &self.available_actions_count;
        println!("{name}:");
        println!("  {hp} hp left");
        println!("  {actions} actions left");
    }
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    my_character: Character,
    current_game_level: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            my_character: Character::new(String::from("Nik")),
            current_game_level: 1,
        }
    }

    pub fn load() -> Result<Game, std::io::Error> {
        let default_save_file = Game::get_save_file_name();
        let file = File::open(default_save_file)?;
        let reader = BufReader::new(file);
        let game: Game = serde_json::from_reader(reader)?;
        Ok(game)
    }

    fn get_save_file_name() -> String {
        String::from("save.json")
    }

    pub fn save(game: Game) -> Result<(), std::io::Error> {
        let default_save_file = Game::get_save_file_name();
        let file = File::open(default_save_file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &game)?;
        Ok(())
    }

    pub fn play(&mut self) {
        // TODO: load correct level
        level_0(&mut self.my_character);
    }
}

pub struct GameLevel {
    number: u32,
    script: fn(main_character: &mut Character) -> (),
}

fn level_0(main_character: &mut Character) {
    println!("Ты очнулся возле грязной лужи.");
    println!("Из лужи на тебя кто-то смотрит...");

    let attack = Confirm::new()
        .with_prompt("Атаковать?")
        .default(true)
        .interact()
        .unwrap();

    if attack {
        println!("Упс, кажется это был ты");
        attack_self(main_character);

        println!("******");
        main_character.render();
        println!("******");
    }
}

fn attack_self(character: &mut Character) {
    let damage = character.attack();
    character.take_damage(damage);
}
