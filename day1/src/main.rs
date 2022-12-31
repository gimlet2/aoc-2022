use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::Arc,
    vec,
};

fn main() {
    let input: String = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000"
        .to_string();
    let result = FoodLog::fromFile("input.txt".to_string()).max_by_key(|x| x.eated());
    println!("{}", result.unwrap().eated());

    let result2 = FoodLog::fromFile("input.txt".to_string())
        .map(|x| x.eated())
        .fold([0; 3], |mut acc: [usize; 3], v| {
            if v > acc[0] {
                if acc[0] > acc[1] {
                    acc[1] = acc[0]
                } else if acc[0] > acc[2] {
                    acc[2] = acc[0]
                }
                acc[0] = v
            }
            if v < acc[0] && v > acc[1] {
                if acc[1] > acc[2] {
                    acc[2] = acc[1]
                }
                acc[1] = v
            }
            if v < acc[0] && v < acc[1] && v > acc[2] {
                acc[2] = v
            }
            acc
        });
    println!("{}", result2.iter().sum::<usize>())
}

trait Eater {
    fn eat(&mut self, food: Food);
    fn eated(&self) -> usize;
}

#[derive(Clone)]
struct Elf {
    index: usize,
    eated: vec::Vec<Food>,
}

impl Eater for Elf {
    fn eat(&mut self, food: Food) {
        self.eated.push(food);
    }

    fn eated(&self) -> usize {
        return self.eated.iter().map(|e| e.calories).sum::<usize>();
    }
}

#[derive(Copy, Clone)]
struct Food {
    calories: usize,
}

struct FoodLog {
    index: usize,
    elfs: vec::Vec<Elf>,
}

impl FoodLog {
    fn fromFile(path: String) -> FoodLog {
        let mut index: usize = 0;
        let mut elfs: vec::Vec<Elf> = vec![];
        let mut acc: vec::Vec<usize> = vec![];

        if let Ok(lines) = read_lines(path) {
            for line in lines {
                if let Ok(s) = line {
                    if s.trim().is_empty() {
                        let elf = Elf {
                            index: index,
                            eated: acc.iter().map(|v| Food { calories: *v }).collect(),
                        };
                        index += 1;
                        elfs.push(elf);
                        acc.clear();
                    } else {
                        match s.trim().parse::<usize>() {
                            Ok(n) => acc.push(n),
                            Err(e) => panic!("{}", e),
                        }
                    }
                }
            }
        }
        return FoodLog {
            index: 0,
            elfs: elfs,
        };
    }

    fn new(input: String) -> FoodLog {
        let mut index: usize = 0;
        let mut elfs: vec::Vec<Elf> = vec![];
        let mut acc: vec::Vec<usize> = vec![];
        for s in input.split('\n') {
            if s.trim().is_empty() {
                let elf = Elf {
                    index: index,
                    eated: acc.iter().map(|v| Food { calories: *v }).collect(),
                };
                index += 1;
                elfs.push(elf);
                acc.clear();
            } else {
                match s.trim().parse::<usize>() {
                    Ok(n) => acc.push(n),
                    Err(e) => panic!("{}", e),
                }
            }
        }
        return FoodLog {
            index: 0,
            elfs: elfs,
        };
    }
}

impl Iterator for FoodLog {
    type Item = Elf;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.index < self.elfs.len() {
            self.index += 1;
            match self.elfs.get(self.index) {
                Some(e) => {
                    let elf = &*e;
                    Some(elf.clone())
                }
                None => None,
            }
        } else {
            None
        };
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
