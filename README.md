# Notes

``` rust
mod vec2;
mod util;
mod collision;
mod message;
mod network;
mod server;
mod events;
mod network_manager;
mod server_manager;
mod client_manager;
mod util;

use collision::Rectangle;
use vec2::Vec2;

use std::rc::Rc;

pub struct Skill {
    name: String,
    id: usize,
    cost: i32,
    cooldown: i32,
}

pub struct Spell {
    skill: Rc<Skill>,
    rectangle: Rectangle
}

pub struct Character {
    name: String,
    skills: Vec<Rc<Skill>>,
}


pub struct Entity {
    character: Rc<Character>,
    rectangle: Rectangle,
    live: i32,
    max_live: i32,
    energy: i32,
    max_energy: i32,
}

pub struct Player {
    id: usize,
    entity: Option<Rc<Entity>>,
    points: usize,
}

pub struct Wall {
    rectangles: Vec<Rectangle>,
}

pub struct Map {
    dimension: Vec2,
    obstacles: Vec<Wall>,
}

pub struct Scene {
    entities: Vec<Rc<Entity>>,
    spells: Vec<Spell>,
    map: Map,
}

pub struct Arena {
    scene: Scene,
}

pub struct Game {
    players: Vec<Player>,
    arena: Arena,
    win_points: usize,
}



use crate::Character;
use crate::Skill;

use std::collections::HashMap;
use std::rc::Rc;

struct Connection {
}

struct PlayerSession {
    id: usize,
    character: Rc<Character>,
    connection: Connection,
}

struct Server {
    skill: HashMap<String, Rc<Skill>>,
    users: HashMap<usize, PlayerSession>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            skill: HashMap::new(), //TODO: initialize
            users: HashMap::new(),
        }
    }

}
```