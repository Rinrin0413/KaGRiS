use crate::board::Board;
use bevy::{ecs::schedule::ShouldRun, prelude::*, sprite::MaterialMesh2dBundle};
use ctrl::*;
use kgrs_const::color::mino_color;
use rand::{thread_rng, Rng};
use util::*;
use IsMino::*;

pub struct MinoPlugin;

impl Plugin for MinoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_mino_ctrl)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(is_waiting_mino)
                    .with_system(spawn_mino),
            )
            // .add_system(resize_minoes) // DEBUG: probably `resize_minoes` is useless
            .add_system(place_mino);
    }
}

fn is_waiting_mino(mut mino_ctrl_query: Query<&MinoControl>) -> ShouldRun {
    if mino_ctrl_query.single_mut().is_waiting {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn spawn_mino(
    mut mino_ctrl_query: Query<&mut MinoControl>,
    mut cmds: Commands,
    board_query: Query<(Entity, &Board)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mino_ctrl = mino_ctrl_query.single_mut();
    let mino_kind = rand_mino(mino_ctrl.nth, mino_ctrl.seed);

    let (board_entity, board_component) = board_query.single();

    let board_width = board_component.width;
    let board_height = board_component.height;
    let one_cell = board_width / 10.;
    let half_cell = one_cell / 2.;
    let spawn_origin = Vec2::new(
        -one_cell - half_cell,
        board_height / 2. + one_cell * 3. - half_cell,
    );

    for x in 0..4 {
        for y in 0..4 {
            if mino_kind.shape()[y][x] == M {
                cmds.entity(board_entity).with_children(|c| {
                    c.spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Quad {
                                size: Vec2::new(one_cell, one_cell),
                                ..default()
                            }))
                            .into(),
                        material: materials.add(ColorMaterial::from(mino_kind.color())),
                        transform: Transform::from_translation(Vec3::new(
                            spawn_origin.x + x as f32 * one_cell,
                            spawn_origin.y - y as f32 * one_cell,
                            0.15,
                        )),
                        ..default()
                    })
                    .insert(Mino::spawn(
                        mino_kind,
                        one_cell,
                        spawn_origin,
                        Vec2::new(x as f32, y as f32),
                    ));
                });
            }
        }
    }
    mino_ctrl.nth += 1;
    mino_ctrl.is_waiting = false;
}

// DEBUG: probably it is useless
// /// Resizes and repositions the minoes when the window is resized.
// pub(crate) fn resize_minoes(
//     mut resize_reader: EventReader<WindowResized>,
//     mut query: Query<(&mut Mino, &mut Transform)>,
// ) {
//     for window in resize_reader.iter() {
//         let window_height = window.height;
//         let board_width = window_height * BOARD_WIDTH_RATIO;
//         let board_height = window_height * BOARD_HEIGHT_RATIO;
//         let one_cell = board_width / 10.;
//         let half_cell = one_cell / 2.;

//         for (mino, mut tf) in query.iter_mut() {
//             tf.scale = Vec3::new(one_cell / mino.size, one_cell / mino.size, 1.);
//             tf.translation = Vec3::new(
//                 -board_width / 2. + half_cell,
//                 board_height / 2. - half_cell,
//                 1.,
//             );
//             // `Transform.scale` is relative size so don't update `mino.size`.
//         }
//     }
// }

#[derive(Component)]
pub struct Mino {
    /// Type of the mino
    kind: MinoType,
    // // The shape of the mino // TODO: Consider Kentou Consider
    // shape: [[IsMino; 4]; 4], // TODO: Consider Kentou Consider
    /// Size of the mino
    size: f32,
    // /// The axis relative point of the mino
    // ///
    // /// It is point within 4x4 range.
    // axis_point: Vec2,
    /// The position of the mino
    position: Vec2,
    /// State of the mino
    state: MinoState,
    /// Relative block position in the mino
    part: Vec2,
}

impl Mino {
    fn spawn(kind: MinoType, size: f32, position: Vec2, part: Vec2) -> Self {
        Self {
            // shape: kind.shape(), // TODO: Consider Kentou Consider
            // axis_point: kind.axis_point(),
            kind,
            size,
            position,
            state: MinoState::Control,
            part,
        }
    }
}

/// Type of the mino
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum MinoType {
    I, O, L, J, Z, S, T,
    Garbage,
}

impl MinoType {
    /// Get default shape of the mino
    ///
    /// # Panics
    ///
    /// If `MinoType::Garbage` is passed.
    #[rustfmt::skip]
    fn shape(&self) -> [[IsMino; 4]; 4] {
        match self {
            Self::I => [
                [E, E, E, E],
                [M, M, M, M],
                [E, E, E, E],
                [E, E, E, E]
            ],
            Self::O => [
                [E, M, M, E],
                [E, M, M, E],
                [E, E, E, E],
                [E, E, E, E]
            ],
            Self::L => [
                [E, E, M, E],
                [M, M, M, E],
                [E, E, E, E],
                [E, E, E, E]
            ],
            Self::J => [
                [M, E, E, E],
                [M, M, M, E],
                [E, E, E, E],
                [E, E, E, E]
            ],
            Self::Z => [
                [M, M, E, E],
                [E, M, M, E],
                [E, E, E, E],
                [E, E, E, E]
            ],
            Self::S => [
                [E, M, M, E],
                [M, M, E, E],
                [E, E, E, E],
                [E, E, E, E]
            ],
            Self::T => [
                [E, M, E, E],
                [M, M, M, E],
                [E, E, E, E],
                [E, E, E, E]
            ],
            Self::Garbage => {
                unreachable!()
            }
        }
    }

    /// Get relative axis point of the mino
    ///
    /// # Panics
    ///
    /// If `MinoType::Garbage` is passed.
    fn axis_point(&self) -> Vec2 {
        match self {
            Self::I => Vec2::new(0.0, 0.0),
            Self::O => Vec2::new(0.0, 1.0),
            Self::L => Vec2::new(-0.5, 0.5),
            Self::J => Vec2::new(-0.5, 0.5),
            Self::Z => Vec2::new(-0.5, 0.5),
            Self::S => Vec2::new(-0.5, 0.5),
            Self::T => Vec2::new(-0.5, 0.5),
            Self::Garbage => {
                unreachable!()
            }
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::I => mino_color::I,
            Self::O => mino_color::O,
            Self::L => mino_color::L,
            Self::J => mino_color::J,
            Self::Z => mino_color::Z,
            Self::S => mino_color::S,
            Self::T => mino_color::T,
            Self::Garbage => mino_color::GARBAGE,
        }
    }
}

/// Whether a block is a part of the mino.
#[derive(PartialEq)]
enum IsMino {
    // Mino
    M,
    // Empty
    E,
}

/// State of the mino
enum MinoState {
    Placed,
    Control,
    Next,
    Hold,
}

mod ctrl {
    use super::*;

    #[derive(Component)]
    pub(crate) struct MinoControl {
        /// nth of the mino (0-indexed)
        pub(crate) nth: usize,
        /// Seed for RNG.
        pub(crate) seed: u64,
        /// Whether waiting for the next mino.
        pub(crate) is_waiting: bool,
    }

    impl MinoControl {
        fn init() -> Self {
            Self {
                nth: 0,
                seed: thread_rng().gen_range(0..1000000000),
                is_waiting: true,
            }
        }
    }

    pub(crate) fn set_mino_ctrl(mut cmds: Commands) {
        cmds.spawn(MinoControl::init());
    }

    pub(crate) fn place_mino(
        mut mino_ctrl_query: Query<&mut MinoControl>,
        input: Res<Input<KeyCode>>,
    ) {
        if input.just_pressed(KeyCode::Space) {
            mino_ctrl_query.single_mut().is_waiting = true;
        }
    }
}

pub(crate) mod util;
