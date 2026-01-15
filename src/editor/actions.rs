use bevy::prelude::*;
use super::resources::{EditorState, InspectorUiState};
use super::components::*;
use bevy::ecs::system::Command;

type DeleteEntityFilter = (Changed<Interaction>, With<DeleteEntityButton>);
type AddComponentToggleFilter = (Changed<Interaction>, With<AddComponentButton>);
type RemoveComponentFilter = (Changed<Interaction>, With<RemoveComponentButton>);
type ComponentAddConfirmFilter = (Changed<Interaction>, With<ComponentAddButton>);

pub fn handle_delete_entity(
    interaction_query: Query<(&Interaction, &DeleteEntityButton), DeleteEntityFilter>,
    mut commands: Commands,
    current_state: Res<EditorState>,
) {
    for (interaction, _) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && let Some(entity) = current_state.selected_entity {
            commands.entity(entity).despawn();
            // Deselect handled by selection logic or we can clear it
        }
    }
}

pub fn handle_add_component_toggle(
    interaction_query: Query<(&Interaction, &AddComponentButton), AddComponentToggleFilter>,
    mut ui_state: ResMut<InspectorUiState>,
) {
    for (interaction, _) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            ui_state.is_adding_component = !ui_state.is_adding_component;
            if ui_state.is_adding_component {
                ui_state.component_filter.clear(); // Reset filter on open
            }
        }
    }
}

pub fn handle_remove_component(
    interaction_query: Query<(&Interaction, &RemoveComponentButton), RemoveComponentFilter>,
    mut commands: Commands,
    current_state: Res<EditorState>,
    type_registry: Res<AppTypeRegistry>,
) {
    for (interaction, btn) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && let Some(entity) = current_state.selected_entity {
            let type_registry = type_registry.read();

            for registration in type_registry.iter() {
                let name = registration.type_info().type_path();
                if name == btn.0 {
                     if registration.data::<ReflectComponent>().is_some() {
                          commands.queue(RemoveComponentCommand {
                              entity,
                              type_id: registration.type_id(),
                              component_name: name.to_string(),
                          });
                     }
                     break;
                }
            }
        }
    }
}


struct RemoveComponentCommand {
    entity: Entity,
    type_id: std::any::TypeId,
    component_name: String,
}

impl Command for RemoveComponentCommand {
    fn apply(self, world: &mut World) {
         let type_registry = world.resource::<AppTypeRegistry>().clone();
         let type_registry = type_registry.read();
         if let Some(registration) = type_registry.get(self.type_id)
            && let Some(reflect_component) = registration.data::<ReflectComponent>()
         {
             reflect_component.remove(&mut world.entity_mut(self.entity));
             info!("Removed component: {}", self.component_name);
         }
    }
}


pub fn handle_add_component_confirm(
    interaction_query: Query<(&Interaction, &ComponentAddButton), ComponentAddConfirmFilter>,
    mut commands: Commands,
    current_state: Res<EditorState>,
    type_registry: Res<AppTypeRegistry>,
    mut ui_state: ResMut<InspectorUiState>,
) {
    for (interaction, btn) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && let Some(entity) = current_state.selected_entity {
              let type_registry = type_registry.read();
              for registration in type_registry.iter() {
                  if registration.type_info().type_path_table().short_path() == btn.0 {
                      commands.queue(AddComponentCommand {
                          entity,
                          type_id: registration.type_id(),
                      });
                      ui_state.is_adding_component = false;
                      break;
                  }
              }
        }
    }
}

struct AddComponentCommand {
    entity: Entity,
    type_id: std::any::TypeId,
}

impl Command for AddComponentCommand {
    fn apply(self, world: &mut World) {
         let type_registry = world.resource::<AppTypeRegistry>().clone();
         let type_registry = type_registry.read();
         if let Some(registration) = type_registry.get(self.type_id)
             && let Some(reflect_default) = registration.data::<ReflectDefault>()
             && let Some(reflect_component) = registration.data::<ReflectComponent>()
         {
             let default_val = reflect_default.default();
             reflect_component.insert(&mut world.entity_mut(self.entity), default_val.as_ref(), &type_registry);
             info!("Added component: {:?}", registration.type_info().type_path());
         }
    }
}
