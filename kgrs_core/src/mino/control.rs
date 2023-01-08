use super::*;
use kgrs_util::function::fixed_update;

pub(crate) struct MinoControlPlugin;

impl Plugin for MinoControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(place_mino)
            .add_system_set(fixed_update(move_mino));
    }
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
    mut board_query: Query<&mut Board>,
    mut mino_mesh_query: Query<(&mut MinoInfo, &mut Transform)>,
) {
    for key in input.get_pressed() {
        if key == &KeyCode::Left || key == &KeyCode::Right {
            let cell_size = board_query.single().width / 10.;
            board_query.single_mut().data.move_mino(
                if let KeyCode::Left = key {
                    MoveDirection::Left
                } else {
                    // Never reach with other than `KeyCode::Right`
                    MoveDirection::Right
                },
                &mut mino_mesh_query,
                cell_size,
            );
        }
    }
}
