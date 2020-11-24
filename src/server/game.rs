mod player;
mod arena;
mod entity;
mod map;

use player::{Player};
use arena::{Arena};

use crate::character::{Character, CharacterId, CharacterBuilder};

use std::collections::{HashMap, HashSet};

use std::rc::{Rc};

pub struct Game {
    map_size: usize,
    winner_points: usize,

    arena_number: usize,
    arena: Option<Arena>,

    characters: HashMap<CharacterId, Rc<Character>>,
    next_character_id: usize,

    players: HashMap<char, Player>,
}

impl Game {
    pub fn new(
        map_size: usize,
        winner_points: usize,
        player_characters: impl Iterator<Item = char>)
        -> Game
    {
        let mut next_character_id = 0;
        let characters = player_characters
            .map(|symbol| {
                let character = CharacterBuilder::default()
                    .id(next_character_id)
                    .symbol(symbol)
                    .max_live(Player::MAX_LIFE)
                    .max_energy(Player::MAX_ENERGY)
                    .speed_base(Player::SPEED_BASE)
                    .build()
                    .unwrap();

                let entry = (next_character_id, Rc::new(character));
                next_character_id += 1;
                entry
            })
            .collect::<HashMap<_, _>>();

        let players = characters
            .values()
            .map(|character|{
                (character.symbol(), Player::new(character.clone()))
            })
            .collect();

        Game {
            map_size,
            winner_points,
            arena_number: 0,
            arena: None,
            players,
            characters,
            next_character_id,
        }
    }

    pub fn arena(&self) -> Option<&Arena> {
        self.arena.as_ref()
    }

    pub fn arena_mut(&mut self) -> Option<&mut Arena> {
        self.arena.as_mut()
    }

    pub fn player(&self, character_symbol: char) -> Option<&Player> {
        self.players.get(&character_symbol)
    }

    pub fn player_mut(&mut self, character_symbol: char) -> Option<&mut Player> {
        self.players.get_mut(&character_symbol)
    }

    pub fn arena_number(&self) -> usize {
        self.arena_number
    }

    pub fn pole(&self) -> Vec<&Player> {
        let mut sorted_players = self.players.values().collect::<Vec<_>>();

        sorted_players.sort_by(|a, b| b.total_points().partial_cmp(&a.total_points()).unwrap());
        sorted_players
    }

    pub fn create_new_arena(&mut self) {
        let mut arena = Arena::new(self.map_size, self.players.len());

        for (index, player) in self.players.values_mut().enumerate() {
            let position = arena.map().initial_position(index);
            let entity = arena.create_entity(player.character().clone(), position);
            player.attach_entity(entity.id());
            player.reset_partial_points();
        }

        self.arena = Some(arena);
        self.arena_number += 1;
    }

    pub fn step(&mut self) {
        let living_players_before = self.living_players();

        self.arena.as_mut().unwrap().update();

        let living_players_after = self.living_players();
        let death_players = living_players_before.difference(&living_players_after);

        let player_number = self.players.len();
        for symbol in death_players {
            let player = self.players.get_mut(symbol).unwrap();
            player.update_points(player_number - living_players_before.len());
        }
    }

    pub fn living_players(&self) -> HashSet<char> {
        self.players
            .values()
            .filter(|player| !player.is_dead())
            .map(|player| player.character().symbol())
            .collect()
    }

    pub fn has_finished(&self) -> bool {
        self.players
            .values()
            .find(|&player| player.total_points() >= self.winner_points)
            .is_some()
    }
}
