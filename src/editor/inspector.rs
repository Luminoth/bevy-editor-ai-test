use bevy::prelude::*;
use super::resources::{EditorState, InspectorUiState};
use super::styles::*;
use super::components::*;
use bevy::ecs::reflect::ReflectComponent;

pub fn inspector_ui_system(
    world: &mut World,
) {
    let selected = world.resource::<EditorState>().selected_entity;

    let mut inspector_panel = Option::<Entity>::None;
    {
        let mut query = world.query_filtered::<Entity, With<super::components::InspectorPanel>>();
        if let Some(e) = query.iter(world).next() {
            inspector_panel = Some(e);
        }
    }

    let Some(panel) = inspector_panel else { return; };

    // Despawn children manually
    let children = world.get::<Children>(panel).map(|c| c.to_vec());
    if let Some(children) = children {
        for child in children {
            // Despawn recursive on World
            world.despawn(child);
        }
    }

    let Some(entity) = selected else { return; };

    // Get component names as Strings to release borrow on World
    let component_names: Vec<(String, String)> = if let Ok(iter) = world.inspect_entity(entity) {
        iter.map(|c| (format!("{:?}", c.name()), c.name().to_string())).collect()
    } else {
        Vec::new()
    };


    // Get state for UI
    let (is_adding, filter) = {
        let state = world.resource::<InspectorUiState>();
        (state.is_adding_component, state.component_filter.clone())
    };

    // Prepare components to add (search results)
    let mut matching_components = Vec::new();
    if is_adding {
        let type_registry = world.resource::<AppTypeRegistry>().read();
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
        // Header with Delete Button
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

        for (display_name, type_name) in component_names {
            p.spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                margin: UiRect::vertical(Val::Px(2.0)),
                ..default()
            }).with_children(|row| {
                 row.spawn((
                    Text::new(&display_name),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                ));

                row.spawn((
                    Button,
                     Node {
                        padding: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    RemoveComponentButton(type_name),
                )).with_children(|btn| {
                     btn.spawn((
                        Text::new("X"),
                        TextFont { font_size: 10.0, ..default() },
                        TextColor(TEXT_COLOR),
                     ));
                });
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
                 // Search Input Display
                 section.spawn((
                     Text::new(format!("Search: {}_", filter)),
                     TextFont { font_size: 14.0, ..default() },
                     TextColor(TEXT_COLOR),
                     ComponentSearchInput, // Marker if needed
                 ));

                 section.spawn((
                     Button,
                     Node {
                         margin: UiRect::top(Val::Px(4.0)),
                         padding: UiRect::all(Val::Px(4.0)),
                         ..default()
                     },
                     BackgroundColor(BUTTON_COLOR_NORMAL),
                     AddComponentButton, // Clicking again toggles off
                 )).with_children(|btn| {
                     btn.spawn((
                         Text::new("Cancel"),
                         TextFont { font_size: 12.0, ..default() },
                         TextColor(TEXT_COLOR),
                     ));
                 });

                 // List results
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
