use crate::entities::{Player, PlayerStat};
use bevy::prelude::*;

use crate::screens::{upgrade::ui::UpgradeScreen, GameMode};

pub fn handle_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_mode: ResMut<GameMode>,
    mut screen_q: Query<&mut UpgradeScreen>,
    mut player_q: Query<&mut Player>,
) {
    let mut screen = screen_q.single_mut();
    let mut player = player_q.single_mut();

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::M => {
                *game_mode = GameMode::Playing;
            },
            KeyCode::Left => {
                if screen.selection_index > 0 {
                    screen.selection_index -= 1;
                }
            },
            KeyCode::Right => {
                if screen.selection_index < 5 {
                    screen.selection_index += 1;
                }
            },
            KeyCode::Space => {
                let selected_stat = PlayerStat::from(screen.selection_index);
                let cost = player.cost_by(selected_stat);

                if player.xp >= cost {
                    let value_before = player.stats.value(selected_stat);
                    let limit_before = player.stats.limit(selected_stat);
                    player.xp -= cost;
                    player.upgrade(selected_stat);
                    println!(
                        "Upgrading {selected_stat}:
                      - value: {value_before} -> {},
                      - limit: {limit_before} -> {},
                      - cost: {cost} -> {}),
                      - max: {}",
                        player.value_by(selected_stat),
                        player.limit_by(selected_stat),
                        player.cost_by(selected_stat),
                        player.max_by(selected_stat)
                    );
                }
            },
            _ => (),
        }
    }

    keyboard_input.reset(KeyCode::M);
}
