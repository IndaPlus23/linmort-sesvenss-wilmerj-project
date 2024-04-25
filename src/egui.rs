use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::f32::consts::PI;
use crate::map::Map;
use crate::Player;
use crate::SceneAssets;
use crate::Wall;
use crate::floor::Floor;

pub fn ui_example_system(
    mut player_query:  Query<&mut Player>,
    mut map_query: Query<&mut Map>,
    mut contexts: EguiContexts,
    asset_server: Res<SceneAssets>,
    mut wall_query: Query<&mut Wall>,
    mut floor_query: Query<&mut Floor>,
) {
    egui::Window::new("Editor").show(contexts.ctx_mut(), |ui| match map_query.get_single_mut() {
        Ok(mut map) => {
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
                        ui.add(egui::DragValue::new(&mut player_transformation.x).speed(1.0)
                        .custom_formatter(|n, _| {
                            format!("{:.0}", n)
                        }));
                    });
                    ui.horizontal(|ui| {
                        ui.label("       y:");
                        ui.add(egui::DragValue::new(&mut player_transformation.y).speed(1.0)
                        .custom_formatter(|n, _| {
                            format!("{:.0}", n)
                        }));
                    });
                    ui.horizontal(|ui| {
                        ui.label("z:");
                        ui.add(egui::DragValue::new(&mut player_transformation.z).speed(1.0)
                        .custom_formatter(|n, _| {
                            format!("{:.0}", n)
                        }));
                    });
                });

                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Yaw:");
                        ui.add(egui::DragValue::new(&mut player_transformation.yaw)
                        .speed(PI / 180.)
                        .custom_formatter(|n, _| {
                            let n = n as f32;
                            let degrees = n.rem_euclid(2.0 * PI) as f32 * 180. / PI;
                            format!("{:.0}", degrees)
                        })
                        .suffix("°"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Pitch:");
                        ui.add(egui::DragValue::new(&mut player_transformation.pitch)
                        .speed(PI / 180.)
                        .custom_formatter(|n, _| {
                            let n = n as f32;
                            let degrees = n.clamp(-PI / 2.0, PI / 2.0) as f32 * 180. / PI;
                            format!("{:.0}", degrees)
                        })
                        .suffix("°"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("   ");
                        egui::reset_button_with(ui, &mut player_transformation, Player::new(0., 5., 0., 0., 0.));
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
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_start.x;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut start_transformation.y)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_start.y;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut start_transformation.z)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_start.z;
                                            format!("{displayed}")
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
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_end.x;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut end_transformation.y)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_end.y;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut end_transformation.z)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_end.z;
                                            format!("{displayed}")
                                        })
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
                        ui.add(egui::Slider::new(&mut uv_scaling.x, 1.0..=10.));
                    });
                    ui.horizontal(|ui| {
                        ui.label("v:");
                        ui.add(egui::Slider::new(&mut uv_scaling.y, 1.0..=10.));
                    });
                    ui.separator();

                    ui.heading("UV offset");
                    ui.horizontal(|ui| {
                        ui.label("u:");
                        ui.add(egui::Slider::new(&mut uv_offset.x, 0.0..=1.));
                    });
                    ui.horizontal(|ui| {
                        ui.label("v:");
                        ui.add(egui::Slider::new(&mut uv_offset.y, 0.0..=1.));
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
                if floor.id == map.selected_id {
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
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_a.x;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut a_transformation.y)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_a.y;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut a_transformation.z)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_a.z;
                                            format!("{displayed}")
                                        })
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
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_b.x;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut b_transformation.y)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_b.y;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut b_transformation.z)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_b.z;
                                            format!("{displayed}")
                                        })
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
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_c.x;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut c_transformation.y)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_c.y;
                                            format!("{displayed}")
                                        })
                                        .speed(1.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label("z:");
                                ui.add(
                                    egui::DragValue::new(&mut c_transformation.z)
                                        .custom_formatter(|n, _| {
                                            let displayed = n as f32 + current_c.z;
                                            format!("{displayed}")
                                        })
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
                            true => {
                                "On"
                            }
                            false => {
                                "Off"
                            }
                        };
                        ui.label("Toggle:");
                        if ui.button(text).clicked() {
                            match floor.world_aligned_uv {
                                true => {
                                    floor.world_aligned_uv = false
                                }
                                false => {
                                    floor.world_aligned_uv = true
                                }
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

                    map.floors[floor.id - n_walls].a.position = floor.a.position;
                    map.floors[floor.id - n_walls].b.position = floor.b.position;
                    map.floors[floor.id - n_walls].c.position = floor.c.position;
                    map.floors[floor.id - n_walls].uv_scalar = floor.uv_scalar;
                    map.floors[floor.id - n_walls].uv_offset = floor.uv_offset;
                    map.floors[floor.id - n_walls].uv_rotation = floor.uv_rotation;
                    map.floors[floor.id - n_walls].world_aligned_uv = floor.world_aligned_uv;
                    map.floors[floor.id - n_walls].texture_id = floor.texture_id;
                }
            }
        }
        Err(_) => {
            println!("Error: No map found");
        }
    });
}
