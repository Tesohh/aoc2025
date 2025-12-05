use std::{env, fs};

use crate::grid::Grid;

#[derive(Debug, Clone)]
pub struct Input {
    file: String,
}

impl Input {
    pub fn from_path(path: String) -> Self {
        let mut file = fs::read_to_string(path).unwrap();
        file.pop();
        Input { file }
    }

    pub fn from_args() -> Self {
        let path = env::args().nth(1).unwrap();
        Self::from_path(path)
    }

    pub fn from_string(s: &str) -> Self {
        Input {
            file: s.to_string(),
        }
    }

    pub fn parts(&self, sep: &str) -> Vec<Self> {
        self.file
            .split(sep)
            .map(|f| Input {
                file: f.to_string(),
            })
            .collect()
    }

    pub fn lines(&self) -> Vec<String> {
        self.file.split("\n").map(str::to_string).collect()
    }

    pub fn splits(&self, sep: &str) -> Vec<String> {
        self.file.split(sep).map(str::to_string).collect()
    }

    pub fn raw(self) -> String {
        self.file
    }

    pub fn char_grid(&self) -> Grid<char> {
        self.lines()
            .into_iter()
            .map(|s| s.chars().collect())
            .collect()
    }

    pub fn i64_grid(&self) -> Grid<i64> {
        self.char_grid()
            .into_iter()
            .map(|vec_char| {
                vec_char
                    .into_iter()
                    .map(|c| {
                        if c == '.' {
                            -1
                        } else {
                            c.to_digit(10).unwrap_or(0) as i64
                        }
                    })
                    .collect()
            })
            .collect()
    }
}
