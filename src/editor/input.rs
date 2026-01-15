use bevy::prelude::*;
use super::resources::InspectorUiState;

pub fn text_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<InspectorUiState>,
) {
    if !ui_state.is_adding_component {
        return;
    }

    if keys.just_pressed(KeyCode::Backspace) {
        ui_state.component_filter.pop();
    }

    // Mapping keys to characters manually for now (simplified)
    let key_map = [
        (KeyCode::KeyA, 'a'), (KeyCode::KeyB, 'b'), (KeyCode::KeyC, 'c'), (KeyCode::KeyD, 'd'),
        (KeyCode::KeyE, 'e'), (KeyCode::KeyF, 'f'), (KeyCode::KeyG, 'g'), (KeyCode::KeyH, 'h'),
        (KeyCode::KeyI, 'i'), (KeyCode::KeyJ, 'j'), (KeyCode::KeyK, 'k'), (KeyCode::KeyL, 'l'),
        (KeyCode::KeyM, 'm'), (KeyCode::KeyN, 'n'), (KeyCode::KeyO, 'o'), (KeyCode::KeyP, 'p'),
        (KeyCode::KeyQ, 'q'), (KeyCode::KeyR, 'r'), (KeyCode::KeyS, 's'), (KeyCode::KeyT, 't'),
        (KeyCode::KeyU, 'u'), (KeyCode::KeyV, 'v'), (KeyCode::KeyW, 'w'), (KeyCode::KeyX, 'x'),
        (KeyCode::KeyY, 'y'), (KeyCode::KeyZ, 'z'),
    ];

    for (code, char) in key_map {
        if keys.just_pressed(code) {
             ui_state.component_filter.push(char);
        }
    }
}
