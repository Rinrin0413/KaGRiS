use crate::mino::MinoData;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized};
use kgrs_config::Config;
use kgrs_const::{color::*, dimension::*};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_board).add_system(adjust_board);
    }
}

/// The board
#[derive(Component)]
pub(crate) struct Board {
    /// The data of the board.
    pub(crate) data: MinoData,
    /// Initial width of the board.
    pub(crate) width: f32,
    /// Initial height of the board.
    pub(crate) height: f32,
}

/// The grid of the board
#[derive(Component)]
struct Grid;

/// Setups the board
fn setup_board(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: ResMut<Windows>,
) {
    info!("Setting up board");
    let window_height = windows.get_primary().unwrap().height();
    let board_width = window_height * BOARD_WIDTH_RATIO;
    let board_height = window_height * BOARD_HEIGHT_RATIO;

    cmds.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad {
                size: Vec2::new(board_width, board_height),
                ..default()
            }))
            .into(),
        material: materials.add(ColorMaterial::from(BOARD_COL)),
        ..default()
    })
    .insert(Board {
        data: MinoData::empty(),
        width: board_width,
        height: board_height,
    })
    // Grids
    .with_children(|c| {
        let opac = Config::load().grid_opacity;
        if 0 < opac {
            let mut draw_grid = |is_horiz: bool| {
                let (board_len, grid_num, size) = if is_horiz {
                    (board_height, 20, Vec2::new(board_width, GRID_THICKNESS))
                } else {
                    (board_width, 10, Vec2::new(GRID_THICKNESS, board_height))
                };

                for i in 1..grid_num {
                    let mut p = board_len * i as f32 / grid_num as f32;
                    if board_len / 2. <= p {
                        p = (p - board_len / 2.) * -1.;
                    }
                    let offset = if is_horiz {
                        Vec2::new(0., p)
                    } else {
                        Vec2::new(p, 0.)
                    };
                    c.spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Quad { size, ..default() }))
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::rgba(
                            GRID_COL.r(),
                            GRID_COL.g(),
                            GRID_COL.b(),
                            opac as f32 / 100.,
                        ))),
                        transform: Transform::from_xyz(offset.x, offset.y, 0.1),
                        ..default()
                    })
                    .insert(Grid);
                }
            };

            draw_grid(true);
            draw_grid(false);

            // Frame

            let half_frame_thick = FRAME_THICKNESS / 2.;
            let double_frame_thick = FRAME_THICKNESS * 2.;

            for (offset, is_horiz) in [
                //(board_height / 2. +half_frame_thick, true),  // Top
                (-board_height / 2. - half_frame_thick, true), // Bottom
                (board_width / 2. + half_frame_thick, false),  // Right
                (-board_width / 2. - half_frame_thick, false), // Left
            ] {
                // Corners will be broken so added thickness to frame length.
                let (size, offset) = if is_horiz {
                    (
                        Vec2::new(board_width + double_frame_thick, FRAME_THICKNESS),
                        Vec2::new(0., offset),
                    )
                } else {
                    (
                        Vec2::new(FRAME_THICKNESS, board_height + double_frame_thick),
                        Vec2::new(offset, 0.),
                    )
                };
                c.spawn((MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad { size, ..default() }))
                        .into(),
                    material: materials.add(ColorMaterial::from(FRAME_COL)),
                    transform: Transform::from_xyz(offset.x, offset.y, 0.2),
                    ..default()
                },));
            }
        }
    });
}

/// Resizes and repositions the board when the window is resized.
fn adjust_board(
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<(&Board, &mut Transform)>,
) {
    for window in resize_reader.iter() {
        for (board, mut tf) in query.iter_mut() {
            tf.scale = Vec3::new(
                window.height * BOARD_WIDTH_RATIO / board.width,
                window.height * BOARD_HEIGHT_RATIO / board.height,
                1.0,
            );
            // `Transform.scale` is relative ratio from the initial size
            // so don't update `board.width` and `board.height`.

            // It is: initial_height * relative_ratio = current_true_height
            let true_board_height = board.height * tf.scale[1];
            // Position the board bottom at the window bottom.
            let pos_bottom = -(window.height - true_board_height) / 2. + GRID_THICKNESS * 2.;
            // Offset based on `BOARD_OFFSET_RATIO_Y`.
            let offset = window.height * BOARD_OFFSET_RATIO_Y;
            tf.translation[1] = pos_bottom + offset;
        }
    }
}
