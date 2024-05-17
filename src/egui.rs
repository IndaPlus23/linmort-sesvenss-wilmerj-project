use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::f32::consts::PI;

use crate::{
    floor::spawn_floor, floor::Floor, map::Map, render::MapFloor, vertex::Vertex, wall::spawn_wall,
    CustomMaterial, Player, SceneAssets, Wall,
};

pub fn editor_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut player_query: Query<&mut Player>,
    mut map_floors: Query<&mut MapFloor>,
    mut map_query: Query<&mut Map>,
    mut contexts: EguiContexts,
    mut asset_server: Res<SceneAssets>,
    mut wall_query: Query<&mut Wall>,
    mut floor_query: Query<&mut Floor>,
) {
    egui::Window::new("Editor").show(contexts.ctx_mut(), |ui| match map_query.get_single_mut() {
        Ok(mut map) => {
            ui.heading("Map");
            let mut map_scale = 1.0;
            let mut x_offset = 0.0;
            let mut y_offset = 0.0;
            for map_floor in map_floors.iter_mut() {
                map_scale = map_floor.scale;
                x_offset = map_floor.x_offset;
                y_offset = -map_floor.y_offset;
            }
            ui.horizontal(|ui| {
                ui.label("Scale:");
                ui.add(
                    egui::DragValue::new(&mut map_scale)
                        .speed(0.01)
                        .custom_formatter(|n, _| format!("{:.2}", n)),
                );
            });
            ui.heading("Offset");
            ui.horizontal(|ui| {
                ui.label("x:");
                ui.add(
                    egui::DragValue::new(&mut x_offset)
                        .speed(1.0)
                        .custom_formatter(|n, _| format!("{:.0}", n)),
                );
            });
            ui.horizontal(|ui| {
                ui.label("y:");
                ui.add(
                    egui::DragValue::new(&mut y_offset)
                        .speed(1.0)
                        .custom_formatter(|n, _| format!("{:.0}", n + 0.001)),
                );
            });
            for mut map_floor in map_floors.iter_mut() {
                map_floor.scale = map_scale;
                map_floor.x_offset = x_offset;
                map_floor.y_offset = -y_offset;
            }
            ui.separator();

            let texture_paths = &asset_server.texture_paths;
            let n_walls = map.walls.len();
            let n_floors = map.floors.len();

            ui.heading("Select compontent");
            ui.horizontal(|ui| {
                ui.label("id:");
                ui.add(egui::Slider::new(
                    &mut map.selected_id,
                    0..=n_walls + n_floors - 1,
                ));
            });

            ui.separator();
            if ui.button("Spawn wall").clicked() {
                for player in player_query.iter_mut() {
                    let wall = Wall::new(
                        map.walls.len(),
                        Vertex::new(
                            Vec3::new(player.x.round(), 0.0, player.z.round()),
                            Vec2::ZERO,
                        ),
                        Vertex::new(
                            Vec3::new(player.x.round() + 10.0, 0.0, player.z.round() + 10.0),
                            Vec2::ZERO,
                        ),
                        10.,
                        Vec2::ONE,
                        Vec2::ZERO,
                        0.0,
                        0,
                    );

                    spawn_wall(
                        &mut commands,
                        &mut meshes,
                        &mut custom_materials,
                        &asset_server,
                        wall.clone(),
                    );

                    map.walls.push(wall)
                }
            }

            ui.separator();
            if ui.button("Spawn floor").clicked() {
                let len = map.floors.len() - 1;
                let floor = Floor::new(
                    map.floors.len() + 1000,
                    Vertex::new(map.floors[len].a.position, Vec2::ZERO),
                    Vertex::new(map.floors[len].b.position, Vec2::ZERO),
                    Vertex::new(map.floors[len].c.position, Vec2::ZERO),
                    Vec2::ONE,
                    Vec2::ZERO,
                    0.0,
                    true,
                    1,
                );

                spawn_floor(
                    &mut commands,
                    &mut meshes,
                    &mut custom_materials,
                    &asset_server,
                    floor.clone(),
                );

                map.floors.push(floor)
            }

            ui.separator();
            if ui.button("Save").clicked() {
                let _ = map.save();
            }
            ui.separator();

            for mut player in player_query.iter_mut() {
                let mut player_transformation = Player::new(0., 0., 0., 0., 0.);
                player_transformation.x = player.x;
                player_transformation.y = player.y;
                player_transformation.z = player.z;
                player_transformation.yaw = player.yaw;
                player_transformation.pitch = player.pitch;

                ui.heading("Player");
                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("     x:");
                        ui.add(
                            egui::DragValue::new(&mut player_transformation.x)
                                .speed(1.0)
                                .custom_formatter(|n, _| format!("{:.0}", n)),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("       y:");
                        ui.add(
                            egui::DragValue::new(&mut player_transformation.y)
                                .speed(1.0)
                                .custom_formatter(|n, _| format!("{:.0}", n)),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("z:");
                        ui.add(
                            egui::DragValue::new(&mut player_transformation.z)
                                .speed(1.0)
                                .custom_formatter(|n, _| format!("{:.0}", n)),
                        );
                    });
                });

                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Yaw:");
                        ui.add(
                            egui::DragValue::new(&mut player_transformation.yaw)
                                .speed(PI / 180.)
                                .custom_formatter(|n, _| {
                                    let n = n as f32;
                                    let degrees = n.rem_euclid(2.0 * PI) as f32 * 180. / PI;
                                    format!("{:.0}", degrees)
                                })
                                .suffix("°"),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Pitch:");
                        ui.add(
                            egui::DragValue::new(&mut player_transformation.pitch)
                                .speed(PI / 180.)
                                .custom_formatter(|n, _| {
                                    let n = n as f32;
                                    let degrees = n.clamp(-PI / 2.0, PI / 2.0) as f32 * 180. / PI;
                                    format!("{:.0}", degrees)
                                })
                                .suffix("°"),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("   ");
                        egui::reset_button_with(
                            ui,
                            &mut player_transformation,
                            Player::new(0., 5., 0., 0., 0.),
                        );
                    });
                });
                ui.separator();

                player.x = player_transformation.x;
                player.y = player_transformation.y;
                player.z = player_transformation.z;
                player.yaw = player_transformation.yaw;
                player.pitch = player_transformation.pitch;

                player.yaw = player.yaw.rem_euclid(2.0 * PI);
                player.pitch = player.pitch.clamp(-PI / 2.0, PI / 2.0);

                map.player.x = player.x;
                map.player.y = player.y;
                map.player.z = player.z;
                map.player.yaw = player.yaw;
                map.player.pitch = player.pitch;
            }

            for mut wall in wall_query.iter_mut() {
                if wall.id == map.selected_id {
                    let mut start_transformation = wall.start.transformation;
                    let current_start = wall.start.position;

                    let mut end_transformation = wall.end.transformation;
                    let current_end = wall.end.position;

                    let mut height = wall.height;
                    let mut uv_scaling = wall.uv_scalar;
                    let mut uv_offset = wall.uv_offset;
                    let mut uv_rotation = wall.uv_rotation;

                    let mut texture_id = wall.texture_id;

                    ui.heading("Transform");
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("       Start");
                            ui.horizontal(|ui| {
                                ui.label("x:");
                                ui.add(
                                    egui::DragValue::new(&mut start_transformation.x)
                                        .custom_formatter(|_, _| {
                                            format!("{}", wall.start.position.x)
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut start_transformation.y)
                                        .custom_formatter(|_, _| {
                                            format!("{}", wall.start.position.y)
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut start_transformation.z)
                                        .custom_formatter(|_, _| {
                                            format!("{}", wall.start.position.z)
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("   ");
                                egui::reset_button(ui, &mut start_transformation);
                            });
                        });

                        ui.label("  ");
                        ui.vertical(|ui| {
                            ui.label("        End");
                            ui.horizontal(|ui| {
                                ui.label("x:");
                                ui.add(
                                    egui::DragValue::new(&mut end_transformation.x)
                                        .custom_formatter(|_, _| format!("{}", wall.end.position.x))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut end_transformation.y)
                                        .custom_formatter(|_, _| format!("{}", wall.end.position.y))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut end_transformation.z)
                                        .custom_formatter(|_, _| format!("{}", wall.end.position.z))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("   ");
                                egui::reset_button(ui, &mut end_transformation);
                            });
                        });
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label("height:");
                        ui.add(egui::DragValue::new(&mut height).speed(1.0));
                    });
                    ui.separator();

                    ui.heading("UV scaling");
                    ui.horizontal(|ui| {
                        ui.label("u:");
                        ui.add(egui::Slider::new(&mut uv_scaling.x, 0.0..=20.));
                    });
                    ui.horizontal(|ui| {
                        ui.label("v:");
                        ui.add(egui::Slider::new(&mut uv_scaling.y, 0.0..=10.));
                    });
                    ui.separator();

                    ui.heading("UV offset");
                    ui.horizontal(|ui| {
                        ui.label("u:");
                        ui.add(egui::Slider::new(&mut uv_offset.x, 0.0..=10.));
                    });
                    ui.horizontal(|ui| {
                        ui.label("v:");
                        ui.add(egui::Slider::new(&mut uv_offset.y, 0.0..=10.));
                    });
                    ui.separator();

                    ui.heading("UV rotation");
                    ui.horizontal(|ui| {
                        ui.label("degrees:");
                        ui.add(egui::Slider::new(&mut uv_rotation, 0.0..=360.).suffix("°"));
                    });

                    let selected_text = asset_server.texture_paths[texture_id].clone();

                    ui.separator();
                    ui.heading("Texture");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{selected_text}"))
                        .show_ui(ui, |ui| {
                            for (i, texture) in texture_paths.iter().enumerate() {
                                ui.selectable_value(&mut texture_id, i, texture);
                            }
                        });

                    wall.start.transformation = start_transformation;
                    wall.start.position = wall.start.original_position + start_transformation;

                    wall.end.transformation = end_transformation;
                    wall.end.position = wall.end.original_position + end_transformation;

                    wall.height = height;

                    wall.uv_scalar = uv_scaling;
                    wall.uv_offset = uv_offset;
                    wall.uv_rotation = uv_rotation;

                    wall.texture_id = texture_id;

                    map.walls[wall.id].start.position = wall.start.position;
                    map.walls[wall.id].end.position = wall.end.position;
                    map.walls[wall.id].height = wall.height;
                    map.walls[wall.id].uv_scalar = wall.uv_scalar;
                    map.walls[wall.id].uv_offset = wall.uv_offset;
                    map.walls[wall.id].uv_rotation = wall.uv_rotation;
                    map.walls[wall.id].texture_id = wall.texture_id;
                }
            }

            for mut floor in floor_query.iter_mut() {
                if floor.id == (1000 + map.selected_id - n_walls) {
                    let mut a_transformation = floor.a.transformation;
                    let current_a = floor.a.position;

                    let mut b_transformation = floor.b.transformation;
                    let current_b = floor.b.position;

                    let mut c_transformation = floor.c.transformation;
                    let current_c = floor.c.position;

                    let mut uv_scaling = floor.uv_scalar;
                    let mut uv_offset = floor.uv_offset;
                    let mut uv_rotation = floor.uv_rotation;

                    let mut texture_id = floor.texture_id;

                    ui.heading("Transform");
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("          A");
                            ui.horizontal(|ui| {
                                ui.label("x:");
                                ui.add(
                                    egui::DragValue::new(&mut a_transformation.x)
                                        .custom_formatter(|_, _| format!("{}", floor.a.position.x))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut a_transformation.y)
                                        .custom_formatter(|_, _| format!("{}", floor.a.position.y))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut a_transformation.z)
                                        .custom_formatter(|_, _| format!("{}", floor.a.position.z))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("   ");
                                egui::reset_button(ui, &mut a_transformation);
                            });
                        });

                        ui.label("  ");
                        ui.vertical(|ui| {
                            ui.label("          B");
                            ui.horizontal(|ui| {
                                ui.label("x:");
                                ui.add(
                                    egui::DragValue::new(&mut b_transformation.x)
                                        .custom_formatter(|_, _| format!("{}", floor.b.position.x))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut b_transformation.y)
                                        .custom_formatter(|_, _| format!("{}", floor.b.position.y))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut b_transformation.z)
                                        .custom_formatter(|_, _| format!("{}", floor.b.position.z))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("   ");
                                egui::reset_button(ui, &mut b_transformation);
                            });
                        });

                        ui.label("  ");
                        ui.vertical(|ui| {
                            ui.label("          C");
                            ui.horizontal(|ui| {
                                ui.label("x:");
                                ui.add(
                                    egui::DragValue::new(&mut c_transformation.x)
                                        .custom_formatter(|_, _| format!("{}", floor.c.position.x))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut c_transformation.y)
                                        .custom_formatter(|_, _| format!("{}", floor.c.position.y))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut c_transformation.z)
                                        .custom_formatter(|_, _| format!("{}", floor.c.position.z))
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("   ");
                                egui::reset_button(ui, &mut c_transformation);
                            });
                        });
                    });

                    ui.separator();

                    ui.heading("UV scaling");
                    ui.horizontal(|ui| {
                        ui.label("u:");
                        ui.add(egui::DragValue::new(&mut uv_scaling.x).speed(0.01));
                    });
                    ui.horizontal(|ui| {
                        ui.label("v:");
                        ui.add(egui::DragValue::new(&mut uv_scaling.y).speed(0.01));
                    });

                    ui.separator();

                    ui.heading("UV offset");
                    ui.horizontal(|ui| {
                        ui.label("u:");
                        ui.add(egui::DragValue::new(&mut uv_offset.x).speed(0.01));
                    });
                    ui.horizontal(|ui| {
                        ui.label("v:");
                        ui.add(egui::DragValue::new(&mut uv_offset.y).speed(0.01));
                    });

                    ui.separator();

                    ui.heading("UV rotation");
                    ui.horizontal(|ui| {
                        ui.label("degrees:");
                        ui.add(egui::Slider::new(&mut uv_rotation, 0.0..=360.).suffix("°"));
                    });

                    ui.separator();

                    ui.heading("World aligned UV");
                    ui.horizontal(|ui| {
                        let text = match floor.world_aligned_uv {
                            true => "On",
                            false => "Off",
                        };
                        ui.label("Toggle:");
                        if ui.button(text).clicked() {
                            match floor.world_aligned_uv {
                                true => floor.world_aligned_uv = false,
                                false => floor.world_aligned_uv = true,
                            }
                        }
                    });

                    ui.separator();

                    let selected_text = asset_server.texture_paths[texture_id].clone();
                    ui.heading("Texture");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{selected_text}"))
                        .show_ui(ui, |ui| {
                            for (i, texture) in texture_paths.iter().enumerate() {
                                ui.selectable_value(&mut texture_id, i, texture);
                            }
                        });

                    floor.a.transformation = a_transformation;
                    floor.a.position = floor.a.original_position + a_transformation;

                    floor.b.transformation = b_transformation;
                    floor.b.position = floor.b.original_position + b_transformation;

                    floor.c.transformation = c_transformation;
                    floor.c.position = floor.c.original_position + c_transformation;

                    floor.uv_scalar = uv_scaling;
                    floor.uv_offset = uv_offset;
                    floor.uv_rotation = uv_rotation;
                    floor.texture_id = texture_id;

                    map.floors[floor.id - 1000].a.position = floor.a.position;
                    map.floors[floor.id - 1000].b.position = floor.b.position;
                    map.floors[floor.id - 1000].c.position = floor.c.position;
                    map.floors[floor.id - 1000].uv_scalar = floor.uv_scalar;
                    map.floors[floor.id - 1000].uv_offset = floor.uv_offset;
                    map.floors[floor.id - 1000].uv_rotation = floor.uv_rotation;
                    map.floors[floor.id - 1000].world_aligned_uv = floor.world_aligned_uv;
                    map.floors[floor.id - 1000].texture_id = floor.texture_id;
                }
            }
        }
        Err(_) => {
            println!("Error: No map found");
        }
    });
}
