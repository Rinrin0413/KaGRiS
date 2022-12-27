use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized};
use kgrs_const::{color::mino_color, dimension::*};
use mino_ctrl::*;
use rand::{thread_rng, Rng};
use util::*;
use IsMino::*;

pub struct MinoPlugin;

impl Plugin for MinoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_mino_ctrl)
            .add_system(spawn_mino)
            .add_system(resize_minoes);
    }
}

fn spawn_mino(
    windows: Res<Windows>,
    mut mino_ctrl_query: Query<&mut MinoControl>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window_height = windows.get_primary().unwrap().height();
    let board_width = window_height * BOARD_WIDTH_RATIO;
    let board_height = window_height * BOARD_HEIGHT_RATIO;
    let one_cell = board_width / 10.;
    let half_cell = one_cell / 2.;
    let pos = Vec2::new(-board_width / 2. + half_cell, board_height / 2. - half_cell);

    let mut mino_ctrl = mino_ctrl_query.single_mut();
    mino_ctrl.nth += 1;
    let mino_kind = rand_mino(mino_ctrl.nth, mino_ctrl.seed);

    cmds.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad {
                size: Vec2::new(one_cell, one_cell),
                ..default()
            }))
            .into(),
        material: materials.add(ColorMaterial::from(mino_kind.color())),
        transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.15)),
        ..default()
    })
    .insert(Mino::spawn(mino_kind, one_cell, pos));
}

/// Resizes and repositions the minoes when the window is resized.
pub fn resize_minoes(
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<(&mut Mino, &mut Transform)>,
) {
    for window in resize_reader.iter() {
        let window_height = window.height;
        let board_width = window_height * BOARD_WIDTH_RATIO;
        let board_height = window_height * BOARD_HEIGHT_RATIO;
        let one_cell = board_width / 10.;
        let half_cell = one_cell / 2.;

        for (mino, mut tf) in query.iter_mut() {
            tf.scale = Vec3::new(one_cell / mino.size, one_cell / mino.size, 1.);
            tf.translation = Vec3::new(
                -board_width / 2. + half_cell,
                board_height / 2. - half_cell,
                1.,
            );
            // `Transform.scale` is relative size so don't update `mino.size`.
        }
    }
}

#[derive(Component)]
pub struct Mino {
    /// Type of the mino
    kind: MinoType,
    // The shape of the mino
    shape: [[IsMino; 4]; 4],
    /// Size of the mino
    size: f32,
    /// The axis relative point of the mino
    ///
    /// It is point within 4x4 range.
    axis_point: Vec2,
    // The position of the mino
    position: Vec2,
    // State of the mino
    state: MinoState,
}

impl Mino {
    fn spawn(kind: MinoType, size: f32, position: Vec2) -> Self {
        Self {
            shape: kind.shape(),
            axis_point: kind.axis_point(),
            size,
            kind,
            position,
            state: MinoState::Control,
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
                [E, E, E, E],
                [E, E, M, E],
                [M, M, M, E],
                [E, E, E, E]
            ],
            Self::J => [
                [E, E, E, E],
                [M, E, E, E],
                [M, M, M, E],
                [E, E, E, E]
            ],
            Self::Z => [
                [E, E, E, E],
                [E, M, M, E],
                [M, M, E, E],
                [E, E, E, E]
            ],
            Self::S => [
                [E, E, E, E],
                [M, M, E, E],
                [E, M, M, E],
                [E, E, E, E]
            ],
            Self::T => [
                [E, E, E, E],
                [E, M, E, E],
                [M, M, M, E],
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
            Self::L => Vec2::new(0.5, 0.5),
            Self::J => Vec2::new(0.5, 0.5),
            Self::Z => Vec2::new(0.5, 0.5),
            Self::S => Vec2::new(0.5, 0.5),
            Self::T => Vec2::new(0.5, 0.5),
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

mod mino_ctrl {
    use super::*;

    #[derive(Component)]
    pub(crate) struct MinoControl {
        /// nth of the mino (0-indexed)
        pub(crate) nth: usize,
        /// Seed for RNG.
        pub(crate) seed: u64,
    }

    impl MinoControl {
        fn init() -> Self {
            Self {
                nth: 0,
                seed: thread_rng().gen_range(0..1000000000),
            }
        }
    }

    pub(crate) fn set_mino_ctrl(mut cmds: Commands) {
        cmds.spawn(MinoControl::init());
    }
}

pub(crate) mod util;
