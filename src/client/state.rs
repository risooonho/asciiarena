use super::server_proxy::{ConnectionStatus};
use super::configuration::{Config};

use crate::version::{Compatibility};
use crate::message::{LoginStatus, EntityData, SpellData, Terrain};
use crate::character::{CharacterId, Character};
use crate::direction::{Direction};
use crate::vec2::{Vec2};
use crate::ids::{EntityId, SpellId};

use std::net::{SocketAddr};
use std::time::{Instant};
use std::collections::{HashMap};

pub struct User {
    pub character_symbol: Option<char>,
    pub login_status: Option<LoginStatus>,
}

impl User {
    pub fn is_logged(&self) -> bool {
        if let Some(LoginStatus::Logged(..)) = self.login_status {
            return true
        }
        false
    }
}

pub struct VersionInfo {
    pub version: String,
    pub compatibility: Compatibility,
}

pub struct StaticGameInfo {
    pub players_number: usize,
    pub map_size: usize,
    pub winner_points: usize,
}

pub struct UserPlayer {
    pub player_id: usize, // The position of server.arena.players Vec.
    pub direction: Direction,
}

pub struct Arena {
    pub user_player: UserPlayer,
    pub entities: HashMap<EntityId, EntityData>,
    pub spells: HashMap<SpellId, SpellData>,
    pub size: usize,
    pub ground: Vec<Terrain>,
}

impl Arena {
    pub fn terrain(&self, position: Vec2) -> Terrain {
        assert!(position.x >= 0 && position.x < self.size as i32);
        assert!(position.y >= 0 && position.y < self.size as i32);
        self.ground[position.y as usize* self.size + position.x as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStatus {
    NotStarted,
    Started,
    Finished,
}

pub struct Player {
    pub id: usize, // The position of server.arena.players Vec.
    pub character_id: CharacterId,
    pub entity_id: EntityId,
    pub points: usize,
}

pub struct Game {
    pub status: GameStatus,
    pub next_arena_timestamp: Option<Instant>,
    pub arena_number: usize,
    pub arena: Option<Arena>,
    pub characters: HashMap<CharacterId, Character>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn arena(&self) -> &Arena {
        self.arena.as_ref().unwrap()
    }

    pub fn arena_mut(&mut self) -> &mut Arena {
        self.arena.as_mut().unwrap()
    }
}

pub struct Server {
    pub addr: Option<SocketAddr>,
    pub connection_status: ConnectionStatus,
    pub udp_port: Option<u16>,
    pub udp_confirmed: Option<bool>,
    pub version_info: Option<VersionInfo>,
    pub game_info: Option<StaticGameInfo>,
    pub logged_players: Vec<char>,
    pub game: Game,
}

impl Server {
    pub fn is_full(&self) -> bool {
        if let Some(StaticGameInfo {players_number, .. }) = self.game_info {
            if players_number == self.logged_players.len() {
                return true
            }
        }
        false
    }

    pub fn is_connected(&self) -> bool {
        match self.connection_status {
            ConnectionStatus::Connected => true,
            _ => false,
        }
    }

    pub fn game_info(&self) -> &StaticGameInfo {
        self.game_info.as_ref().unwrap()
    }

    pub fn has_compatible_version(&self) -> bool {
        if let Some(version_info) = &self.version_info {
            return version_info.compatibility.is_compatible()
        }
        false
    }
}

pub struct State {
    pub user: User,
    pub server: Server,
}

impl State {
    pub fn new(config: &Config) -> State {
        State {
            user: User {
                character_symbol: config.character,
                login_status: None,
            },
            server: Server {
                addr: config.server_addr,
                connection_status: ConnectionStatus::NotConnected,
                udp_port: None,
                udp_confirmed: None,
                version_info: None,
                game_info: None,
                logged_players: Vec::new(),
                game: Game {
                    status: GameStatus::NotStarted,
                    arena_number: 0,
                    next_arena_timestamp: None,
                    arena: None,
                    characters: HashMap::new(),
                    players: Vec::new()
                },
            },
        }
    }
}
