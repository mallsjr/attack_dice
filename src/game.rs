use ansi_term::Colour::*;
use ansi_term::Style;

use crate::actions::ActionType;
use crate::player::RollOver;
use crate::print_blank_line;
use crate::{
    actions::Action,
    player::{Player, PlayerAction, PlayerRolls},
};

pub struct Game {
    pub complete: bool,
    pub player_one: Player,
    pub player_two: Player,
    pub round: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            complete: false,
            player_one: Player::new(),
            player_two: Player::new(),
            round: 1,
        }
    }

    pub fn play_round(&mut self) -> bool {
        //player_one rolls
        println!("{}", Style::new().bold().paint("Player One Roll"));
        let player_one_roll: PlayerRolls = (
            self.player_one.roll_dice(),
            self.player_one.roll_dice(),
            self.player_one.roll_dice(),
        );
        let player1_action =
            self.determine_player_action(player_one_roll, &self.player_one.roll_over);
        //player_two rolls
        println!("{}", Style::new().bold().paint("Player Two Roll"));
        let player_two_roll: PlayerRolls = (
            self.player_two.roll_dice(),
            self.player_two.roll_dice(),
            self.player_two.roll_dice(),
        );
        let player2_action =
            self.determine_player_action(player_two_roll, &self.player_two.roll_over);
        //calculate damage
        self.calculate_damage(player1_action, player2_action);
        self.round += 1;
        //determine if game completed if not update round number
        self.player_one.hp <= 0 || self.player_two.hp <= 0
    }

    fn calculate_damage(&mut self, player1_action: PlayerAction, player2_action: PlayerAction) {
        if player1_action.action == Action::Attack && player2_action.action == Action::Attack {
            //"Both players attack and lose health"
            println!(
                "{} {} {} {}",
                Red.paint("Player One deals"),
                Red.paint(player1_action.damage.to_string()),
                Red.paint(player1_action.action_type.to_string()),
                Red.paint("damage")
            );
            self.player_two.hp -= player1_action.damage;
            println!(
                "{} {} {} {}",
                Green.paint("Player Two deals"),
                Green.paint(player2_action.damage.to_string()),
                Green.paint(player2_action.action_type.to_string()),
                Green.paint("damage")
            );
            self.player_one.hp -= player2_action.damage;
            //"clear roll over"
            self.clear_rollover();
        } else if player1_action.action == Action::Defend && player2_action.action == Action::Defend
        {
            //"Both players defend numbers roll over to next round"
            self.player_one.roll_over.magical = player1_action.magical;
            self.player_one.roll_over.physical = player1_action.physical;
            self.player_two.roll_over.magical = player2_action.magical;
            self.player_two.roll_over.physical = player2_action.physical;
            println!(
                "{}",
                Blue.bold()
                    .paint("Both Players Defend. Damage values rollover to next round")
            );
        } else {
            //"one player is attacking and the other is defending"
            if player1_action.action == Action::Defend {
                // "Player one is defending so will lose HP if player two attack is different or higher"
                if player1_action.action_type == ActionType::Magical
                    && player2_action.action_type == ActionType::Magical
                    || player1_action.action_type == ActionType::Physical
                        && player2_action.action_type == ActionType::Physical
                {
                    if player2_action.damage > player1_action.damage {
                        //Player 2 rolled higher than player 1 so calculate damage
                        let damage = player2_action.damage - player1_action.damage;
                        self.player_one.hp -= damage;
                        println!(
                            "{} {} {} {}",
                            Green.paint("Player Two deals"),
                            Green.paint(damage.to_string()),
                            Green.paint(player2_action.action_type.to_string()),
                            Green.paint("damage")
                        );
                    } else {
                        println!("Player One has a higher defend");
                    }
                } else {
                    self.player_one.hp -= player2_action.damage;
                    println!(
                        "{} {} {} {}",
                        Green.paint("Player Two deals"),
                        Green.paint(player2_action.damage.to_string()),
                        Green.paint(player2_action.action_type.to_string()),
                        Green.paint("damage")
                    );
                }
            } else {
                //"Player two is defending so will lose HP if player one attack is different or higher"
                if player1_action.action_type == ActionType::Magical
                    && player2_action.action_type == ActionType::Magical
                    || player1_action.action_type == ActionType::Physical
                        && player2_action.action_type == ActionType::Physical
                {
                    if player1_action.damage > player2_action.damage {
                        //Player 2 rolled higher than player 1 so calculate damage
                        let damage = player1_action.damage - player2_action.damage;
                        self.player_two.hp -= damage;
                        println!(
                            "{} {} {} {}",
                            Red.paint("Player One deals"),
                            Red.paint(damage.to_string()),
                            Red.paint(player1_action.action_type.to_string()),
                            Red.paint("damage")
                        );
                    } else {
                        println!("Player Two has higher defend");
                    }
                } else {
                    self.player_two.hp -= player1_action.damage;
                    println!(
                        "{} {} {} {}",
                        Red.paint("Player One deals"),
                        Red.paint(player1_action.damage.to_string()),
                        Red.paint(player1_action.action_type.to_string()),
                        Red.paint("damage")
                    );
                }
            }
            self.clear_rollover();
        }
    }

    fn determine_player_action(
        &self,
        player_roll: PlayerRolls,
        rollover: &RollOver,
    ) -> PlayerAction {
        let (mut physical, mut magical, action) = player_roll;

        let player_action = match action % 2 {
            0 => Action::Attack,
            _ => Action::Defend,
        };

        println!(
            "{} Physical + {} rollover = {} ",
            physical,
            rollover.physical,
            physical + rollover.physical
        );
        println!(
            "{} Magical + {} rollover = {}",
            magical,
            rollover.magical,
            magical + rollover.magical
        );

        physical += rollover.physical;
        magical += rollover.magical;

        let (player_action_type, player_damage) = match magical > physical {
            true => (ActionType::Magical, magical),
            false => (ActionType::Physical, physical),
        };

        println!(
            "{}: {} {}",
            Style::new().bold().paint("Player Action"),
            Style::new().bold().paint(player_action.to_string()),
            Style::new().bold().paint(player_action_type.to_string())
        );
        print_blank_line();

        PlayerAction {
            action: player_action,
            action_type: player_action_type,
            damage: player_damage,
            magical,
            physical,
        }
    }

    fn clear_rollover(&mut self) {
        self.player_one.roll_over = RollOver::default();
        self.player_two.roll_over = RollOver::default();
    }
}
