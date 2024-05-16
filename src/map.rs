use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use std::collections::HashMap;
use std::time::Duration;

use crate::collision_detection::Collider;
use crate::enemy::{ActionState, EnemyState};
use crate::player::PLAYER_HIT_RADIUS;
use crate::sprites::SpriteComponent;
use crate::timer::{ShootingTimer, WalkTimer};
use crate::{
    enemy::Enemy,
    floor::Floor,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    render::MAX_STRUCTURES,
    vertex::Vertex,
    CustomMaterial, Player, SceneAssets, Wall,
};

#[derive(Component, Clone)]
pub struct Map {
    pub filename: String,
    pub selected_id: usize,
    pub camera: Vec<f32>,
    pub player: Player,
    pub walls: Vec<Wall>,
    pub floors: Vec<Floor>,
    pub enemies: Vec<Enemy>,
}

#[derive(Component)]
pub struct PlayerComponent;

impl Map {
    fn new() -> Self {
        let filename = String::from("");
        let selected_id = 0;
        let camera = Vec::new();
        let player = Player::new(0., 0., 0., 0., 0.);
        let walls = Vec::new();
        let floors = Vec::new();
        let enemies = Vec::new();
        Self {
            filename,
            selected_id,
            camera,
            player,
            walls,
            floors,
            enemies,
        }
    }

    pub fn populate_scene(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        custom_materials: &mut ResMut<Assets<CustomMaterial>>,
        scene_assets: &mut Res<SceneAssets>,
        window_query: &mut Query<&mut Window, With<PrimaryWindow>>,
    ) {
        let _window = window_query.single_mut();

        commands.spawn((
            self.player.clone(),
            PlayerComponent,
            Collider::new(PLAYER_HIT_RADIUS),
        ));

        for wall in &self.walls {
            commands.spawn((
                wall.clone(),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Wall::mesh()).into(),
                    material: custom_materials.add(CustomMaterial {
                        texture: scene_assets.textures[wall.texture_id].clone(),
                        id: -1.,
                        mask: [Vec3::new(0., 0., 0.); MAX_STRUCTURES],
                        mask_len: 0,
                        a_screen: Vec3::new(0., 0., 0.),
                        b_screen: Vec3::new(0., 0., 0.),
                        c_screen: Vec3::new(0., 0., 0.),
                        a_uv: Vec2::new(0., 0.),
                        b_uv: Vec2::new(0., 0.),
                        c_uv: Vec2::new(0., 0.),
                        uv_scalar: Vec2::new(1., 1.),
                        uv_offset: Vec2::new(0., 0.),
                        uv_rotation: 0.,
                        a_position: Vec3::new(0., 0., 0.),
                        b_position: Vec3::new(0., 0., 0.),
                        c_position: Vec3::new(0., 0., 0.),
                        pitch: 0.0,
                        selected: 0,
                    }),
                    ..Default::default()
                },
            ));
        }

        for floor in &self.floors {
            commands.spawn((
                floor.clone(),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Floor::mesh()).into(),
                    material: custom_materials.add(CustomMaterial {
                        texture: scene_assets.textures[floor.texture_id].clone(),
                        id: -1.,
                        mask: [Vec3::new(0., 0., 0.); MAX_STRUCTURES],
                        mask_len: 0,
                        a_screen: Vec3::new(0., 0., 0.),
                        b_screen: Vec3::new(0., 0., 0.),
                        c_screen: Vec3::new(0., 0., 0.),
                        a_uv: Vec2::new(0., 0.),
                        b_uv: Vec2::new(0., 0.),
                        c_uv: Vec2::new(0., 0.),
                        uv_scalar: Vec2::new(1., 1.),
                        uv_offset: Vec2::new(0., 0.),
                        uv_rotation: 0.,
                        a_position: Vec3::new(0., 0., 0.),
                        b_position: Vec3::new(0., 0., 0.),
                        c_position: Vec3::new(0., 0., 0.),
                        pitch: 0.0,
                        selected: 0,
                    }),
                    ..Default::default()
                },
            ));
        }

        // Spawn enemies
        for enemy in &self.enemies {
            commands.spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(Vec3::ZERO),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    sprite: SpriteBundle {
                        texture: scene_assets.enemy.clone(),
                        transform: Transform::from_translation(enemy.position),
                        ..default()
                    },
                },
                SpriteComponent {
                    position: enemy.position,
                    health: 100.,
                },
                ShootingTimer {
                    // create the non-repeating fuse timer
                    timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
                },
                EnemyState {
                    state: ActionState::Dormant,
                },
                WalkTimer {
                    timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
                },
                Collider::new(5.),
            ));
        }
    }

    pub fn save(&self) -> Option<()> {
        let path = Path::new("assets\\maps\\").join(&self.filename);

        let file = match File::create(&path) {
            Ok(file) => file,
            Err(_) => {
                println!("Could not open {}", &self.filename);
                return None;
            }
        };

        let mut writer = BufWriter::new(file);

        // Camera
        write_vector(&mut writer, &self.camera);

        // Player
        write_vector(
            &mut writer,
            &vec![
                self.player.x,
                self.player.y,
                self.player.z,
                self.player.yaw,
                self.player.pitch,
            ],
        );

        // Number of walls
        write_integer(&mut writer, self.walls.len() as i32);

        // Walls
        for wall in &self.walls {
            let data = vec![
                wall.id as f32,
                wall.start.position.x,
                wall.start.position.y,
                wall.start.position.z,
                wall.end.position.x,
                wall.end.position.y,
                wall.end.position.z,
                wall.height,
                wall.uv_scalar.x,
                wall.uv_scalar.y,
                wall.uv_offset.x,
                wall.uv_offset.y,
                wall.uv_rotation,
                wall.texture_id as f32,
            ];
            write_vector(&mut writer, &data);
        }

        // Number of floors
        write_integer(&mut writer, self.floors.len() as i32);

        // Floors
        for floor in &self.floors {
            let data = vec![
                floor.id as f32,
                floor.a.position.x,
                floor.a.position.y,
                floor.a.position.z,
                floor.b.position.x,
                floor.b.position.y,
                floor.b.position.z,
                floor.c.position.x,
                floor.c.position.y,
                floor.c.position.z,
                floor.uv_scalar.x,
                floor.uv_scalar.y,
                floor.uv_offset.x,
                floor.uv_offset.y,
                floor.uv_rotation,
                floor.world_aligned_uv as i32 as f32,
                floor.texture_id as f32,
            ];
            write_vector(&mut writer, &data);
        }

        println!("Saved to {}", &self.filename);
        return None;
    }
}

pub fn load_from_file(filename: &str, enemy_types: &HashMap<String, Enemy>) -> Option<Map> {
    let mut map = Map::new();
    map.filename = filename.to_string();

    // Load file
    let path = Path::new("assets\\maps\\").join(filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    let mut reader = BufReader::new(file);

    // Camera
    for i in read_vector(&mut reader) {
        map.camera.push(i);
    }

    // Player
    let player = read_vector(&mut reader);
    map.player = Player::new(player[0], player[1], player[2], player[3], player[4]);

    // Walls
    for _ in 0..read_integer(&mut reader) {
        let data = read_vector(&mut reader);
        let wall = Wall::new(
            data[0] as usize,
            Vertex::new(Vec3::new(data[1], data[2], data[3]), Vec2::new(0., 0.)),
            Vertex::new(Vec3::new(data[4], data[5], data[6]), Vec2::new(0., 0.)),
            data[7],
            Vec2::new(data[8], data[9]),
            Vec2::new(data[10], data[11]),
            data[12],
            data[13] as usize,
        );
        map.walls.push(wall);
    }

    // Floors
    for _ in 0..read_integer(&mut reader) {
        let data = read_vector(&mut reader);
        let floor = Floor::new(
            data[0] as usize,
            Vertex::new(Vec3::new(data[1], data[2], data[3]), Vec2::new(0., 0.)),
            Vertex::new(Vec3::new(data[4], data[5], data[6]), Vec2::new(0., 1.)),
            Vertex::new(Vec3::new(data[7], data[8], data[9]), Vec2::new(1., 1.)),
            Vec2::new(data[10], data[11]),
            Vec2::new(data[12], data[13]),
            data[14],
            data[15] == 1.,
            data[16] as usize,
        );
        map.floors.push(floor);
    }

    // Enemies
    for _ in 0..read_integer(&mut reader) {
        let data = read_vector(&mut reader);

        // Convert to enemy_type
        let enemy_type = match data[0] as i32 {
            0 => "enemy_a",
            _ => "enemy_a",
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
    let line = values
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    writeln!(writer, "{}", line).expect("Failed to write vector");
}
