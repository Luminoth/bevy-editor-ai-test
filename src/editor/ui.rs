use bevy::prelude::*;
use super::components::*;
use super::styles::*;
use crate::editor::resources::EditorConfig;
use bevy::input::mouse::AccumulatedMouseMotion;
use crate::editor::log::{LogPanel, LogPanelContent};


pub fn setup_editor_ui(mut commands: Commands) {
    // 1. Spawn Inspector Panel (Detached initially to get ID)
    let inspector = commands.spawn((
        Node {
            width: Val::Px(250.0),
            min_width: Val::Px(250.0),
            flex_shrink: 0.0,
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(1.0)),
            padding: UiRect::all(Val::Px(4.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
          BackgroundColor(PANEL_COLOR),
          InspectorPanel,
          ResizablePanel::default(),
          GlobalTransform::default(),
          Transform::default(),
          Visibility::default(),
          InheritedVisibility::default(),
          ViewVisibility::default(),
    )).with_children(|p| {
          p.spawn((
             Text::new("Inspector"),
          ));
    }).id();

    // 2. Spawn Log Panel (Detached)
    let log_panel = commands.spawn((
        log_panel_style(),
        BackgroundColor(PANEL_COLOR),
        ResizablePanel {
            min_height: 50.0,
            max_height: 500.0,
            ..default()
        },
        LogPanel,
        GlobalTransform::default(),
        Transform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    )).with_children(|p| {
          p.spawn((
              Text::new("Output"),
              TextFont::default(),
              TextColor(HEADER_COLOR),
              Node {
                  margin: UiRect::bottom(Val::Px(4.0)),
                  ..default()
              }
          ));
          p.spawn((
              Node {
                  flex_direction: FlexDirection::Column,
                  width: Val::Percent(100.0),
                  overflow: Overflow::clip(),
                  flex_grow: 1.0,
                  ..default()
              },
              LogPanelContent
          ));
    }).id();

    // 3. Spawn Center Column (Detached)
    let center_col = commands.spawn((
        Node {
             flex_direction: FlexDirection::Column,
             flex_grow: 1.0,
             height: Val::Percent(100.0),
             min_width: Val::Px(0.0),
             ..default()
        },
        GlobalTransform::default(),
        Transform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    )).id();


    commands.entity(center_col).with_children(|center| {
         // Viewport
         center.spawn((
             viewport_style(),
             ViewportPanel,
             GlobalTransform::default(),
             Transform::default(),
             Visibility::default(),
             InheritedVisibility::default(),
             ViewVisibility::default(),
         ));
         // Resize Handle Up
         center.spawn((
             resize_handle_horizontal_style(),
             BackgroundColor(RESIZE_HANDLE_COLOR),
             ResizeHandle {
                 direction: ResizeDirection::Up,
                 target_panel: log_panel,
             },
             Interaction::default(),
             GlobalTransform::default(),
             Transform::default(),
             Visibility::default(),
             InheritedVisibility::default(),
             ViewVisibility::default(),
         ));
    });
    commands.entity(center_col).add_child(log_panel);

    // 4. Spawn Hierarchy (Detached)
    let hierarchy = commands.spawn((
         Node {
             width: Val::Px(250.0),
             min_width: Val::Px(250.0),
             flex_shrink: 0.0,
             height: Val::Percent(100.0),
             border: UiRect::all(Val::Px(1.0)),
             padding: UiRect::all(Val::Px(4.0)),
             flex_direction: FlexDirection::Column,
             ..default()
         },
         BackgroundColor(PANEL_COLOR),
         HierarchyPanel,
         ResizablePanel::default(),
         GlobalTransform::default(),
         Transform::default(),
         Visibility::default(),
         InheritedVisibility::default(),
         ViewVisibility::default(),
    )).with_children(|p| {
         p.spawn((
             Text::new("Hierarchy"),
             TextFont::default(),
             TextColor(HEADER_COLOR),
             BackgroundColor(TEXT_COLOR)
         ));
         p.spawn(Node {
             flex_direction: FlexDirection::Column,
             width: Val::Percent(100.0),
             ..default()
         });
    }).id();

    // 5. Spawn Root Node (Overlay)
    let root = commands.spawn((
        Node {
             width: Val::Percent(100.0),
             height: Val::Percent(100.0),
             flex_direction: FlexDirection::Column,
             ..default()
        },
        GlobalTransform::default(),
        Transform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        EditorRoot,
        Pickable::IGNORE, // Don't block picking for the scene unless hitting UI
    )).id();

    // 6. Configure Root Children
    commands.entity(root).with_children(|parent| {
        // Menu Bar
        parent.spawn((
            menu_bar_style(),
            MenuBar,
            BackgroundColor(PANEL_COLOR),
            GlobalTransform::default(),
            Transform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        )).with_children(|menu| {
            // File Button
            menu.spawn((
                Button,
                menu_button_style(),
                BackgroundColor(BUTTON_COLOR_NORMAL),
                FileMenuButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("File"),
                    TextFont::default(),
                    TextColor(TEXT_COLOR),
                ));
            });

            // Dropdown Logic (Initially Hidden)
            menu.spawn((
                dropdown_style(),
                BackgroundColor(PANEL_COLOR),
                FileMenuDropdown,
                Visibility::Hidden,
                GlobalZIndex(10), // Ensure it's on top
            )).with_children(|dropdown| {
                // Save
                dropdown.spawn((
                    Button,
                    menu_button_style(),
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    MenuButtonAction { action: MenuAction::Save },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Save"),
                        TextFont::default(),
                        TextColor(TEXT_COLOR),
                    ));
                });

                // Save As
                dropdown.spawn((
                    Button,
                    menu_button_style(),
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    MenuButtonAction { action: MenuAction::SaveAs },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Save As"),
                        TextFont::default(),
                        TextColor(TEXT_COLOR),
                    ));
                });

                // Load
                dropdown.spawn((
                    Button,
                    menu_button_style(),
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    MenuButtonAction { action: MenuAction::Load },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Load"),
                        TextFont::default(),
                        TextColor(TEXT_COLOR),
                    ));
                });

                // Exit
                dropdown.spawn((
                    Button,
                    menu_button_style(),
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    MenuButtonAction { action: MenuAction::Exit },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Exit"),
                        TextFont::default(),
                        TextColor(TEXT_COLOR),
                    ));
                });
            });
        });
    });

    // 7. Attach Main Content
    let main_content = commands.spawn((
        Node {
             width: Val::Percent(100.0),
             flex_grow: 1.0,
             flex_direction: FlexDirection::Row,
             justify_content: JustifyContent::FlexStart,
             ..default()
        },
        GlobalTransform::default(),
        Transform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    )).id();

    // Add Main Content to Root
    commands.entity(root).add_child(main_content);

    // Add Children to Main Content in Order
    commands.entity(main_content).add_child(hierarchy);

    commands.entity(main_content).with_children(|main_parent| {
          // Resize Handle (Left -> Viewport)
          main_parent.spawn((
              resize_handle_style(),
              BackgroundColor(RESIZE_HANDLE_COLOR),
              ResizeHandle {
                  direction: ResizeDirection::Left,
                  target_panel: hierarchy,
              },
              Interaction::default(),
              GlobalTransform::default(),
              Transform::default(),
              Visibility::default(),
              InheritedVisibility::default(),
              ViewVisibility::default(),
          ));
    });

    commands.entity(main_content).add_child(center_col);

    commands.entity(main_content).with_children(|main_parent| {
          // Resize Handle (Viewport -> Right)
          main_parent.spawn((
              resize_handle_style(),
              BackgroundColor(RESIZE_HANDLE_COLOR),
              ResizeHandle {
                  direction: ResizeDirection::Right,
                  target_panel: inspector,
              },
              Interaction::default(),
              GlobalTransform::default(),
              Transform::default(),
              Visibility::default(),
              InheritedVisibility::default(),
              ViewVisibility::default(),
          ));
    });

    // Finally add inspector
    commands.entity(main_content).add_child(inspector);
}

pub fn toggle_editor(
    input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<EditorConfig>,
    mut query: Query<&mut Visibility, With<EditorRoot>>,
) {
    if input.just_pressed(KeyCode::F1) {
        config.show_editor = !config.show_editor;
        for mut vis in query.iter_mut() {
            *vis = if config.show_editor { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}

pub fn ui_resize_system(
    resize_reader: Res<AccumulatedMouseMotion>,
    mut interactions: Query<(&Interaction, &ResizeHandle)>,
    mut panels: Query<(&mut Node, &ResizablePanel)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut resizing_state: Local<Option<(Entity, ResizeDirection, f32)>>, // Track dragging state
    mut is_resizing: ResMut<crate::editor::resources::IsResizing>,
) {
    // Check for start dragging
    if resizing_state.is_none() {
        for (interaction, handle) in interactions.iter_mut() {
            if *interaction == Interaction::Pressed {
                // Find initial dimension
                if let Ok((node, _)) = panels.get(handle.target_panel) {


                     let initial_value = match handle.direction {
                         ResizeDirection::Left | ResizeDirection::Right => {
                             if let Val::Px(w) = node.width { w } else { 250.0 }
                         }
                         ResizeDirection::Up | ResizeDirection::Down => {
                             if let Val::Px(h) = node.height { h } else { 150.0 }
                         }
                     };
                     *resizing_state = Some((handle.target_panel, handle.direction, initial_value));
                     is_resizing.0 = true;
                }
            }
        }
    }

    // Handle Dragging
    if let Some((target, direction, _)) = *resizing_state {
        if mouse_button.pressed(MouseButton::Left) {
            let delta_x = resize_reader.delta.x;
            let delta_y = resize_reader.delta.y; // Capture Y delta

            if (delta_x != 0.0 || delta_y != 0.0) // Check both
                && let Ok((mut node, panel_config)) = panels.get_mut(target)
            {
                match direction {
                     ResizeDirection::Left => {
                         if let Val::Px(current_width) = node.width {
                             let new_width = (current_width + delta_x).clamp(panel_config.min_width, panel_config.max_width);
                             node.width = Val::Px(new_width);
                         }
                     }
                     ResizeDirection::Right => {
                         if let Val::Px(current_width) = node.width {
                             let new_width = (current_width - delta_x).clamp(panel_config.min_width, panel_config.max_width);
                             node.width = Val::Px(new_width);
                         }
                     }
                     ResizeDirection::Up => {
                         // Dragging Up (negative delta Y) should INCREASE height of a bottom panel.
                         if let Val::Px(current_height) = node.height {
                             // invert delta_y because up is negative but we want to increase size
                             let new_height = (current_height - delta_y).clamp(panel_config.min_height, panel_config.max_height);
                             node.height = Val::Px(new_height);
                         }
                     }


                     ResizeDirection::Down => {
                         // But we don't have top panels yet. Assuming bottom panel for consistent logic?
                         // If we had a top panel, dragging down (positive) would increase its height.
                         // So new_height = current + delta.
                         if let Val::Px(current_height) = node.height {
                              let new_height = (current_height + delta_y).clamp(panel_config.min_height, panel_config.max_height);
                              node.height = Val::Px(new_height);
                         }
                     }
                };
            }
        } else {
             // Stop dragging
             *resizing_state = None;
             is_resizing.0 = false;
             // Reset cursor? Bevy 0.15 doesn't expose cursor setting easily on Node yet.
        }
    }
}

pub fn ui_cursor_system(
    mut windows: Query<&mut Window>,
    resize_handles: Query<&Interaction, With<ResizeHandle>>,
) {
    let Some(_window) = windows.iter_mut().next() else { return };

    let mut _any_interacting = false;
    for interaction in resize_handles.iter() {
        if matches!(interaction, Interaction::Hovered | Interaction::Pressed) {
            _any_interacting = true;
            break;
        }
    }


    // Cursor change logic temporarily removed due to API incompatibility
}
