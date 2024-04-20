use crate::CustomMaterial;
use crate::SceneAssets;
use crate::Wall;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};


pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<SceneAssets>,
    mut wall_query: Query<(&mut Wall, &mut Transform, &mut Handle<CustomMaterial>)>,
) {
    egui::Window::new("Editor").show(contexts.ctx_mut(), |ui| {
        let mut start_transformation = Vec3::new(0., 0., 0.);
        let mut end_transformation = Vec3::new(0., 0., 0.);

        let mut current_start = Vec3::new(0., 0., 0.);
        let mut current_end = Vec3::new(0., 0., 0.);

        let mut height = 0.;
        let mut uv_scaling = Vec2::new(1., 1.);
        let mut uv_offset = Vec2::new(0., 0.);
        let mut uv_rotation = 0.;

        let mut material = &mut CustomMaterial {
            texture: asset_server.textures[0].clone(),
            a: Vec3::ZERO,
            b: Vec3::ZERO,
            c: Vec3::ZERO,
            a_uv: Vec2::ZERO,
            b_uv: Vec2::ZERO,
            c_uv: Vec2::ZERO,
            uv_scalar: Vec2::new(1., 1.),
            uv_offset: Vec2::new(0., 0.),
            uv_rotation: 0.,
        };

        for (wall, _, material_handle) in wall_query.iter_mut() {
            start_transformation = wall.start.transformation;
            current_start = wall.start.position;

            end_transformation = wall.end.transformation;
            current_end = wall.end.position;

            height = wall.height;
            uv_scaling = wall.uv_scalar;
            uv_offset = wall.uv_offset;
            uv_rotation = wall.uv_rotation;

            let material_handle = material_handle.clone();
            material = custom_materials.get_mut(material_handle).unwrap();
        }

        let texture_paths = &asset_server.texture_paths;

        ui.heading("Transform");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Start");
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
                ui.label("End");
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
            ui.label("x:");
            ui.add(egui::Slider::new(&mut uv_scaling.x, 1.0..=10.));
        });
        ui.horizontal(|ui| {
            ui.label("y:");
            ui.add(egui::Slider::new(&mut uv_scaling.y, 1.0..=10.));
        });
        ui.separator();

        ui.heading("UV offset");
        ui.horizontal(|ui| {
            ui.label("x:");
            ui.add(egui::Slider::new(&mut uv_offset.x, 0.0..=1.));
        });
        ui.horizontal(|ui| {
            ui.label("y:");
            ui.add(egui::Slider::new(&mut uv_offset.y, 0.0..=1.));
        });
        ui.separator();

        ui.heading("UV rotation");
        ui.horizontal(|ui| {
            ui.label("degrees:");
            ui.add(egui::Slider::new(&mut uv_rotation, 0.0..=360.).suffix("°"));
        });

        ui.separator();
        ui.heading("Texture");
        egui::ComboBox::from_label("")
            .selected_text(format!("Texture"))
            .show_ui(ui, |ui| {
                for (i, texture) in texture_paths.iter().enumerate() {
                    ui.selectable_value(
                        &mut material.texture,
                        asset_server.textures[i].clone(),
                        texture,
                    );
                }
            });

        for (mut wall, _transform, _) in wall_query.iter_mut() {
            wall.start.transformation = start_transformation;
            wall.start.position = wall.start.original_position + start_transformation;

            wall.end.transformation = end_transformation;
            wall.end.position = wall.end.original_position + end_transformation;

            wall.height = height;

            wall.uv_scalar = uv_scaling;
            wall.uv_offset = uv_offset;
            wall.uv_rotation = uv_rotation;

            //wall.texture = material.texture.clone();
        }
 
    });
}