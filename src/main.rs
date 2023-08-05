use std::io;
use rand::Rng;

struct Player {
    name: String,
    agent: String,
    credits: u32,
    health: u32,
    armor: u32,
    position: (i32, i32),
    weapon: String,
}

impl Player {
    fn take_damage(&mut self, damage: u32) {
        let total_damage = damage.saturating_sub(self.armor);
        if total_damage > 0 {
            self.health = self.health.saturating_sub(total_damage);
        }
    }

    fn move_player(&mut self, x: i32, y: i32) {
        self.position = (self.position.0 + x, self.position.1 + y);
    }

    fn shoot(&self, target: &mut Player) {
        let damage: u32 = match self.weapon.as_str() {
            "Pistol" => rand::thread_rng().gen_range(10..=20),
            "Rifle" => rand::thread_rng().gen_range(20..=30),
            "Shotgun" => rand::thread_rng().gen_range(5..=15),
            _ => 0,
        };

        target.take_damage(damage);
        println!("{} shot {} for {} damage.", self.name, target.name, damage);
    }

    fn purchase_weapon(&mut self) {
        println!("Available weapons:");
        let weapons = vec!["Pistol", "Rifle", "Shotgun"];
        for (idx, weapon) in weapons.iter().enumerate() {
            println!("{}. {}", idx + 1, weapon);
        }

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: usize = choice.trim().parse().unwrap_or(0);

        if choice < 1 || choice > weapons.len() {
            println!("Invalid choice!");
        } else {
            let weapon = weapons[choice - 1];
            let weapon_cost = if weapon == "Rifle" { 100 } else { 50 };

            if self.credits >= weapon_cost {
                self.weapon = weapon.to_string();
                self.credits -= weapon_cost;
                println!("You purchased a {}.", weapon);
            } else {
                println!("Not enough credits to purchase the weapon.");
            }
        }
    }

    fn shoot_attack(&mut self, target: &mut Player) {
        self.shoot(target);
    }

    fn move_attack(&mut self, target: &mut Player) {
        let move_direction = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let choice = rand::thread_rng().gen_range(0..move_direction.len());
        let (x, y) = move_direction[choice];
        self.move_player(x, y);
        println!("{} moved to {:?}.", self.name, self.position);
    }

    fn purchase_weapon_attack(&mut self, _target: &mut Player) {
        self.purchase_weapon();
    }

    fn opponent_attack(&mut self, target: &mut Player) {
        let attack_choice: fn(&mut Player, &mut Player) = match rand::thread_rng().gen_range(0..=2) {
            0 => Player::move_attack,
            1 => Player::shoot_attack,
            _ => Player::purchase_weapon_attack,
        };
        attack_choice(self, target);
    }
}

struct Game {
    mode: String,
    agents: Vec<String>,
    weapons: Vec<String>,
}

impl Game {
    fn new(mode: &str) -> Self {
        let agents = Game::load_data("agents.txt");
        let weapons = Game::load_data("weapons.txt");
        Self {
            mode: mode.to_string(),
            agents,
            weapons,
        }
    }

    fn load_data(filename: &str) -> Vec<String> {
        std::fs::read_to_string(filename)
            .expect("Failed to read file")
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    fn start(&self) {
        let mut player1_name = String::new();
        println!("Enter your name: ");
        io::stdin().read_line(&mut player1_name).expect("Failed to read line");
        let agent = self.choose_agent();
        let credits = if self.mode == "unrated" { 200 } else { 0 };
        let mut player1 = Player {
            name: player1_name.trim().to_string(),
            agent,
            credits,
            health: 100,
            armor: 0,
            position: (0, 0),
            weapon: "Pistol".to_string(),
        };

        let opponent_agent = self.agents[rand::thread_rng().gen_range(0..self.agents.len())].clone();
        let opponent_credits = 200;
        let mut player2 = Player {
            name: "Opponent".to_string(),
            agent: opponent_agent,
            credits: opponent_credits,
            health: 100,
            armor: 0,
            position: (0, 0),
            weapon: "Pistol".to_string(),
        };

        while player1.health > 0 && player2.health > 0 {
            println!("\n=== Turn: {} ===", player1.name);
            self.display_stats(&player1, &player2);
            self.take_turn(&mut player1, &mut player2);

            if player2.health <= 0 {
                break;
            }

            player2.opponent_attack(&mut player1);
        }

        if player1.health > 0 {
            println!("\nCongratulations! {} wins!", player1.name);
        } else {
            println!("\nYou lost! Better luck next time.");
        }
    }

    fn choose_agent(&self) -> String {
        println!("Choose an agent:");
        for (idx, agent) in self.agents.iter().enumerate() {
            println!("{}. {}", idx + 1, agent);
        }

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: usize = choice.trim().parse().unwrap_or(0);
        self.agents[choice - 1].clone()
    }

    fn take_turn(&self, current_player: &mut Player, other_player: &mut Player) {
        println!("\nChoose an action:");
        println!("1. Move");
        println!("2. Shoot");
        println!("3. Purchase Weapon");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: usize = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => current_player.move_attack(other_player),
            2 => current_player.shoot_attack(other_player),
            3 => current_player.purchase_weapon_attack(other_player),
            _ => println!("Invalid choice!"),
        }
    }

    fn display_stats(&self, player: &Player, opponent: &Player) {
        println!("\n{}'s Stats:", player.name);
        println!("Agent: {}", player.agent);
        println!("Weapon: {}", player.weapon);
        println!("Credits: {}", player.credits);
        println!("Health: {}", player.health);
        println!("Armor: {}", player.armor);

        println!("\n{}'s Stats:", opponent.name);
        println!("Agent: {}", opponent.agent);
        println!("Weapon: {}", opponent.weapon);
        println!("Health: {}", opponent.health);
        println!("Armor: {}", opponent.armor);
    }
}

fn main() {
    println!("Welcome to Text-Based Valorant!");
    println!("Choose a mode:");
    println!("1. Unrated");
    println!("2. Competitive");
    println!("3. Team Death Match");

    let mut mode_choice = String::new();
    io::stdin().read_line(&mut mode_choice).expect("Failed to read line");
    let mode_choice: usize = mode_choice.trim().parse().unwrap_or(0);

    let mode = match mode_choice {
        1 => "unrated",
        2 => "competitive",
        3 => "team_death_match",
        _ => {
            println!("Invalid choice!");
            return;
        }
    };

    let game = Game::new(mode);
    game.start();
}
