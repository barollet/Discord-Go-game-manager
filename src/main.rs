use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::*;

mod board;
mod challenge;
mod game;

use crate::board::Intersection;
use crate::challenge::*;
use crate::game::*;

#[group]
#[commands(ping, challenge, remove, list, accept, play)]
struct General;

use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
            .group(&GENERAL_GROUP),
    );

    {
        let mut data = client.data.write();
        data.insert::<ChallengeList>(ChallengeList::default());
        data.insert::<GameList>(GameList::default());
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
#[aliases(c)]
// Challenge a player to a game
fn challenge(ctx: &mut Context, msg: &Message) -> CommandResult {
    let opponents = &msg.mentions;
    let channel = msg.channel_id;

    if opponents.is_empty() {
        channel.say(
            ctx,
            "You have to mention some people to challenge.\nExample: ~challenge @SinJinseo @KeJie",
        )?;
    } else {
        // Acquire write lock
        {
            let mut data = ctx.data.write();
            let challenges = data.get_mut::<ChallengeList>().unwrap();

            // Create a challenge for each opponent
            for opponent in opponents {
                challenges.insert(msg.author.id, opponent.id);
            }
        }
        channel.say(ctx, "Challenges pending")?;
    }
    Ok(())
}

#[command]
#[aliases(r)]
// Removes the pending challenges of the author
fn remove(ctx: &mut Context, msg: &Message) -> CommandResult {
    let channel = msg.channel_id;
    {
        let mut data = ctx.data.write();
        let challenges = data.get_mut::<ChallengeList>().unwrap();

        challenges.remove_all(msg.author.id);
    }
    channel.say(ctx, "Your challenges are not pending anymore")?;
    Ok(())
}

#[command]
#[aliases(l)]
fn list(ctx: &mut Context, msg: &Message) -> CommandResult {
    let channel = msg.channel_id;
    let users = {
        let data = ctx.data.read();
        let challenges = data.get::<ChallengeList>().unwrap();

        challenges.list(msg.author.id)
    };

    let usernames: Vec<String> = users
        .iter()
        .map(|user_id| user_id.to_user(&ctx))
        .filter_map(Result::ok)
        .map(|user| user.name)
        .collect();

    if usernames.is_empty() {
        channel.say(ctx, "You have no current pending challenges")?;
    } else {
        channel.say(
            ctx,
            format!("You are currently challenging {}", usernames.join(", ")),
        )?;
    }

    Ok(())
}

#[command]
#[aliases(a)]
// Accept a pending challenge and starts a game in the current channel
fn accept(ctx: &mut Context, msg: &Message) -> CommandResult {
    let player = msg.author.id;
    let opponents = &msg.mentions;
    let channel = msg.channel_id;

    if opponents.len() != 1 {
        channel.say(ctx, "You can only accept a single challenge")?;
        return Ok(());
    }

    let opponent = opponents[0].id;

    // Checking that the channel is available
    let channel_available = {
        let data = ctx.data.read();
        // Creates the game
        let games = data.get::<GameList>().unwrap();
        games.is_channel_available(channel)
    };

    if !channel_available {
        channel.say(&ctx, "Game occupied")?;
        return Ok(());
    }

    let game_creation_success = {
        let mut data = ctx.data.write();
        let challenges = data.get_mut::<ChallengeList>().unwrap();
        let game = challenges.accept_challenge_to_game(player, opponent);

        // Adds the game to game list if the game is successfully created
        let games = data.get_mut::<GameList>().unwrap();
        game.map_or(false, |game_info| games.try_start_game(game_info, channel))
    };

    if game_creation_success {
        let game_message = channel.say(&ctx, "Game started")?;
        // Set game message
        let mut data = ctx.data.write();
        let games = data.get_mut::<GameList>().unwrap();
        games.set_game_message(channel, game_message.id);
    } else {
        channel.say(&ctx, "Game no")?;
    }

    Ok(())
}

#[command]
#[aliases(p)]
fn play(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let player = msg.author.id;
    let channel = msg.channel_id;

    let mut data = ctx.data.write();
    let games = data.get_mut::<GameList>().unwrap();

    // Checks that a game is played by you in this channel
    let game_info = games.is_game_played_in_channel_by(player, channel);
    let game_info = if game_info.is_none() {
        channel.say(&ctx, "You are not playing")?;
        return Ok(());
    } else {
        game_info.unwrap()
    };
    // Checks that it is your turn
    if !game_info.is_to_move(player) {
        channel.say(&ctx, "Not your move")?;
    }
    // Parse the move
    if args.len() != 1 {
        channel.say(
            &ctx,
            "You have to specify a single coordinate.\nExample: ~p k10",
        )?;
        return Ok(());
    }
    let intersection = args.parse::<Intersection>();
    let intersection = match intersection {
        Ok(intersection) => intersection,
        Err(_) => {
            channel.say(&ctx, "Cannot parse your move")?;
            return Ok(());
        }
    };
    // Plays the move
    let legal_move = game_info.board.play(intersection);
    if !legal_move {
        channel.say(&ctx, "Illegal move")?;
    } else {
        // If the move is legal we update the board
        // TODO update instead of printing
        channel.say(&ctx, format!("{}", game_info.board))?;
    }

    Ok(())
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong!")?;
    msg.reply(ctx, format!("coucou\n{}", crate::board::Board::new()))?;

    Ok(())
}
