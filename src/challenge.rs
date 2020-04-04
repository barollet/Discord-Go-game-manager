use std::collections::HashMap;

use serenity::model::id::UserId;
use serenity::prelude::*;

use crate::game::GameInfo;

// A hashmap with a list of current challenges (two instance per challenge)
#[derive(Default)]
pub struct ChallengeList(HashMap<UserId, Vec<UserId>>);

impl ChallengeList {
    // Add the corresponding challenge entries for the two given players
    // Technically, challenger and challengee can be interchanged
    // If the challenge already exists, nothing is done
    // Returns if a challenge was created
    pub fn insert(&mut self, challenger: UserId, challengee: UserId) -> bool {
        // We have to or the values to support self challenge
        self.insert_single_challenge(challenger, challengee)
            || self.insert_single_challenge(challengee, challenger)
    }
    fn insert_single_challenge(&mut self, player: UserId, other: UserId) -> bool {
        let challenges = self
            .0
            .entry(player)
            // Creates the vector if empty
            .or_insert_with(Vec::new);
        // This can be optimized
        if !challenges.contains(&other) {
            challenges.push(other);
            true
        } else {
            false
        }
    }

    // Remove a challenge between two players
    pub fn remove_challenge(&mut self, player: UserId, other: UserId) {
        self.remove_from_list(player, other);
        self.remove_from_list(other, player);
    }

    // Removes if present the given challenger from the player's challenge list
    // Does nothing if the player is not in the challenge list
    fn remove_from_list(&mut self, player_list: UserId, challenger: UserId) {
        let challenges = self.0.entry(player_list).or_insert_with(Vec::new);
        // remove the player from challenges
        if let Some(index) = challenges.iter().position(|&elem| elem == challenger) {
            challenges.swap_remove(index);
        }
    }

    // Removes all the challenges concerning the given player
    pub fn remove_all(&mut self, player: UserId) {
        // Remove the given player from the list and take its current challenge list
        let others = self.0.remove(&player).unwrap_or_default();

        for other in others {
            // Remove the current player from the other's challenge list
            self.remove_from_list(other, player);
        }
    }

    // List the current challenges for a player
    pub fn list(&self, player: UserId) -> Vec<UserId> {
        self.0.get(&player).cloned().unwrap_or_default()
    }

    // Removes a challenge from the list and returns a new game instance
    pub fn accept_challenge_to_game(&mut self, player: UserId, other: UserId) -> Option<GameInfo> {
        // If there is no challenge between the two players we returns nothing
        let game_info = self.0.get(&player).and_then(|challenge_list| {
            if challenge_list.contains(&other) {
                Some(GameInfo::from_challenge(player, other))
            } else {
                None
            }
        });

        if game_info.is_some() {
            self.remove_challenge(player, other);
        }

        game_info
    }
}

impl TypeMapKey for ChallengeList {
    // <Challengee, challenger>
    type Value = ChallengeList;
}
