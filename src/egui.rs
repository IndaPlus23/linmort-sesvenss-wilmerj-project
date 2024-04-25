use crate::SceneAssets;
use crate::Wall;
use crate::map::Map;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn ui_example_system(
    mut map_query: Query<&mut Map>,
    mut contexts: EguiContexts,
    asset_server: Res<SceneAssets>,
    mut wall_query: Query<&mut Wall>,
) {
    egui::Window::new("Editor").show(contexts.ctx_mut(), |ui| {
        match map_query.get_single_mut() {
            Ok(mut map) => {
                let number_of_walls = map.walls.len();

                ui.heading("Select compontent");
                ui.horizontal(|ui| {
                    ui.label("id:");
                    ui.add(egui::Slider::new(&mut map.selected_id, 0..=number_of_walls - 1));
                });
                ui.separator();

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
                
                        let texture_paths = &asset_server.texture_paths;
        
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
                    }
                }
            }
            Err(_) => {
                println!("Error: No map found");
            }
        }
    });
}
