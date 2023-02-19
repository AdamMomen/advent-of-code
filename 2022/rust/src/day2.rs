use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use std::path::PathBuf;
use std::collections::HashMap;

enum Move {
    ROCK,
    PAPER,
    SCISSOR
}

enum _Result {
    WIN,
    LOSE,
    DRAW
}

struct MoveScore {
    name: Move,
    score: u32,
}

struct ResultScore {
    result: _Result,
    score: u32
}

pub struct Solution {
    movement_scores: HashMap<String, MoveScore>,
    result_scores: HashMap<String, ResultScore>,
}


impl Solution {
    pub fn new() -> Solution{
          let mut movement_scores = HashMap::new();
          let mut result_scores = HashMap::new();

          movement_scores.insert(String::from("A"), MoveScore{name:Move::ROCK, score: 1});
          movement_scores.insert(String::from("B"), MoveScore{name:Move::PAPER,score: 2});
          movement_scores.insert(String::from("C"), MoveScore{name:Move::SCISSOR,score: 3});

          result_scores.insert(String::from("X"), ResultScore{result: _Result::LOSE, score: 0});
          result_scores.insert(String::from("Y"), ResultScore{result: _Result::DRAW, score: 3});
          result_scores.insert(String::from("Z"), ResultScore{result: _Result::WIN, score: 6});

          Solution {
              movement_scores,
              result_scores
          }
    }


    fn is_rock(&self, movement:&str) -> bool {
        match self.movement_scores.get(movement)
            .expect("value not found")
            .name {
                Move::ROCK => true,
                _ => false
            }
    }

    fn is_paper(&self, movement: &str) -> bool{
        match self.movement_scores.get(movement)
            .expect("value not found")
            .name {
                Move::PAPER=> true,
                _ => false
            }
    }

    fn is_scissor(&self, movement: &str) -> bool{
        match self.movement_scores.get(movement)
            .expect("value not found")
            .name {
                Move::SCISSOR=> true,
                _ => false
            }
    }

    fn should_win(&self, result: &str) -> bool {
        match self.result_scores.get(result)
            .expect("value not found")
            .result {
                _Result::WIN=> true,
                _ => false
            }
    }

    fn should_lose(&self, result: &str) -> bool {
        match self.result_scores.get(result)
            .expect("value not found")
            .result {
                _Result::LOSE=> true,
                _ => false
            }
    }

    fn should_draw(&self, result:&str) -> bool {
        match self.result_scores.get(result)
            .expect("value not found")
            .result {
                _Result::DRAW => true,
                _ => false
            }
    }

    fn read_file_lines(&self, file_path: &PathBuf) -> Result<Vec<String>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut lines = vec![];
        for line in reader.lines() {
            lines.push(line?);
        }
        Ok(lines)
    }

    fn choose_next_move(&self, opponent_move: &str, required_result: &str) -> &str {
         if self.should_win(&required_result) {
            if self.is_rock(&opponent_move) {
                return "B"
            }
            else if self.is_paper(&opponent_move) {
                return "C"
            }
            else if self.is_scissor(&opponent_move) {
                return "A"
            }
         }
         if self.should_lose(&required_result) {
            if self.is_rock(&opponent_move) {
                return "C"
            }
            else if self.is_paper(&opponent_move) {
                return "A"
            }
            else if self.is_scissor(&opponent_move) {
                return "B"
            }
         }

         if self.should_draw(&required_result) {
            if self.is_rock(&opponent_move) {
                return "A"
            }
            if self.is_paper(&opponent_move) {
                return "B"
            }
            else if self.is_scissor(&opponent_move) {
                return "C"
            }
            else { "" }
         }
         else { "" }
    }

    pub fn evaluate_round(&self, line: String) -> u32 {

        let mut splited_line = line.split_whitespace().into_iter();
        let p1_move = splited_line.next().unwrap();
        let required_result = splited_line.next().unwrap();
        let next_move = self.choose_next_move(p1_move, required_result);
        let first_score = self.movement_scores.get(next_move).unwrap().score ;
        let second_score = self.result_scores.get(required_result).unwrap().score;
        return first_score + second_score;
    }


    pub fn solve(&self) -> Result<u32> {
        let mut result: Vec<u32> = Vec::new();
        let mut file_path = PathBuf::new();
        file_path.push("/Users/adam/personal/advent_of_code/2022/input/day2.txt");
        let lines = self.read_file_lines(&file_path).expect("Error reading the file");
        for line in lines {
            result.push(self.evaluate_round(line))
        }

        Ok(result.into_iter().sum())
    }
}
