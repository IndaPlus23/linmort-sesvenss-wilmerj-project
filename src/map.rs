use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::CustomMaterial;
use crate::Player;
use crate::SceneAssets;
use crate::Vertice;
use crate::Wall;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::enemy::Enemy;
use std::collections::HashMap;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

#[derive(Component, Clone)]
pub struct Map {
    pub filename: String,
    pub selected_id: usize,
    pub camera: Vec<f32>,
    pub player: Player,
    pub walls: Vec<Wall>,
    pub enemies: Vec<Enemy>,
}

impl Map {
    fn new() -> Self {
        let filename = String::from("");
        let selected_id = 0;
        let camera = Vec::new();
        let player = Player::new(0., 0., 0., 0., 0.);
        let walls = Vec::new();
        let enemies = Vec::new();
        Self {
            filename,
            selected_id,
            camera,
            player,
            walls,
            enemies,
        }
    }

    pub fn populate_scene(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        custom_materials: &mut ResMut<Assets<CustomMaterial>>,
        scene_assets: &mut Res<SceneAssets>,
    ) {
        commands.spawn(self.player.clone());

        for wall in &self.walls {
            commands.spawn((
                wall.clone(),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Wall::new_wall_mesh()).into(),
                    material: custom_materials.add(CustomMaterial {
                        texture: scene_assets.textures[wall.texture_id].clone(),
                        a: Vec3::new(0., 0., 0.),
                        b: Vec3::new(0., 0., 0.),
                        c: Vec3::new(0., 0., 0.),
                        a_uv: Vec2::new(0., 0.),
                        b_uv: Vec2::new(0., 0.),
                        c_uv: Vec2::new(0., 0.),
                        uv_scalar: Vec2::new(1., 1.),
                        uv_offset: Vec2::new(0., 0.),
                        uv_rotation: 0.,
                    }),
                    ..Default::default()
                },
            ));
        }

        // Spawn enemies
        for enemy in &self.enemies {
            commands.spawn(
                MovingObjectBundle {
                    velocity: Velocity::new(Vec3::ZERO),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    sprite: SpriteBundle {
                        texture: scene_assets.enemy.clone(),
                        transform: Transform::from_translation(enemy.position),
                        ..default()
                    },
                }
            );
        }
    }

    // TODO: Implement some way of saving enemies to file
    pub fn save(&self) -> Option<()> {
        let path = Path::new("assets\\maps\\").join(&self.filename);

        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let mut writer = BufWriter::new(file);

        write_vector(&mut writer, &self.camera);

        write_vector(&mut writer, &vec![
            self.player.x,
            self.player.y,
            self.player.z,
            self.player.yaw,
            self.player.pitch,
        ]);

        write_integer(&mut writer, self.walls.len() as i32);

        for wall in &self.walls {
            let data = vec![
                wall.id as f32,
                wall.start.position.x,
                wall.start.position.y,
                wall.start.position.z,
                wall.end.position.x,
                wall.end.position.y,
                wall.end.position.z,
                wall.uv_scalar.x,
                wall.uv_scalar.y,
                wall.uv_offset.x,
                wall.uv_offset.y,
                wall.uv_rotation,
                wall.texture_id as f32,
            ];
            write_vector(&mut writer, &data);
        }

        return None
    }
}

pub fn load_from_file(filename: &str, enemy_types: &HashMap<String, Enemy>) -> Option<Map> {
    let mut map = Map::new();

    let path = Path::new("assets\\maps\\").join(filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    let mut reader = BufReader::new(file);

    for i in read_vector(&mut reader) {
        map.camera.push(i);
    }

    let player = read_vector(&mut reader);
    map.player = Player::new(player[0], player[1], player[2], player[3], player[4]);

    for _ in 0..read_integer(&mut reader) {
        let data = read_vector(&mut reader);
        let wall = Wall::new(
            data[0] as usize,
            Vertice::new(Vec3::new(data[1], data[2], data[3]), Vec2::new(0., 0.)),
            Vertice::new(Vec3::new(data[4], data[5], data[6]), Vec2::new(0., 0.)),
            data[7],
            Vec2::new(data[8], data[9]),
            Vec2::new(data[10], data[11]),
            data[12],
            data[13] as usize,
        );
        map.walls.push(wall);
    }

    for _ in 0..read_integer(&mut reader) {
        let data = read_vector(&mut reader);

        // Convert to enemy_type
        let enemy_type = match data[0] as i32 {
            0 => "enemy_a",
            _ => "enemy_a"
        };

        let mut enemy = enemy_types.get(enemy_type).unwrap().clone();
        enemy.update_position(Vec3::new(data[1], data[2], data[3]));

        map.enemies.push(enemy.clone());
    }

    Some(map)
}

fn read_integer(reader: &mut BufReader<File>) -> i32 {
    if let Some(Ok(line)) = reader.lines().next() {
        match line.parse::<i32>() {
            Ok(parsed_value) => return parsed_value,
            Err(_) => return 0,
        }
    } else {
        return 0;
    }
}

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

fn write_integer(writer: &mut BufWriter<File>, value: i32) {
    writeln!(writer, "{}", value).expect("Failed to write integer");
}

fn write_vector(writer: &mut BufWriter<File>, values: &Vec<f32>) {
    let line = values.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" ");
    writeln!(writer, "{}", line).expect("Failed to write vector");
}