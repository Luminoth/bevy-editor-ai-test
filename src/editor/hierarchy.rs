use bevy::prelude::*;
use super::styles::*;
use super::components::*;
use super::resources::EditorState;

pub fn update_hierarchy(
    mut current_state: ResMut<EditorState>,
    interactions: Query<(&Interaction, &HierarchyEntityRow), Changed<Interaction>>,
) {
    for (interaction, row) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            current_state.selected_entity = Some(row.entity);
        }
    }
}
type AddEntityInteractionQuery<'w, 's> = Query<'w, 's,
    (&'static Interaction, &'static AddEntityButton),
    (Changed<Interaction>, With<AddEntityButton>),
>;

pub fn handle_hierarchy_actions(
    interaction_query: AddEntityInteractionQuery,
    mut commands: Commands,
) {
    for (interaction, _) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            commands.spawn((
                Name::new("New Entity"),
                Transform::default(),
                Visibility::default(),
            ));
        }
    }
}

#[derive(Resource, Default)]
pub struct HierarchyRefreshTimer(pub Timer);

pub type RootEntityFilter = (With<Transform>, Without<ChildOf>, Without<Node>);

pub fn update_hierarchy_list(
    mut commands: Commands,
    root_query: Query<Entity, RootEntityFilter>,
    panel_query: Query<Entity, With<HierarchyPanel>>,
    children_query: Query<&Children>,
    time: Res<Time>,
    mut timer: Local<HierarchyRefreshTimer>,
    asset_server: Res<AssetServer>,
) {
    if timer.0.duration().as_secs_f32() == 0.0 {
        timer.0 = Timer::from_seconds(1.0, TimerMode::Repeating);
    }
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    if let Some(panel) = panel_query.iter().next() {
        // Manual despawn using standard despawn()
        if let Ok(children) = children_query.get(panel) {
            for &child in children {
                commands.entity(child).despawn();
            }
        }

        commands.entity(panel).with_children(|p| {
              p.spawn((
                    Text::new("Hierarchy"),
                    TextFont::default(),
                    TextColor(super::styles::HEADER_COLOR),
                ));

                // Add Entity Button
                p.spawn((
                    Button,
                    Node {
                        margin: UiRect::left(Val::Px(10.0)),
                        padding: UiRect::all(Val::Px(4.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    AddEntityButton,
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("+"),
                        TextFont {
                            font_size: 16.0,
                             ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });

              for entity in root_query.iter() {
                  p.spawn((
                      Button,
                      Node {
                          width: Val::Percent(100.0),
                          height: Val::Px(24.0),
                          margin: UiRect::all(Val::Px(1.0)),
                          justify_content: JustifyContent::Start,
                          align_items: AlignItems::Center,
                          padding: UiRect::left(Val::Px(5.0)),
                          ..default()
                      },
                      BackgroundColor(BUTTON_COLOR_NORMAL),
                      HierarchyEntityRow { entity },
                  )).with_children(|btn| {
                       btn.spawn((
                           Text::new(format!("Entity {:?}", entity)),
                           text_style(&asset_server),
                           TextColor(TEXT_COLOR),
                       ));
                  });
              }
        });
    }
}
