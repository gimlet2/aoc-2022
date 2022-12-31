use std::{
    fs::File,
    io::{self, BufRead},
    ops,
    path::Path,
    str::FromStr,
    vec,
};

fn main() {
    let result = GameLog::fromFile("input.txt".to_string())
        .rounds
        .iter()
        .map(|r| r.outcome + r.p2)
        .sum::<usize>();
    println!("{}", result)
}

#[derive(Copy, Clone)]
enum Figures {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Figures {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Figures::Rock),
            "B" => Ok(Figures::Paper),
            "C" => Ok(Figures::Scissors),
            "X" => Ok(Figures::Rock),
            "Y" => Ok(Figures::Paper),
            "Z" => Ok(Figures::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

impl ops::Add<Figures> for Figures {
    type Output = usize;

    fn add(self, rhs: Figures) -> Self::Output {
        match rhs {
            Figures::Rock => match self {
                Figures::Rock => 1 + 3,
                Figures::Paper => 2 + 6,
                Figures::Scissors => 3 + 0,
            },
            Figures::Paper => match self {
                Figures::Rock => 1 + 0,
                Figures::Paper => 2 + 3,
                Figures::Scissors => 3 + 6,
            },
            Figures::Scissors => match self {
                Figures::Rock => 1 + 6,
                Figures::Paper => 2 + 0,
                Figures::Scissors => 3 + 3,
            },
        }
    }
}

impl ops::Add<Figures> for Outcome {
    type Output = usize;

    fn add(self, rhs: Figures) -> Self::Output {
        match rhs {
            Figures::Rock => match self {
                Outcome::Lose => 3 + 0,
                Outcome::Draw => 1 + 3,
                Outcome::Win => 2 + 6,
            },
            Figures::Paper => match self {
                Outcome::Lose => 1 + 0,
                Outcome::Draw => 2 + 3,
                Outcome::Win => 3 + 6,
            },
            Figures::Scissors => match self {
                Outcome::Lose => 2 + 0,
                Outcome::Draw => 3 + 3,
                Outcome::Win => 1 + 6,
            },
        }
    }
}

struct GameLog {
    rounds: vec::Vec<Round>,
}
struct Round {
    p1: Figures,
    p2: Figures,
    outcome: Outcome,
}

impl GameLog {
    fn fromFile(path: String) -> GameLog {
        let mut rounds: vec::Vec<Round> = vec![];

        if let Ok(lines) = read_lines(path) {
            for line in lines {
                if let Ok(s) = line {
                    let i = s.trim().split_whitespace().collect::<vec::Vec<&str>>();

                    let round = Round {
                        p1: Figures::from_str(i[0]).unwrap(),
                        p2: Figures::from_str(i[1]).unwrap(),
                        outcome: Outcome::from_str(i[1]).unwrap(),
                    };
                    rounds.push(round);
                }
            }
        }
        return GameLog { rounds: rounds };
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
