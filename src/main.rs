mod actions;
mod game;
mod player;

use crate::game::Game;
use ansi_term::Colour::*;
use ansi_term::Style;

fn main() {
    let mut game: Game = Game::new();

    print_title();

    while !game.complete {
        before_round(&game);
        game.complete = game.play_round();
        after_round(&game);
    }

    display_winner(&game);
}

fn print_blank_line() {
    println!("{}", format!(""));
}

fn print_title() {
    print_blank_line();
    println!("{}", Style::new().blink().bold().paint("ATTACK DICE GAME"));
    print_blank_line();
}

fn before_round(game: &Game) {
    println!(
        "{} {}",
        Style::new().bold().paint("Round"),
        Style::new().bold().paint(game.round.to_string())
    );
    println!(
        "{} {}",
        Red.paint("Player One HP"),
        Red.paint(game.player_one.hp.to_string())
    );
    println!(
        "{} {}",
        Green.paint("Player Two HP"),
        Green.paint(game.player_two.hp.to_string())
    );
    print_blank_line();
}

fn after_round(game: &Game) {
    print_blank_line();
    println!("{}", Style::new().bold().paint("HP after round"));
    println!(
        "{} {}",
        Red.paint("Player One HP"),
        Red.paint(game.player_one.hp.to_string())
    );
    println!(
        "{} {}",
        Green.paint("Player Two HP"),
        Green.paint(game.player_two.hp.to_string())
    );

    print_blank_line();
    println!("---------------");
    print_blank_line();
}

fn display_winner(game: &Game) {
    if game.player_one.hp <= 0 && game.player_two.hp > 0 {
        println!("{}", Green.bold().paint("Player Two Wins"));
    } else if game.player_two.hp <= 0 && game.player_one.hp > 0 {
        println!("{}", Red.bold().paint("Player One Wins"));
    } else if game.player_one.hp <= 0 && game.player_two.hp <= 0 {
        println!(
            "{}",
            Blue.bold().paint("No one wins players killed each other")
        );
    } else {
        panic!(
            "How did this happen? Player One HP: {}, Player Two HP {}",
            game.player_one.hp, game.player_two.hp
        );
    }
    print_blank_line();
}
