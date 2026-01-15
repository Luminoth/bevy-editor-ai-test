use bevy::prelude::*;
use super::resources::{EditorState, InspectorUiState};
use super::components::*;
use bevy::ecs::system::Command;
use crate::editor::menu::SceneInfo;

type DeleteEntityFilter = (Changed<Interaction>, With<DeleteEntityButton>);
type AddComponentToggleFilter = (Changed<Interaction>, With<AddComponentButton>);
type RemoveComponentFilter = (Changed<Interaction>, With<RemoveComponentButton>);
type ComponentAddConfirmFilter = (Changed<Interaction>, With<ComponentAddButton>);

pub fn handle_delete_entity(
    interaction_query: Query<(&Interaction, &DeleteEntityButton), DeleteEntityFilter>,
    mut commands: Commands,
    current_state: ResMut<EditorState>,
    mut scene_info: ResMut<SceneInfo>,
) {
    for (interaction, _) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && let Some(entity) = current_state.selected_entity {
            commands.entity(entity).despawn();
            scene_info.is_dirty = true;
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
             if let Some(mut info) = world.get_resource_mut::<SceneInfo>() {
                 info.is_dirty = true;
             }
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
             if let Some(mut info) = world.get_resource_mut::<SceneInfo>() {
                 info.is_dirty = true;
             }
         }
    }
}

pub struct PropertyChangeCommand {
    pub entity: Entity,
    pub component_type_id: std::any::TypeId,
    pub field_name: String,
    pub new_value: String,
}

impl Command for PropertyChangeCommand {
    fn apply(self, world: &mut World) {
        use bevy::reflect::ReflectMut;
        let type_registry = world.resource::<AppTypeRegistry>().clone();
        let type_registry = type_registry.read();

        if let Some(registration) = type_registry.get(self.component_type_id)
            && let Some(reflect_component) = registration.data::<ReflectComponent>()
        {
             // We need to get the component mutably.
             // reflect_component.reflect_mut does that but returns ReflectMut.
             // We need to work with it.

             let mut applied = false;
             if let Some(mut component_reflect) = reflect_component.reflect_mut(world.entity_mut(self.entity))
                 && let ReflectMut::Struct(s) = component_reflect.reflect_mut()
                 && let Some(field) = s.field_mut(&self.field_name)
             {
                 applied = try_apply_value(field, &self.new_value);
             }

             if applied {
                 if let Some(mut info) = world.get_resource_mut::<SceneInfo>() {
                     info.is_dirty = true;
                 }
             }
        }
    }
}

// Helper to attempt to parse string into the field
fn try_apply_value(field: &mut dyn bevy::reflect::PartialReflect, value: &str) -> bool {
    // Try some common types
    if let Some(v) = field.try_downcast_mut::<f32>() {
        if let Ok(parsed) = value.parse::<f32>() {
            *v = parsed;
            return true;
        }
    } else if let Some(v) = field.try_downcast_mut::<f64>() {
        if let Ok(parsed) = value.parse::<f64>() {
            *v = parsed;
            return true;
        }
    } else if let Some(v) = field.try_downcast_mut::<String>() {
        *v = value.to_string();
        return true;
    } else if let Some(v) = field.try_downcast_mut::<bool>() {
        if let Ok(parsed) = value.parse::<bool>() {
            *v = parsed;
            return true;
        }
    } else if let Some(v) = field.try_downcast_mut::<usize>() {
        if let Ok(parsed) = value.parse::<usize>() {
            *v = parsed;
            return true;
        }
    } else if let Some(v) = field.try_downcast_mut::<i32>()
        && let Ok(parsed) = value.parse::<i32>()
    {
        *v = parsed;
        return true;
    }
    false
}
