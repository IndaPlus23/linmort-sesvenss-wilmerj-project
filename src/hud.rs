use bevy::{
    prelude::*,
    render::color::Color::Rgba,
    render::{
        mesh::Indices, mesh::Mesh, render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
    sprite::Mesh2dHandle,
    window::PrimaryWindow,
};

use crate::Player;

#[derive(Component, Clone)]
pub struct RenderItem {
    pub id: usize,
    pub main_menu_exclusive: bool,
}

impl RenderItem {
    pub fn new_with_id(id: usize) -> Self {
        Self {
            id,
            main_menu_exclusive: false,
        }
    }

    pub fn new_main_menu_with_id(id: usize) -> Self {
        Self {
            id,
            main_menu_exclusive: true,
        }
    }

    pub fn new_mesh() -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.], [0., 0., 0.]],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
        )
        .with_inserted_indices(Indices::U32(vec![0, 3, 1, 1, 3, 2]))
    }
}

#[derive(Component, Clone)]
pub struct MainMenuText {
    pub id: usize,
    pub selected_id: usize,
    pub shadow: bool,
}

impl MainMenuText {
    pub fn new_with_id(id: usize, shadow: bool) -> Self {
        let selected_id = 0;
        Self {
            id,
            selected_id,
            shadow,
        }
    }
}

pub fn render_game_hud(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut hud_query: Query<(&mut RenderItem, &mut Mesh2dHandle)>,
) {
    let primary_window = window.single_mut();

    for (item, mesh2dhandle) in hud_query.iter_mut() {
        let mesh_handle = &mesh2dhandle.0;
        let mesh = meshes.get_mut(mesh_handle).unwrap();
        if let Some(_) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            if item.main_menu_exclusive {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.], [0., 0., 0.]],
                );
            } else {
                let scale = primary_window.width() / (64. * 2.) / 10.;

                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [
                            -primary_window.width() / 2.,
                            -primary_window.height() / 2. + (64. * 2.) * scale,
                            0.0,
                        ],
                        [
                            primary_window.width() / 2.,
                            -primary_window.height() / 2. + (64. * 2.) * scale,
                            0.0,
                        ],
                        [
                            primary_window.width() / 2.,
                            -primary_window.height() / 2.,
                            0.0,
                        ],
                        [
                            -primary_window.width() / 2.,
                            -primary_window.height() / 2.,
                            0.0,
                        ],
                    ],
                );
            }
        }
    }
}

pub fn render_main_menu(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut hud_query: Query<(&mut RenderItem, &mut Mesh2dHandle)>,
) {
    let primary_window = window.single_mut();

    for (item, mesh2dhandle) in hud_query.iter_mut() {
        if item.id == 0 {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();
            if let Some(_) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [
                            -primary_window.width() / 2.,
                            primary_window.height() / 2.,
                            0.0,
                        ],
                        [
                            primary_window.width() / 2.,
                            primary_window.height() / 2.,
                            0.0,
                        ],
                        [
                            primary_window.width() / 2.,
                            -primary_window.height() / 2.,
                            0.0,
                        ],
                        [
                            -primary_window.width() / 2.,
                            -primary_window.height() / 2.,
                            0.0,
                        ],
                    ],
                );
            }
        }
    }
}

pub fn main_menu_text(
    time: Res<'_, Time<Real>>,
    mut text_query: Query<(&mut MainMenuText, &mut Text, &mut Transform)>,
) {
    for (main_menu_text, mut text, mut transform) in text_query.iter_mut() {
        if !main_menu_text.shadow {
            transform.scale = Vec3::ONE;

            if main_menu_text.selected_id == main_menu_text.id {
                text.sections[0].style.color = animate_color(time.elapsed_seconds())
            } else {
                text.sections[0].style.color = Color::RED
            }
        } else {
            transform.scale = Vec3::new(1.05, 1.2, 1.);
        }
    }
}

#[derive(Component, Clone)]
pub struct GameScreenText {
    pub id: usize,
    pub shadow: bool,
}

impl GameScreenText {
    pub fn new_with_id(id: usize, shadow: bool) -> Self {
        Self { id, shadow }
    }
}

pub fn game_screen_text(
    mut player_query: Query<&Player>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut text_query: Query<(&mut GameScreenText, &mut Text, &mut Transform)>,
) {
    let primary_window = window.single_mut();

    for player in player_query.iter_mut() {
        for (game_screen_text, mut text, mut transform) in text_query.iter_mut() {
            if game_screen_text.id == 0 {
                text.sections[0].value = format!("{}%", player.health);

                let scale1 = primary_window.width() / (640.);
                let scale2 = primary_window.width() / (64. * 2.) / 10.;

                transform.translation = Vec3::new(
                    primary_window.width() / 2. - 57. * scale1,
                    -primary_window.height() / 2. + 80. * scale2,
                    100.0,
                );

                if !game_screen_text.shadow {
                    transform.scale = Vec3::new(scale2, scale2, scale2);
                } else {
                    transform.translation.z = 90.;
                    transform.scale = Vec3::new(1.05 * scale2, 1.2 * scale2, scale2);
                }
            }

            if game_screen_text.id == 1 {
                text.sections[0].value = player.ammo.to_string();

                let scale1 = primary_window.width() / (640.);
                let scale2 = primary_window.width() / (64. * 2.) / 10.;

                transform.translation = Vec3::new(
                    -primary_window.width() / 2. + 46. * scale1,
                    -primary_window.height() / 2. + 80. * scale2,
                    100.0,
                );

                if !game_screen_text.shadow {
                    transform.scale = Vec3::new(scale2, scale2, scale2);
                } else {
                    transform.translation.z = 90.;
                    transform.scale = Vec3::new(1.05 * scale2, 1.2 * scale2, scale2);
                }
            }
        }
    }
}

fn animate_color(time: f32) -> Color {
    Rgba {
        red: 1.0,
        green: (0.5 * (time * 7.0).sin() + 0.5),
        blue: (0.5 * (time * 7.0).sin() + 0.5),
        alpha: 1.0,
    }
}
