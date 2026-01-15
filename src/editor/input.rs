use bevy::prelude::*;
use super::resources::InspectorUiState;

type PropertyInputQuery<'w, 's> = Query<'w, 's, (Entity, &'static Interaction), (Changed<Interaction>, With<super::components::PropertyInput>)>;

pub fn text_input_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<InspectorUiState>,
    mut editor_state: ResMut<super::resources::EditorState>,
    query: PropertyInputQuery,
    mut property_inputs: Query<(&mut super::components::PropertyInput, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    // 1. Handle Focus Selection
    for (entity, interaction) in query.iter() {
        if *interaction == Interaction::Pressed {
            editor_state.focused_input = Some(entity);
            // Close component search if we are editing properties
            ui_state.is_adding_component = false;
        }
    }

    // Helper to map keys to chars (Fallback implementation)
    let mut pushed_chars = Vec::new();
    // A-Z
    if keys.just_pressed(KeyCode::KeyA) { pushed_chars.push('a'); }
    if keys.just_pressed(KeyCode::KeyB) { pushed_chars.push('b'); }
    if keys.just_pressed(KeyCode::KeyC) { pushed_chars.push('c'); }
    if keys.just_pressed(KeyCode::KeyD) { pushed_chars.push('d'); }
    if keys.just_pressed(KeyCode::KeyE) { pushed_chars.push('e'); }
    if keys.just_pressed(KeyCode::KeyF) { pushed_chars.push('f'); }
    if keys.just_pressed(KeyCode::KeyG) { pushed_chars.push('g'); }
    if keys.just_pressed(KeyCode::KeyH) { pushed_chars.push('h'); }
    if keys.just_pressed(KeyCode::KeyI) { pushed_chars.push('i'); }
    if keys.just_pressed(KeyCode::KeyJ) { pushed_chars.push('j'); }
    if keys.just_pressed(KeyCode::KeyK) { pushed_chars.push('k'); }
    if keys.just_pressed(KeyCode::KeyL) { pushed_chars.push('l'); }
    if keys.just_pressed(KeyCode::KeyM) { pushed_chars.push('m'); }
    if keys.just_pressed(KeyCode::KeyN) { pushed_chars.push('n'); }
    if keys.just_pressed(KeyCode::KeyO) { pushed_chars.push('o'); }
    if keys.just_pressed(KeyCode::KeyP) { pushed_chars.push('p'); }
    if keys.just_pressed(KeyCode::KeyQ) { pushed_chars.push('q'); }
    if keys.just_pressed(KeyCode::KeyR) { pushed_chars.push('r'); }
    if keys.just_pressed(KeyCode::KeyS) { pushed_chars.push('s'); }
    if keys.just_pressed(KeyCode::KeyT) { pushed_chars.push('t'); }
    if keys.just_pressed(KeyCode::KeyU) { pushed_chars.push('u'); }
    if keys.just_pressed(KeyCode::KeyV) { pushed_chars.push('v'); }
    if keys.just_pressed(KeyCode::KeyW) { pushed_chars.push('w'); }
    if keys.just_pressed(KeyCode::KeyX) { pushed_chars.push('x'); }
    if keys.just_pressed(KeyCode::KeyY) { pushed_chars.push('y'); }
    if keys.just_pressed(KeyCode::KeyZ) { pushed_chars.push('z'); }
    // 0-9
    if keys.just_pressed(KeyCode::Digit0) { pushed_chars.push('0'); }
    if keys.just_pressed(KeyCode::Digit1) { pushed_chars.push('1'); }
    if keys.just_pressed(KeyCode::Digit2) { pushed_chars.push('2'); }
    if keys.just_pressed(KeyCode::Digit3) { pushed_chars.push('3'); }
    if keys.just_pressed(KeyCode::Digit4) { pushed_chars.push('4'); }
    if keys.just_pressed(KeyCode::Digit5) { pushed_chars.push('5'); }
    if keys.just_pressed(KeyCode::Digit6) { pushed_chars.push('6'); }
    if keys.just_pressed(KeyCode::Digit7) { pushed_chars.push('7'); }
    if keys.just_pressed(KeyCode::Digit8) { pushed_chars.push('8'); }
    if keys.just_pressed(KeyCode::Digit9) { pushed_chars.push('9'); }
    // Misc
    if keys.just_pressed(KeyCode::Space) { pushed_chars.push(' '); }
    if keys.just_pressed(KeyCode::Period) { pushed_chars.push('.'); }
    if keys.just_pressed(KeyCode::Minus) { pushed_chars.push('-'); }

    // 2. Handle Input for Focused Property
    if let Some(focused) = editor_state.focused_input {
        if let Ok((mut prop_input, children)) = property_inputs.get_mut(focused) {
             let mut changed = false;

             for char in &pushed_chars {
                 prop_input.current_value.push(*char);
                 changed = true;
             }

             if keys.just_pressed(KeyCode::Backspace) {
                 prop_input.current_value.pop();
                 changed = true;
             }

             if keys.just_pressed(KeyCode::Enter) {
                  commands.queue(super::actions::PropertyChangeCommand {
                      entity: prop_input.entity,
                      component_type_id: prop_input.component_type_id,
                      field_name: prop_input.field_name.clone(),
                      new_value: prop_input.current_value.clone(),
                  });
             }

             if changed
                 && let Some(text_entity) = children.first()
                 && let Ok(mut text) = text_query.get_mut(*text_entity) {
                     **text = prop_input.current_value.clone();
             }
             return;
        } else {
             // Focused entity probably despawned or error
             editor_state.focused_input = None;
        }
    }

    // 3. Handle Component Search Input (Fallback)
    if !ui_state.is_adding_component {
        return;
    }

    if keys.just_pressed(KeyCode::Backspace) {
        ui_state.component_filter.pop();
    }

    for char in &pushed_chars {
         ui_state.component_filter.push(*char);
    }
}
