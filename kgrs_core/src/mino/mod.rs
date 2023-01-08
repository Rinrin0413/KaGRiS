use crate::board::Board;
use bevy::{ecs::schedule::ShouldRun, prelude::*, sprite::MaterialMesh2dBundle};
use control::MinoControlPlugin;
use controlled::*;
use kgrs_const::color::mino_color;
use mesh::MinoInfo;
use rand::{thread_rng, Rng};
use util::*;
use IsMino::*;

pub struct MinoPlugin;

impl Plugin for MinoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_mino_ctrl)
            .add_plugin(MinoControlPlugin)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(is_waiting_mino)
                    .with_system(spawn_mino),
            );
    }
}

/// Returns `ShouldRun` based on `MinoCtrl::is_waiting`.
fn is_waiting_mino(mut mino_ctrl_query: Query<&MinoCtrl>) -> ShouldRun {
    if mino_ctrl_query.single_mut().is_waiting {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

/// Spawns a new controlled mino.
fn spawn_mino(
    mut mino_ctrl_query: Query<&mut MinoCtrl>,
    mut cmds: Commands,
    mut board_query: Query<(Entity, &mut Board)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mino_ctrl = mino_ctrl_query.single_mut();
    let mino_kind = rand_mino(mino_ctrl.nth, mino_ctrl.seed);

    let (board_entity, mut board_component) = board_query.single_mut();

    let board_width = board_component.width;
    let board_height = board_component.height;
    let one_cell = board_width / 10.;
    let half_cell = one_cell / 2.;
    let spawn_origin = Vec2::new(
        -one_cell - half_cell,
        board_height / 2. + one_cell * 3. - half_cell,
    );
    let (spawn_origin_in_board_x, spawn_origin_in_board_y) = (3, 17);

    for x in 0..4 {
        for y in 0..2 {
            if let M = mino_kind.shape()[y][x] {
                cmds.entity(board_entity).with_children(|c| {
                    let pos_in_board = UVec2::new(
                        (spawn_origin_in_board_x + x) as u32,
                        (spawn_origin_in_board_y + y) as u32,
                    );
                    board_component
                        .data
                        .spawn_mino(CellState::Controlled, pos_in_board);
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
                    .insert(MinoInfo::new(
                        mino_kind,
                        pos_in_board,
                        UVec2::new(x as u32, y as u32),
                    ));
                });
            }
        }
    }

    println!("Current board:\n{}", board_component.data); // DEBUG

    mino_ctrl.nth += 1;
    mino_ctrl.is_waiting = false;
}

/// The kind of mino.
#[derive(Clone, Copy, Debug, PartialEq)]
#[rustfmt::skip]
pub(crate) enum MinoType {
    I, O, L, J, Z, S, T,
    Garbage,
}

impl MinoType {
    /// Returns the 4x2 shape of the mino.
    ///
    /// # Panics
    ///
    /// If `MinoType::Garbage` was passed.
    #[rustfmt::skip]
    fn shape(&self) -> [[IsMino; 4]; 2] {
        match self {
            Self::I => [
                [E, E, E, E],
                [M, M, M, M]
            ],
            Self::O => [
                [E, M, M, E],
                [E, M, M, E]
            ],
            Self::L => [
                [E, E, M, E],
                [M, M, M, E]
            ],
            Self::J => [
                [M, E, E, E],
                [M, M, M, E]
            ],
            Self::Z => [
                [M, M, E, E],
                [E, M, M, E]
            ],
            Self::S => [
                [E, M, M, E],
                [M, M, E, E]
            ],
            Self::T => [
                [E, M, E, E],
                [M, M, M, E]
            ],
            Self::Garbage => {
                unreachable!()
            }
        }
    }

    /// Returns relative axis point of the mino.
    ///
    /// # Panics
    ///
    /// If `MinoType::Garbage` was passed.
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

    /// Return a color of the mino.
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

    /// Returns side margin of the mino in 4x2 shape.
    ///
    /// # Panics
    ///
    /// If `MinoType::Garbage` was passed.
    fn side_margin(&self) -> (isize, isize) {
        match self {
            Self::I => (0, 0),
            Self::O => (1, 1),
            Self::L => (0, 1),
            Self::J => (0, 1),
            Self::Z => (0, 1),
            Self::S => (0, 1),
            Self::T => (0, 1),
            Self::Garbage => {
                unreachable!()
            }
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

mod controlled {
    use super::*;

    #[derive(Component)]
    pub(crate) struct MinoCtrl {
        /// nth of the mino (0-indexed)
        pub(crate) nth: usize,
        /// Seed for RNG.
        pub(crate) seed: u64,
        /// Whether waiting for the next mino.
        pub(crate) is_waiting: bool,
    }

    impl MinoCtrl {
        fn init() -> Self {
            Self {
                nth: 0,
                seed: thread_rng().gen_range(0..1000000000),
                is_waiting: true,
            }
        }
    }

    pub(crate) fn set_mino_ctrl(mut cmds: Commands) {
        cmds.spawn(MinoCtrl::init());
    }
}

/// Mino data for the board
pub(crate) struct MinoData([[CellState; 10]; 40]);

impl MinoData {
    /// Creates a empty mino data.
    pub(crate) fn empty() -> Self {
        Self([[CellState::Empty; 10]; 40])
    }

    /// Spawns a mino to specified position.
    pub(crate) fn spawn_mino(&mut self, state: CellState, pos: UVec2) {
        self.0[pos.y as usize][pos.x as usize] = state;
    }

    /// Moves the controlled mino.
    pub(crate) fn move_mino(
        &mut self,
        direction: MoveDirection,
        mino_mesh_query: &mut Query<(&mut MinoInfo, &mut Transform)>,
        cell_size: f32,
    ) {
        for r in 0..self.0.len() {
            let row = &mut self.0[r];
            let row_len = row.len();
            for mut c in 0..row_len {
                // Reverse the index if the direction is right.
                // Because if move in order from the left minoes,
                // the minoes will be overlapped by the right minoes.
                c = if direction.is_left() {
                    c
                } else {
                    row.len() - c - 1
                };
                if row[c].is_controlled() {
                    for (mut mesh, mut tf) in mino_mesh_query.iter_mut() {
                        if mesh.position == UVec2::new(c as u32, r as u32) {
                            let part_pos = mesh.part.x as isize;
                            let side_margin = mesh.kind.side_margin();
                            let is_not_mino_on_edge = if direction.is_left() {
                                !(c as isize - part_pos + side_margin.0 == 0)
                            } else {
                                !(c as isize + (3 - part_pos) - side_margin.1
                                    == row_len as isize - 1)
                            };

                            if is_not_mino_on_edge {
                                // Remove the mino to move.
                                row[c] = CellState::Empty;

                                // The cell that mino moves to turns into controlled mino.
                                let moved_pos = if direction.is_left() { c - 1 } else { c + 1 };
                                row[moved_pos] = CellState::Controlled;

                                // Update rendering position.
                                mesh.position = UVec2::new(moved_pos as u32, r as u32);
                                tf.translation.x += if direction.is_left() {
                                    -cell_size
                                } else {
                                    cell_size
                                };
                            }
                        }
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for MinoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (r, row) in self.0.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let frame = |at: usize| -> String {
                    String::from(if c == at {
                        if 19 < r {
                            "!"
                        } else {
                            " "
                        }
                    } else {
                        ""
                    })
                };
                write!(f, "{}", frame(0))?;
                match cell {
                    CellState::Empty => write!(f, " .")?,
                    CellState::Controlled => write!(f, "{{}}")?,
                    CellState::Placed => write!(f, "[]")?,
                }
                write!(f, "{}", frame(9))?;
            }
            writeln!(f)?;
            if r == 19 {
                writeln!(f, "!--------------------!")?;
            } else if r == 39 {
                writeln!(f, "======================")?;
            }
        }
        Ok(())
    }
}

/// State of the mino cell in the board
#[derive(Debug, Clone, Copy)]
pub(crate) enum CellState {
    /// Empty
    Empty,
    /// Controlled mino
    Controlled,
    /// Placed mino
    Placed,
}

impl CellState {
    /// Whether the cell is empty.
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Whether the cell is controlled mino.
    fn is_controlled(&self) -> bool {
        matches!(self, Self::Controlled)
    }

    /// Whether the cell is placed mino.
    fn is_placed(&self) -> bool {
        matches!(self, Self::Placed)
    }

    /// Whether the cell is filled with mino.
    fn is_filled(&self) -> bool {
        !self.is_empty()
    }
}

pub(crate) mod control;
pub(crate) mod mesh;
pub(crate) mod util;
