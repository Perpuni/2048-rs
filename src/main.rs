use std::io::Write;
use std::{fmt, vec};
use std::cmp::max;
use std::process::Command;

use rand::seq::SliceRandom;
use rand::{random, thread_rng};

#[derive(PartialEq)]
enum Message {
    IncorrectMove,
    Win,
    Loose,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(PartialEq)]
enum GameState {
    Win,
    Loose,
    InGame,
}

pub struct Game {
    field: Vec<Vec<Option<u32>>>,
    state: GameState,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut max_len = 1;

        for line in self.field.iter() {
            for item in line.iter() {
                if match item {
                    None => 0,
                    Some(i) => i.to_string().len(),
                } > max_len {
                    max_len = item.unwrap().to_string().len();
                }
            }
        }

        for line in self.field.iter() {
            for item in line.iter() {
                match item {
                    None => {
                        let space = ((max_len - max_len % 2 + 1) as f32 / 2.).ceil();
                        write!(f, "{}", " ".repeat(space as usize));
                        write!(f, "-");
                        write!(f, "{}", " ".repeat(space as usize));
                    },
                    Some(n) => {
                        let space = ((max_len - max_len % 2 + 1) as f32 / 2.).ceil() - (n.to_string().len() as f32 / 2.).floor();
                        write!(f, "{}", " ".repeat(space as usize));
                        if n.to_string().len() % 2 == 0 {
                            write!(f, "{n} ");
                        } else {
                            write!(f, "{n}");
                        }
                        write!(f, "{}", " ".repeat(space as usize));
                    },
                };
            }
            writeln!(f).unwrap();
        }

        write!(f, "")
    }
}

impl Game {
    // System functions
    fn build() -> Game {
        Game {
            field: vec![vec![None; 4]; 4],
            state: GameState::InGame,
        }
    }

    fn render(&self, msg: Option<Message>) {
        // write!(stdout(), "\x1B[2J\x1B[1;1H").unwrap();
        // stdout().flush().unwrap();
        if cfg!(windows) {
            let _ = Command::new("cmd").arg("/c").arg("cls").status();
        } else {
            let _ = Command::new("clear").status();
        }

        print!("{}", self);

        if let Some(m) = msg {
            println!();
            match m {
                Message::IncorrectMove => {println!("This move is incorrect")},
                Message::Win => {println!("Congratulations! You won!")},
                Message::Loose => {println!("Looser!")},
            }
        }
    }

    fn update(&mut self) {
        let mut msg = self.make_move();
        if msg == None {
            self.spawn_number();
            if self.state == GameState::Loose {
                msg = Some(Message::Loose);
            }

            let mut max_num = 2;

            for line in self.field.iter() {
                for item in line.iter() {
                    if match item {
                        None => 0,
                        Some(i) => *i,
                    } > max_num {
                        max_num = item.unwrap();
                    }
                }
            }

            if max_num == 2048 {
                msg = Some(Message::Win);
                self.state = GameState::Win;
            }
        }
        self.render(msg);
    }

    // Gameplay functions
    fn spawn_number(&mut self) {
        let mut empty_cells: Vec<(usize, usize)> = vec![];

        for i in 0..16 {
            let x = (i as f32 / 4.).floor() as usize;
            let y = (i % 4) as usize;

            if self.field[x][y] == None {
                empty_cells.insert(empty_cells.len(), (x, y));
            }
        }

        let place = empty_cells.choose(&mut thread_rng());
        match place {
            None => {self.state = GameState::Loose;},
            Some((x, y)) => {
                let chance = random::<f32>();
                let mut number = 2;
                if chance > 0.9 { number = 4; }

                self.field[*x][*y] = Some(number);
            }
        }
    }

    fn shift(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => {
                for x in 0..4 {
                    for _ in 0..3 {
                        for y in 1..4 {
                            if self.field[y-1][x] == None {
                                self.field[y-1][x] = self.field[y][x];
                                self.field[y][x] = None;
                            }
                        }
                    }
                }
            }
            Direction::Left => {
                for y in 0..4 {
                    for _ in 0..3 {
                        for x in 1..4 {
                            if self.field[y][x-1] == None {
                                self.field[y][x-1] = self.field[y][x];
                                self.field[y][x] = None;
                            }
                        }
                    }
                }
            }
            Direction::Down => {
                for x in 0..4 {
                    for _ in 0..3 {
                        for y in 1..4 {
                            if self.field[4-y][x] == None {
                                self.field[4-y][x] = self.field[3-y][x];
                                self.field[3-y][x] = None;
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                for y in 0..4 {
                    for _ in 0..3 {
                        for x in 1..4 {
                            if self.field[y][4-x] == None {
                                self.field[y][4-x] = self.field[y][3-x];
                                self.field[y][3-x] = None;
                            }
                        }
                    }
                }
            }
        }
    }

    fn summarize(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => {
                for x in 0..4 {
                    for _ in 0..3 {
                        for y in 1..4 {
                            if self.field[y-1][x] == self.field[y][x] {
                                match self.field[y][x] {
                                    None => {}
                                    Some(b) => {
                                        self.field[y-1][x] = Some(b * 2);
                                        self.field[y][x] = None;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Direction::Left => {
                for y in 0..4 {
                    for _ in 0..3 {
                        for x in 1..4 {
                            if self.field[y][x-1] == self.field[y][x] {
                                match self.field[y][x] {
                                    None => {}
                                    Some(b) => {
                                        self.field[y][x-1] = Some(b * 2);
                                        self.field[y][x] = None;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Direction::Down => {
                for x in 0..4 {
                    for _ in 0..3 {
                        for y in 1..4 {
                            if self.field[4-y][x] == self.field[3-y][x] {
                                match self.field[4-y][x] {
                                    None => {}
                                    Some(b) => {
                                        self.field[3-y][x] = Some(b * 2);
                                        self.field[4-y][x] = None;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                for y in 0..4 {
                    for _ in 0..3 {
                        for x in 1..4 {
                            if self.field[y][4-x] == self.field[y][3-x] {
                                match self.field[y][4-x] {
                                    None => {}
                                    Some(b) => {
                                        self.field[y][3-x] = Some(b * 2);
                                        self.field[y][4-x] = None;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn make_move(&mut self) -> Option<Message> {
        let mut move_dir = "".to_string();
        std::io::stdin().read_line(&mut move_dir).expect("Failed to read line");

        let dir = move_normalize(&move_dir);

        match dir {
            None => {Some(Message::IncorrectMove)}
            Some(d) => {
                self.shift(&d);
                self.summarize(&d);
                self.shift(&d);

                None
            }
        }
    }
}

fn move_normalize(inp: &String) -> Option<Direction> {
    let mut m = inp.to_lowercase();
    m.truncate(m.len()-2);
    match m.as_str() {
        "w" => Some(Direction::Up),
        "a" => Some(Direction::Left),
        "s" => Some(Direction::Down),
        "d" => Some(Direction::Right),
        _ => {None},
    }
}

fn main() {
    println!("Welcome to 2048 CLI!");
    println!("To move print w, a, s or d");
    let mut stop = "".to_string();
    std::io::stdin().read_line(&mut stop).expect("Failed to read line");
    let mut game = Game::build();
    game.spawn_number();
    game.render(None);

    // Game loop
    while game.state == GameState::InGame {
        game.update();
    }
    if game.state == GameState::Loose {game.render(Some(Message::Loose))}
    else if game.state == GameState::Win {game.render(Some(Message::Win))}
    std::io::stdin().read_line(&mut stop).expect("Failed to read line");
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn move_normalized() {
//         let mv = "1".to_string();
//         assert_eq!(None, move_normalize(&mv));
//
//         let mv = "ss".to_string();
//         assert_eq!(None, move_normalize(&mv));
//
//         let mv = "a-".to_string();
//         assert_eq!(None, move_normalize(&mv));
//
//         let mv = "SDX12ef".to_string();
//         assert_eq!(None, move_normalize(&mv));
//
//         let mv = "W".to_string();
//         assert_eq!(Some(Direction::Up), move_normalize(&mv));
//
//         let mv = "a".to_string();
//         assert_eq!(Some(Direction::Left), move_normalize(&mv));
//
//         let mv = "s".to_string();
//         assert_eq!(Some(Direction::Down), move_normalize(&mv));
//
//         let mv = "D".to_string();
//         assert_eq!(Some(Direction::Right), move_normalize(&mv));
//     }
// }