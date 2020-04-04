use std::collections::HashMap;

use serenity::model::id::{ChannelId, MessageId, UserId};
use serenity::prelude::*;

use crate::board::Board;

// Storing the current state of a game
pub struct GameInfo {
    pub board: Board,
    players: [UserId; 2],
}
impl GameInfo {
    pub fn from_challenge(player: UserId, other: UserId) -> Self {
        Self {
            board: Board::new(),
            players: [player, other],
        }
    }

    // Returns if a player is playing the current game
    pub fn is_player(&self, player: UserId) -> bool {
        self.players[0] == player || self.players[1] == player
    }

    pub fn is_to_move(&self, player: UserId) -> bool {
        self.players[self.board.to_move as usize] == player
    }
}

pub struct Game {
    game_info: Box<GameInfo>, // We are using a pointer, it feels like it should help
    channel: ChannelId,
    message: MessageId,
}

// A hashmap with a list of current games
#[derive(Default)]
pub struct GameList {
    games: HashMap<ChannelId, Game>, // There can be a single game per channel
    players: HashMap<UserId, Vec<ChannelId>>, // A player can play in multiple channels
}

impl TypeMapKey for GameList {
    type Value = GameList;
}

impl Game {
    fn new(game_info: GameInfo, channel: ChannelId) -> Self {
        Self {
            game_info: Box::new(game_info),
            channel,
            message: MessageId(0),
        }
    }
}

impl GameList {
    // Tries to create a new game in the given channel, returns if creation is a success
    pub fn try_start_game(&mut self, game_info: GameInfo, channel: ChannelId) -> bool {
        if !self.games.contains_key(&channel) {
            // If there is no game in the current channel then add the current one
            self.push_player_channel(game_info.players[0], channel);
            self.push_player_channel(game_info.players[1], channel);
            self.games.insert(channel, Game::new(game_info, channel));

            true
        } else {
            false
        }
    }

    pub fn is_channel_available(&self, channel: ChannelId) -> bool {
        !self.games.contains_key(&channel)
    }

    pub fn set_game_message(&mut self, channel: ChannelId, message: MessageId) {
        if let Some(game) = self.games.get_mut(&channel) {
            game.message = message;
        }
    }

    fn push_player_channel(&mut self, player: UserId, channel: ChannelId) {
        self.players
            .entry(player)
            .or_insert_with(Vec::new)
            .push(channel);
    }

    // Remove the game of the current channel from the list
    pub fn end_game(&mut self, channel: ChannelId) {
        // Remove the game from the list
        let game = self.games.remove(&channel);
        if let Some(game) = game {
            let [player, other] = game.game_info.players;
            // Remove the game from the two players list
            self.remove_channel_from_player(channel, player);
            self.remove_channel_from_player(channel, other);
        }
    }

    fn remove_channel_from_player(&mut self, channel: ChannelId, player: UserId) {
        if let Some(channels) = self.players.get_mut(&player) {
            if let Some(index) = channels.iter().position(|&elem| elem == channel) {
                channels.swap_remove(index);
            }
        }
    }

    pub fn is_game_played_in_channel_by(
        &mut self,
        player: UserId,
        channel: ChannelId,
    ) -> Option<&mut GameInfo> {
        self.games.get_mut(&channel).and_then(|game| {
            if game.game_info.is_player(player) {
                Some(game.game_info.as_mut())
            } else {
                None
            }
        })
    }
}
