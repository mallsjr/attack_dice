use std::collections::HashMap;

use ansi_term::Colour::*;
use ansi_term::Style;
use rand::Rng;

use crate::actions::ActionType;
use crate::player::RollOver;
use crate::print_blank_line;
use crate::{
    actions::Action,
    player::{Player, PlayerAction, PlayerRolls},
};

/**
 * How this works:
Challenge someone via tagging / @-ing. The dice roll command will not work if you tag someone in the same post. If there's enough players perhaps a tournament / king of the hill mode is possible.
The players both have 30HP at the start of the duel and will end once one player's HP reaches zero. The challenger rolls first and keeps tally of the game. These are negotiable.
To determine your action, roll a 6-sided dice thrice or input [!roll 1,6,3] without the brackets [ ] to determine your action. The opponent follows suit.
Roll result: Physical, Magical, Action
Physical determines your physical attack / defense
Magical determines your magical attack / defense.
Action will determine what you do for the turn, odd numbers(1,3,5) will allow you to perform an attack while even numbers(2,4,6) will have you on the defensive.
An attack is done if the Action roll is an odd number(1,3,5). Attacking inflicts damage to your opponent and is classified as Physical or Magical.
The higher value between Physical and Magical will determine what type of attack is performed and the attack value. (Ex: [Roll result: 3, 4, 1] Magical attack is higher so a Magical attack of 4 is performed.)
Critical Attack: If the roll gives 2 of the same number, attack value is doubled(highest value) and tripled if every number's the same. Cannot be blocked normally. This only applies on the latest roll after reroll additions.
Defending is done if the Action roll is an even number(2,4,6). Defending blocks the corresponding classification of attack.
Physical defense only blocks Physical attacks while Magical defense only blocks Magical attacks.
If an attack exceeds the defensive value, the difference is dealt as damage.
Stalwart Defense: If the roll gives 2 of the same number, defensive values are doubled and tripled if every number's the same. If the defense is higher than the attack, the difference is added to the HP as recovery. Stalwart Defense can block critical attack. This only applies on the latest roll after reroll additions.
If both players get odd Action rolls, both players will perform an attack and take damage.
If both players get even Action rolls, both players will roll again but the Physical values and Magical values will be added to the next roll respectively.
*/

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
        let player_one_rolls = self.roll_dice();
        let player1_action =
            self.determine_player_action(player_one_rolls, &self.player_one.roll_over);
        //player_two rolls
        println!("{}", Style::new().bold().paint("Player Two Roll"));
        let player_two_rolls = self.roll_dice();
        let player2_action =
            self.determine_player_action(player_two_rolls, &self.player_two.roll_over);
        //calculate damage
        self.calculate_damage(player1_action, player2_action);
        self.round += 1;
        //determine if game completed if not update round number
        self.is_game_complete()
    }

    fn is_game_complete(&self) -> bool {
        self.player_one.hp <= 0 || self.player_two.hp <= 0
    }

    fn roll_dice(&self) -> PlayerRolls {
        (roll(), roll(), roll())
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
                if player1_action.stalwart_defend && player2_action.critical_attack {
                    println!("Player 1 Stalwart Defend, Player 2 Critical Attack");
                    if player2_action.damage > player1_action.damage {
                        self.player_one.hp -= player2_action.damage - player1_action.damage;
                    } else {
                        self.player_one.hp += player1_action.damage - player2_action.damage;
                    }
                } else if player1_action.stalwart_defend && !player2_action.critical_attack {
                    println!("Player 1 Stalwart Defend, Player 2 Normal Attack");
                    if player1_action.damage > player2_action.damage {
                        self.player_one.hp += player1_action.damage - player2_action.damage;
                    } else {
                        self.player_one.hp -= player2_action.damage - player1_action.damage;
                    }
                } else if !player1_action.stalwart_defend && player2_action.critical_attack {
                    println!("Player 1 Normal Defend, Player 2 Critical Attack");
                    self.player_one.hp -= player2_action.damage;
                } else if !player1_action.stalwart_defend && !player2_action.critical_attack {
                    println!("Player 1 Normal Defend, Player 2 Normal Attack");
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
                }
            } else {
                //"Player two is defending so will lose HP if player one attack is different or higher"
                if player2_action.stalwart_defend && player1_action.critical_attack {
                    println!("Player 2 Stalwart Defend, Player 1 Critical Attack");
                    if player1_action.damage > player2_action.damage {
                        self.player_two.hp -= player1_action.damage - player2_action.damage;
                    } else {
                        self.player_two.hp += player2_action.damage - player1_action.damage;
                    }
                } else if player2_action.stalwart_defend && !player1_action.critical_attack {
                    println!("Player 2 Stalwart Defend, Player 1 Normal Attack");
                    if player2_action.damage > player1_action.damage {
                        self.player_two.hp += player2_action.damage - player1_action.damage;
                    } else {
                        self.player_two.hp -= player1_action.damage - player2_action.damage;
                    }
                } else if !player2_action.stalwart_defend && player1_action.critical_attack {
                    println!("Player 2 Normal Defend, Player 1 Critical Attack");
                    self.player_two.hp -= player1_action.damage;
                } else if !player2_action.stalwart_defend && !player1_action.critical_attack {
                    println!("Player 2 Normal Defend, Player 1 Normal Attack");
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
            }
            self.clear_rollover();
        }
    }

    fn determine_player_action(
        &self,
        player_rolls: PlayerRolls,
        rollover: &RollOver,
    ) -> PlayerAction {
        let mut critical_attack = false;
        let mut stalwart_defend = false;

        let (mut physical, mut magical, action) = player_rolls;

        let player_action = match action % 2 {
            0 => Action::Attack,
            _ => Action::Defend,
        };

        let (player_action_type, player_damage) = match magical > physical {
            true => {
                if check_for_triples(vec![physical, magical, action]) {
                    println!("Rolled triple! Do something here later: {:?}", player_rolls);
                    if player_action == Action::Attack {
                        critical_attack = true
                    } else {
                        stalwart_defend = true;
                    }
                } else {
                    magical = match check_for_doubles(vec![physical, magical, action]) {
                        true => {
                            println!("Rolled doubles! Doubling magical value: {:?}", player_rolls);
                            magical * 2
                        }
                        false => magical,
                    };
                }
                (ActionType::Magical, magical)
            }
            false => {
                if check_for_triples(vec![physical, magical, action]) {
                    println!("Rolled triple! Do something here later: {:?}", player_rolls);
                    if player_action == Action::Attack {
                        critical_attack = true
                    } else {
                        stalwart_defend = true;
                    }
                } else {
                    physical = match check_for_doubles(vec![physical, magical, action]) {
                        true => {
                            println!(
                                "Rolled doubles! Doubling physical value: {:?}",
                                player_rolls
                            );
                            physical * 2
                        }
                        false => physical,
                    };
                }

                (ActionType::Physical, physical)
            }
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
            critical_attack,
            stalwart_defend,
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

fn roll() -> isize {
    let mut rng = rand::thread_rng();
    let random_number: isize = rng.gen_range(1..=6);
    random_number
}

fn check_for_triples(values: Vec<isize>) -> bool {
    let mut count_map = HashMap::new();

    for item in values {
        let count = count_map.entry(item).or_insert(0);
        *count += 1;
    }

    count_map.values().any(|&count| count == 3)
}

fn check_for_doubles(values: Vec<isize>) -> bool {
    let mut count_map = HashMap::new();

    for item in values {
        let count = count_map.entry(item).or_insert(0);
        *count += 1;
    }

    count_map.values().any(|&count| count == 2)
}
