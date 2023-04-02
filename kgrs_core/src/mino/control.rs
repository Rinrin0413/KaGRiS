use super::*;
use kgrs_config::Config;
use kgrs_util::function::fixed_update;

pub(crate) struct MinoControlPlugin;

impl Plugin for MinoControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_mino_ctrl)
            .add_system(place_mino)
            .add_system_set(fixed_update(move_mino));
    }
}

/// Initializes `MinoCtrl`.
pub(crate) fn set_mino_ctrl(mut cmds: Commands) {
    cmds.spawn(MinoCtrl::init());
}

/// Hard drops the mino.
fn place_mino(mut mino_ctrl_query: Query<&mut MinoCtrl>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        mino_ctrl_query.single_mut().is_waiting = true;
    }
}

/// Moves the mino horizontally.
fn move_mino(
    input: Res<Input<KeyCode>>,
    mut mino_ctrl_query: Query<&mut MinoCtrl>,
    mut board_query: Query<&mut Board>,
    mut mino_mesh_query: Query<(&mut MinoInfo, &mut Transform)>,
) {
    let keys_iter = input.get_pressed();
    let mut mino_ctrl = mino_ctrl_query.single_mut();

    if keys_iter.len() == 0 {
        mino_ctrl.movement = Movement::No;
    }

    for key in keys_iter {
        if key == &KeyCode::Left || key == &KeyCode::Right {
            let input_direction = if let KeyCode::Left = key {
                MoveDirection::Left
            } else {
                MoveDirection::Right
            };
            let is_dir_changed = match mino_ctrl.movement {
                Movement::No => false,
                Movement::InDas(_, direction) => {
                    direction != input_direction
                }
                Movement::InArr(_, direction) => {
                    direction != input_direction
                }
            };

            let cell_size = board_query.single().width / 10.;
            let board_data = &mut board_query.single_mut().data;

            // Initialize the movement
            if mino_ctrl.movement == Movement::No || is_dir_changed {
                board_data.move_mino(
                    input_direction,
                    &mut mino_mesh_query,
                    cell_size,
                );
                mino_ctrl.movement = Movement::InDas(0, input_direction);  
            }

            match mino_ctrl.movement {
                Movement::InDas(f, direction) => {
                    let das = Config::load().handling.das.get();
                    if f < das {
                        mino_ctrl.movement.increase();
                    } else {
                        board_data.move_mino(
                            direction,
                            &mut mino_mesh_query,
                            cell_size,
                        );
                        
                        mino_ctrl.movement = Movement::InArr(0, direction);
                    }
                }
                Movement::InArr(f, direction) => {
                    let arr = Config::load().handling.arr;
                    if arr == 0 {
                        for _ in 0..10 {
                            board_data.move_mino(
                                direction,
                                &mut mino_mesh_query,
                                cell_size,
                            );
                        }
                    } else if f < arr {
                        mino_ctrl.movement.increase();
                    } else {
                        board_data.move_mino(
                            direction,
                            &mut mino_mesh_query,
                            cell_size,
                        );
                        
                        mino_ctrl.movement = Movement::InArr(0, direction);
                    }
                }
                Movement::No => {}
            }
        }
    }
}

#[derive(Component)]
pub(crate) struct MinoCtrl {
    /// nth of the mino (0-indexed)
    pub(crate) nth: usize,
    /// Seed for RNG.
    pub(crate) seed: u64,
    /// Whether waiting for the next mino.
    pub(crate) is_waiting: bool,
    /// Horizontal movement of the controlled mino.
    pub(crate) movement: Movement,
}

impl MinoCtrl {
    fn init() -> Self {
        let seed = thread_rng().gen_range(0..1000000000);
        info!("Randomize seed: {:0>9}", seed);
        Self {
            nth: 0,
            seed,
            is_waiting: true,
            movement: Movement::No,
        }
    }
}

/// The horizontal movement of the controlled mino.
#[derive(PartialEq)]
pub(crate) enum Movement {
    /// No movement.
    No,
    /// In DAS movement.
    /// contains the timer of DAS and the direction of movement.
    InDas(u8, MoveDirection),
    /// In ARR movement.
    /// contains the timer of each ARR.
    InArr(u8, MoveDirection),
}

impl Movement {
    /// Increases the timer rate of the movement.
    fn increase(&mut self) {
        match self {
            Movement::InDas(f, _) => {
                *f += 1;
            }
            Movement::InArr(f, _) => {
                *f += 1;
            }
            _ => {}
        }
    }
}
