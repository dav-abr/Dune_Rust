use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub i: i8,
    pub j: i8,
}

impl Position {
    pub fn new(i: i8, j: i8) -> Self {
        Self {
            i, j
        }
    }
    pub fn distance(&self, other: &Position) -> u32 {
        (self.i.abs_diff(other.i) + self.j.abs_diff(other.j)) as u32
    }

    pub fn successors(&self, creatures_position: &Vec<Self>) -> Vec<(Position, u32)> {
        let i = self.i;
        let j = self.j;

        let result = vec![
                (Position::new(i + 1, j + 1), 2), (Position::new(i + 1, j - 1), 2), (Position::new(i - 1, j + 1), 2), (Position::new(i - 1, j - 1), 2),
                (Position::new(i + 1, j), 1), (Position::new(i - 1, j), 1), (Position::new(i, j + 1), 1), (Position::new(i, j - 1), 1)
            ]
            .into_iter()
            .filter(|position| if creatures_position.len() > 0 {
                creatures_position.into_iter().any(|creature_position| (position.0.i, position.0.j) != (creature_position.i, creature_position.j))
            } else {
                true
            })
            .collect();
        
        result
    }
}

impl Into<(i32, i32)> for Position {
    fn into(self) -> (i32, i32) {
        (self.i as i32, self.j as i32)
    }
}

#[derive(Component)]
pub struct PathToGo {
    pub path: VecDeque<Position>
}

#[derive(Component)]
pub struct Movable {}

#[derive(Component)]
pub struct GameCursor {}

#[derive(Component)]
pub struct SelectedCreature {}

#[derive(Component)]
pub struct Dimensions {
    pub width: i8,
    pub height: i8,
}

#[derive(Component)]
pub struct SelectCursor {}
