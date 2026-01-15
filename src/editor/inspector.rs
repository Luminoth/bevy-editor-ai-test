use bevy::prelude::*;
use super::resources::{EditorState, InspectorUiState};
use super::styles::*;
use super::components::*;
use bevy::ecs::reflect::ReflectComponent;
use bevy::reflect::ReflectRef;

type InspectorQueryState<'w, 's> = bevy::ecs::query::QueryState<(Entity, &'static mut PropertyInput, &'static Children)>;

pub fn inspector_ui_system(
    world: &mut World,
    mut last_state: Local<(Option<Entity>, Option<bevy::ecs::archetype::ArchetypeId>)>,
    mut query_state: Local<Option<InspectorQueryState<'static, 'static>>>,
    mut text_query_state: Local<Option<bevy::ecs::query::QueryState<&'static mut Text>>>,
) {
    let selected = world.resource::<EditorState>().selected_entity;
    let focused = world.resource::<EditorState>().focused_input;

    // Initialize query states
    if query_state.is_none() {
        *query_state = Some(world.query::<(Entity, &mut PropertyInput, &Children)>());
    }
    if text_query_state.is_none() {
        *text_query_state = Some(world.query::<&mut Text>());
    }

    let Some(entity) = selected else {
        // If nothing selected, clear panel and state
        if last_state.0.is_some() {
             clear_inspector(world);
             *last_state = (None, None);
        }
        return;
    };

    let current_archetype = world.get_entity(entity).ok().map(|e| e.archetype().id());

    // Check if we need to rebuild
    let needs_rebuild = last_state.0 != some(entity) || last_state.1 != current_archetype;

    // Helper to clear
    fn clear_inspector(world: &mut World) {
         let mut component_panel = Option::<Entity>::None;
        {
            let mut query = world.query_filtered::<Entity, With<super::components::InspectorPanel>>();
            if let Some(e) = query.iter(world).next() {
                component_panel = Some(e);
            }
        }
        if let Some(panel) = component_panel {
             let children = world.get::<Children>(panel).map(|c| c.to_vec());
            if let Some(children) = children {
                for child in children {
                    world.despawn(child);
                }
            }
        }
    }

    // Helper to some
    fn some<T>(t: T) -> Option<T> { Some(t) }

    if needs_rebuild {
        clear_inspector(world);
        rebuild_inspector(world, entity);
        *last_state = (Some(entity), current_archetype);
    } else {
        // Update values
         update_values(world, entity, focused, query_state.as_mut().unwrap(), text_query_state.as_mut().unwrap());
    }
}

fn rebuild_inspector(world: &mut World, entity: Entity) {
    let mut inspector_panel = Option::<Entity>::None;
    {
        let mut query = world.query_filtered::<Entity, With<super::components::InspectorPanel>>();
        if let Some(e) = query.iter(world).next() {
            inspector_panel = Some(e);
        }
    }
    let Some(panel) = inspector_panel else { return; };

    // Collect component data
    struct FieldInfo {
        name: String,
        value: String,
    }
    struct ComponentInfo {
        name: String,
        type_name: String,
        type_id: std::any::TypeId,
        fields: Vec<FieldInfo>,
    }

    let mut components_to_show: Vec<ComponentInfo> = Vec::new();

    // Use robust component iteration via TypeRegistry and Archetype
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let type_registry = type_registry.read();

    // We need to access world components, but we have immutable world ref inside get_entity if we aren't careful?
    // world.get_entity returns EntityRef which borrows world. safely.
     if let Ok(entity_ref) = world.get_entity(entity) {
          for component_id in entity_ref.archetype().components() {
              if let Some(info) = world.components().get_info(*component_id)
                  && let Some(type_id) = info.type_id()
                  && let Some(registration) = type_registry.get(type_id)
                  && let Some(reflect_component) = registration.data::<ReflectComponent>()
                  && let Some(component) = reflect_component.reflect(entity_ref)
              {
                    // Process component
                    let type_info = component.get_represented_type_info().unwrap_or(registration.type_info());
                    let name = type_info.type_path_table().short_path().to_string();
                    let type_name = type_info.type_path().to_string();
                    let comp_type_id = type_info.type_id();

                    let mut fields = Vec::new();
                    if let ReflectRef::Struct(s) = component.reflect_ref() {
                        for i in 0..s.field_len() {
                            let field_name = s.name_at(i).unwrap().to_string();
                            let field_value = s.field_at(i).unwrap();
                            let value_str = format!("{:?}", field_value);
                             fields.push(FieldInfo {
                                name: field_name,
                                value: value_str,
                            });
                        }
                    }

                    components_to_show.push(ComponentInfo {
                        name,
                        type_name,
                        type_id: comp_type_id,
                        fields,
                    });
              }
          }
     }

    // IsAdding state
     let (is_adding, filter) = {
        let state = world.resource::<InspectorUiState>();
        (state.is_adding_component, state.component_filter.clone())
    };

    let mut matching_components = Vec::new();
    if is_adding {
        for registration in type_registry.iter() {
            if let Some(_reflect_component) = registration.data::<ReflectComponent>()
                 && registration.data::<ReflectDefault>().is_some()
            {
                 let name = registration.type_info().type_path_table().short_path();
                 if name.to_lowercase().contains(&filter.to_lowercase()) {
                    matching_components.push((name.to_string(), registration.type_id()));
                 }
            }
        }
    }

    world.entity_mut(panel).with_children(|p| {
          // Header
        p.spawn(Node {
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            ..default()
        }).with_children(|header| {
             header.spawn((
                 Text::new(format!("Inspector: {:?}", entity)),
                 TextFont::default(),
                 TextColor(HEADER_COLOR),
            ));
            header.spawn((
                Button,
                Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    margin: UiRect::right(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                DeleteEntityButton,
            )).with_children(|btn| {
                 btn.spawn((
                    Text::new("Delete"),
                    TextFont { font_size: 12.0, ..default() },
                    TextColor(TEXT_COLOR),
                 ));
            });
        });

        for info in components_to_show {
            p.spawn(Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                margin: UiRect::vertical(Val::Px(4.0)),
                padding: UiRect::all(Val::Px(2.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            })
            .with_children(|comp_section| {
                 comp_section.spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Percent(100.0),
                    ..default()
                 }).with_children(|header| {
                     header.spawn((
                        Text::new(&info.name),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));

                    header.spawn((
                        Button,
                         Node {
                            padding: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                        RemoveComponentButton(info.type_name.clone()),
                    )).with_children(|btn| {
                         btn.spawn((
                            Text::new("X"),
                            TextFont { font_size: 10.0, ..default() },
                            TextColor(TEXT_COLOR),
                         ));
                    });
                 });

                 for field in info.fields {
                     comp_section.spawn(Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(2.0)),
                        ..default()
                     }).with_children(|row| {
                         row.spawn((
                             Text::new(format!("{}: ", field.name)),
                             TextFont { font_size: 12.0, ..default() },
                             TextColor(TEXT_COLOR),
                         ));

                         row.spawn((
                             Button,
                             Node {
                                 min_width: Val::Px(50.0),
                                 padding: UiRect::all(Val::Px(2.0)),
                                 border: UiRect::all(Val::Px(1.0)),
                                 ..default()
                             },
                             PropertyInput {
                                 entity,
                                 component_type_id: info.type_id,
                                 field_name: field.name.clone(),
                                 current_value: field.value.clone(),
                             }
                         )).with_children(|input_container| {
                             input_container.spawn((
                                 Text::new(field.value.clone()),
                                 TextFont { font_size: 12.0, ..default() },
                                 TextColor(TEXT_COLOR),
                             ));
                         });
                     });
                 }
            });
        }

         // Add Component Section
        p.spawn((
            Node {
                margin: UiRect::top(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        )).with_children(|section| {
             if !is_adding {
                 section.spawn((
                    Button,
                     Node {
                        padding: UiRect::all(Val::Px(4.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    AddComponentButton,
                )).with_children(|btn| {
                     btn.spawn((
                        Text::new("Add Component"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(TEXT_COLOR),
                     ));
                });
             } else {
                 section.spawn((
                     Text::new(format!("Search: {}_", filter)),
                     TextFont { font_size: 14.0, ..default() },
                     TextColor(TEXT_COLOR),
                     ComponentSearchInput,
                 ));
                 section.spawn((
                     Button,
                     Node {
                         margin: UiRect::top(Val::Px(4.0)),
                         padding: UiRect::all(Val::Px(4.0)),
                         ..default()
                     },
                     BackgroundColor(BUTTON_COLOR_NORMAL),
                     AddComponentButton,
                 )).with_children(|btn| {
                     btn.spawn((
                         Text::new("Cancel"),
                         TextFont { font_size: 12.0, ..default() },
                         TextColor(TEXT_COLOR),
                     ));
                 });

                 for (name, _type_id) in matching_components.iter().take(10) {
                     section.spawn((
                         Button,
                         Node {
                             margin: UiRect::top(Val::Px(2.0)),
                             padding: UiRect::all(Val::Px(2.0)),
                             width: Val::Percent(100.0),
                             ..default()
                         },
                         BackgroundColor(BUTTON_COLOR_NORMAL),
                         ComponentAddButton(name.clone()),
                     )).with_children(|btn| {
                         btn.spawn((
                             Text::new(name),
                             TextFont { font_size: 12.0, ..default() },
                             TextColor(TEXT_COLOR),
                         ));
                     });
                 }
             }
        });
    });
}

fn update_values(
    world: &mut World,
    entity: Entity,
    focused_input: Option<Entity>,
    input_query: &mut InspectorQueryState<'_, '_>,
    text_query: &mut bevy::ecs::query::QueryState<&mut Text>,
) {
     // Gather current values from world
     // We can't iterate PropertyInput and access world.inspect_entity at the same time if we are not careful.
     // QueryState allows iter(world).

     // 1. Collect needed updates
     let mut updates = Vec::new();
     for (input_entity, prop_input, children) in input_query.iter(world) {
         if Some(input_entity) == focused_input {
             continue; // Don't fight user input
         }

         // We need to fetch the value from the component
         // We need to fetch the value from the component

         // To access component reflectively we need world access.
         // This is tricky inside query iteration loop if we need mutable access later,
         // but here we just need read access to world for reflection, which is blocked by query iteration borrowing world.
         // Solution: Collect identify info, then fetch values, then apply.
         updates.push((input_entity, prop_input.component_type_id, prop_input.field_name.clone(), children[0]));
     }

     let type_registry_arc = world.resource::<AppTypeRegistry>().clone();
     let type_registry = type_registry_arc.read();

     for (input_entity, type_id, field_name, text_child) in updates {
          let mut new_value_str = None;

           if let Some(registration) = type_registry.get(type_id)
              && let Some(reflect_component) = registration.data::<ReflectComponent>()
              && let Some(component) = reflect_component.reflect(world.entity(entity))
              && let ReflectRef::Struct(s) = component.reflect_ref()
              && let Some(field) = s.field(&field_name)
           {
               new_value_str = Some(format!("{:?}", field));
           }

          if let Some(val) = new_value_str {
               // Update PropertyInput component state
               if let Ok((_, mut prop_input, _)) = input_query.get_mut(world, input_entity)
                   && prop_input.current_value != val
               {
                   prop_input.current_value = val.clone();

                   // Update Text
                   if let Ok(mut text) = text_query.get_mut(world, text_child) {
                       **text = val;
                   }
               }
          }
     }
}
