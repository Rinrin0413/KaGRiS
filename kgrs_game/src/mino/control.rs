use bevy::time::FixedTimestep;
use super::*;

pub(crate) struct MinoControlPlugin;

impl Plugin for MinoControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(place_mino)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1./60.))
                    .with_system(move_mino),
            );
    }
}

fn place_mino(mut mino_ctrl_query: Query<&mut MinoCtrl>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        mino_ctrl_query.single_mut().is_waiting = true;
    }
}

fn move_mino(input: Res<Input<KeyCode>>) {
    for key in input.get_pressed() {
        match key {
            KeyCode::Left => {
                println!("Right");
            }
            KeyCode::Right => {
                println!("Left");
            }
            _ => {}
        }
    }
}
