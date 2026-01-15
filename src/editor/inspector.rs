use bevy::prelude::*;
use super::resources::EditorState;
use super::styles::*;

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
    let component_names: Vec<String> = if let Ok(iter) = world.inspect_entity(entity) {
        iter.map(|c| format!("{:?}", c.name())).collect()
    } else {
        Vec::new()
    };

    world.entity_mut(panel).with_children(|p| {
        p.spawn((
             Text::new(format!("Inspector: {:?}", entity)),
             TextFont::default(),
             TextColor(HEADER_COLOR),
             Node {
                 margin: UiRect::bottom(Val::Px(10.0)),
                 ..default()
             }
        ));

        for name in component_names {
            p.spawn((
                Text::new(name),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::vertical(Val::Px(2.0)),
                    ..default()
                }
            ));
        }
    });

}
