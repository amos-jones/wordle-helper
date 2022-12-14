const ANSWERS: &[&str] = &include!(concat!(env!("OUT_DIR"), "/answers.in.rs"));
const GUESSES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/guesses.in.rs"));

use std::collections::HashSet;
use std::hash::Hash;

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Validity {
    Exact(char, u8),
    Present(char, u8),
    NotPresent(char),
} // end enum Validity

impl Validity {
    fn exact(c: char, i: usize) -> Self {
        Self::Exact(c, i as u8)
    }
    fn present(c: char, i: usize) -> Self {
        Self::Present(c, i as u8)
    }
    fn not_present(c: char) -> Self {
        Self::NotPresent(c)
    }
} // end impl Validity

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Word {
    validity: [Validity; 5],
} // end struct Word

#[derive(PartialEq, Eq, Debug)]
pub struct WordFinder {
    knowns: Vec<Validity>,
}

impl WordFinder {
    pub fn new() -> Self {
        Self { knowns: vec![] }
    }

    pub fn exact(&mut self, c: char, i: usize) -> &Self {
        self.knowns.push(Validity::exact(c, i));
        self
    }

    pub fn present(&mut self, c: char, not_i: usize) -> &Self {
        self.knowns.push(Validity::present(c, not_i));
        self
    }

    pub fn not_present(&mut self, c: char) -> &Self {
        self.knowns.push(Validity::not_present(c));
        self
    }

    pub fn possibilities(&self) -> Vec<&'static str> {
        ANSWERS
            .iter()
            .chain(GUESSES)
            .filter(|word| {
                self.knowns.iter().all(|v| match *v {
                    Validity::Exact(v, i) => word.chars().nth(i as usize) == Some(v),
                    Validity::Present(v, i) => {
                        word.chars().nth(i as usize) != Some(v) && word.contains(v)
                    }
                    Validity::NotPresent(v) => !word.contains(v),
                })
            })
            .copied()
            .collect()
    }
}

#[derive(Ord, PartialEq, Eq)]
pub struct WeightedString {
    pub word: &'static str,
    pub score: i32,
}

impl WeightedString {
    pub fn new(word: &'static str) -> Self {
        let score: i32 = word
            .chars()
            .map(|c| match c {
                'a' | 'e' => 3,
                'o' | 'u' | 'i' => 2,
                'r' | 's' | 't' | 'l' | 'n' => 3,
                'c' | 'h' | 'p' | 'm' | 'g' | 'f' | 'd'  => 2,
                'q' | 'z' | 'x' => -1,
                _ => 1,
            })
            .sum();
        let score: i32 = score + word.as_bytes().windows(2).map(|ch| match ch {
            b"ea" | b"ch" | b"ty" => 1,
            _ => 0
        }).sum::<i32>();
        let uniq = word
            .chars()
            .fold(String::new(), |mut acc, c| {
                if !acc.contains(c) {
                    acc.push(c)
                }
                acc
            })
            .len();
        Self {
            word,
            score: score * uniq as i32,
        }
    }
}

use std::cmp::Ordering;
impl PartialOrd for WeightedString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.score == other.score {
            Some(self.word.cmp(&other.word))
        } else {
            Some(self.score.cmp(&other.score))
        }
    }
}

//impl Word {
//    //fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Validity>> {
//    //    Box::new(self.validity.iter())
//    //}
//
//    fn word(&self) -> String {
//        self.validity.iter().map(|v| match v {
//            Validity::Exact(c, _) => c,
//            Validity::Present(c, _) => c,
//            Validity::NotPresent(c) => c
//        })
//        .cloned()
//        .collect::<String>()
//    }
//
//    fn compare(answer: &str, word: &str) -> Self {
//        if word.len() != 5 && answer.len() != word.len() {
//            todo!("handle invalid word lengths");
//        }
//
//        let validity: [Validity; 5] = answer
//            .chars()
//            .zip(word.chars())
//            .enumerate()
//            .map(|(i, (a, w))| {
//                if a == w {
//                    Validity::exact(w, i)
//                } else {
//                    if answer.contains(w) {
//                        Validity::present(w, i)
//                    } else {
//                        Validity::not_present(w)
//                    }
//                }
//            })
//            .collect::<Vec<_>>()
//            .try_into()
//            .unwrap();
//        Self { validity }
//    }
//
//    fn is_exact(&self) -> bool {
//        self.validity.iter().all(|v| matches!(v, Validity::Exact(_, _)))
//    }
//}
//
//#[derive(Copy, Clone, PartialEq, Eq, Debug)]
//enum GuessResult {
//    Correct,
//    Incorrect(Word),
//    Duplicate,
//    Invalid,
//}
//
//struct Puzzle<'a> {
//    answer: &'a str,
//    guesses: Vec<Word>,
//}
//
//impl <'a> Puzzle<'a> {
//    fn new(word: &'a str) -> Self {
//        Self {
//            answer: word,
//            guesses: Vec::new(),
//        }
//    }
//
//    fn guess(&mut self, word: &str) -> GuessResult {
//        if word.len() != 5 {
//            return GuessResult::Invalid;
//        }
//
//        let guess = Word::compare(self.answer, word);
//        if self.guesses.contains(&guess) {
//            return GuessResult::Duplicate;
//        }
//
//        self.guesses.push(guess.clone());
//        if guess.is_exact() {
//            GuessResult::Correct
//        } else {
//            GuessResult::Incorrect(guess)
//        }
//    }
//
//    fn guesses(&self) -> &[Word] {
//        &self.guesses
//    }
//
//    fn validity(&self) -> HashSet<Validity> {
//        self.guesses().iter().flat_map(|word| word.validity.iter().copied()).collect()
//    }
//}
//
//trait Solver {
//    fn solve(&self, puzzle: &mut Puzzle);
//    fn name(&self) -> String {
//        String::from("solver")
//    }
//}
//
//struct RandomSolver {
//    start_word: Option<String>,
//    hard_mode: bool
//}
//
//impl RandomSolver {
//    fn new() -> Self {
//        Self {
//            start_word: None,
//            hard_mode: true
//        }
//    }
//
//    fn with_start_word(word: &str) -> Self {
//        Self {
//            start_word: Some(String::from(word)),
//            hard_mode: true
//        }
//    }
//
//    fn possibilities(puzzle: &Puzzle) -> Vec<&'static str> {
//        let validity_set = puzzle.validity();
//        ANSWERS.iter().chain(GUESSES)
//            .filter(|word| {
//                validity_set.iter().all(|v| match *v {
//                    Validity::Exact(v, i) => word.chars().nth(i as usize) == Some(v),
//                    Validity::Present(v, i) => word.chars().nth(i as usize) != Some(v)
//                                              && word.contains(v),
//                    Validity::NotPresent(v) => !word.contains(v)
//                })
//            })
//            .copied()
//            .collect()
//    }
//}
//
//impl Solver for RandomSolver {
//    fn name(&self) -> String {
//        if let Some(start_word) = &self.start_word {
//            format!("random-{}", start_word)
//        }
//        else {
//            String::from("random")
//        }
//    }
//
//    fn solve(&self, puzzle: &mut Puzzle) {
//        let mut first = true;
//        loop {
//            let word = if first && self.start_word.is_some() {
//                first = false;
//                self.start_word.as_ref().unwrap().as_str()
//            } else {
//                rand_word(&RandomSolver::possibilities(puzzle))
//            };
//
//            let guess = puzzle.guess(word);
//            match guess {
//                GuessResult::Correct => break,
//                _ => continue,
//            };
//        }
//    }
//}
//
//fn rand_word<'a>(choices: &[&'a str]) -> &'a str {
//    let i = rand::random::<usize>() % choices.len();
//    choices.get(i).unwrap()
//}
//
//fn main() {
//    for start_word in ANSWERS.iter().chain(GUESSES) {
//
//        let mut i = 0;
//        loop {
//            let mut puzzle = Puzzle::new(rand_word(&ANSWERS));
//            let solver = RandomSolver::with_start_word(start_word);
//            solver.solve(&mut puzzle);
//
//            let guesses = puzzle.guesses().iter().map(|g| g.word()).collect::<Vec<String>>().join(",");
//            println!("{}|{}|{}", puzzle.answer, guesses, solver.name());
//            i += 1;
//            if i == 100 { break; }
//        }
//    }
//
//    let mut i = 0;
//    loop {
//        let mut puzzle = Puzzle::new(rand_word(&ANSWERS));
//        let solver = RandomSolver::new();
//        solver.solve(&mut puzzle);
//
//        let guesses = puzzle.guesses().iter().map(|g| g.word()).collect::<Vec<String>>().join(",");
//        println!("{}|{}|{}", puzzle.answer, guesses, solver.name());
//        i += 1;
//        if i == 100000 { break; }
//    }
//}
