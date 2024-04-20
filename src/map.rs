use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

use bevy::prelude::*;
use crate::Wall;

#[derive(Component, Clone)]
pub struct Map {
    pub walls: Vec<Wall>,

}

impl Map {
    pub fn new() -> Self {
        let walls = Vec::new();
        Self {
            walls,
        }
    }
}

pub fn load() -> Option<Map> {
    let path = Path::new("../assets/maps/map.map");

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut map = Map::new();

    let mut reader = BufReader::new(file);
    let number_of_walls = read_integer(&mut reader);

    for i in 0..number_of_walls {
        let data = read_vector(&mut reader);
    }

    Some(map)
}

fn read_integer(reader: &mut BufReader<File>) -> i32 {
    if let Some(Ok(line)) = reader.lines().next() {
        match line.parse::<i32>() {
            Ok(parsed_value) => {
                return parsed_value
            }
            Err(e) => {
                return 0
            }
        }
    } else {
        return 0
    }
}

/*
pub fn new(
        start: Vertice,
        end: Vertice,
        height: f32,
        upper_triangle: bool,
        uv_scalar: Vec2,
        uv_offset: Vec2,
        uv_rotation: f32,
        texture_id: i32,
    )
*/

fn read_vector(reader: &mut BufReader<File>) -> Vec<f32> {
    if let Some(Ok(line)) = reader.lines().next() {
        let values: Vec<f32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<f32>().ok())
            .collect();

        values
    } else {
        Vec::new()
    }
}