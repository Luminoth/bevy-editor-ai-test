use bevy::prelude::*;
use super::components::*;
use super::styles::*;
use crate::editor::resources::EditorConfig;
use bevy::input::mouse::AccumulatedMouseMotion;

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

    // 2. Spawn Root Node (Overlay)
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

    // 3. Configure Root Children
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


    // 4. Attach Inspector to Main Content Area
    // We need to access the main content area entity. We didn't capture it above.
    // Let's modify the above to capture it?
    // Or we can query for it, but better to capture.
    // Since we used `parent.spawn`, we can capture the ID inside the closure, but we can't extract it easily without Cells.

    // Easier way: Spawn Main Content Area detached first (like Inspector), then add to Root helpers?
    // Let's spawn Main Content Area detached.

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

    // Configure Main Content
    commands.entity(main_content).with_children(|main_parent| {
             // Left Panel (Hierarchy)
             let hierarchy = main_parent.spawn((
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
                     BackgroundColor(TEXT_COLOR) // Invert for header bg?
                 ));
                 // List container
                 p.spawn(Node {
                     flex_direction: FlexDirection::Column,
                     width: Val::Percent(100.0),
                     ..default()
                 });
             }).id();

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

             // Viewport (Center)
             main_parent.spawn((
                 Node {
                     width: Val::Px(0.0),
                     min_width: Val::Px(0.0),
                     flex_basis: Val::Px(0.0),
                     flex_grow: 1.0,
                     flex_shrink: 1.0,
                     height: Val::Percent(100.0),
                     ..default()
                 },
                 ViewportPanel,
                 GlobalTransform::default(),
                 Transform::default(),
                 Visibility::default(),
                 InheritedVisibility::default(),
                 ViewVisibility::default(),
             ));

             // Resize Handle (Viewport -> Right)
             main_parent.spawn((
                 resize_handle_style(),
                 BackgroundColor(RESIZE_HANDLE_COLOR),
                 ResizeHandle {
                     direction: ResizeDirection::Right,
                     target_panel: inspector, // Uses the pre-spawned inspector ID
                 },
                 Interaction::default(),
                 GlobalTransform::default(),
                 Transform::default(),
                 Visibility::default(),
                 InheritedVisibility::default(),
                 ViewVisibility::default(),
             ));
    });

    // Finally add inspector to main content (at the end)
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
    windows: Query<&Window>,
) {
    let window = windows.single();

    // Check for start dragging
    if resizing_state.is_none() {
        for (interaction, handle) in interactions.iter_mut() {
            if *interaction == Interaction::Pressed {
                // Find initial width
                if let Ok((node, _)) = panels.get(handle.target_panel) {
                     let initial_width = if let Val::Px(w) = node.width { w } else { 250.0 };
                     *resizing_state = Some((handle.target_panel, handle.direction, initial_width));
                     is_resizing.0 = true;
                }
            }
        }
    }

    // Handle Dragging
    if let Some((target, direction, _)) = *resizing_state {
        if mouse_button.pressed(MouseButton::Left) {
            let delta_x = resize_reader.delta.x;

            if delta_x != 0.0 {
                if let Ok((mut node, panel_config)) = panels.get_mut(target) {
                    if let Val::Px(current_width) = node.width {
                        let new_width = match direction {
                            ResizeDirection::Left => current_width + delta_x, // Expand hierarchy when dragging right
                            ResizeDirection::Right => current_width - delta_x, // Expand inspector when dragging left
                        };

                        // Clamp
                        let clamped_width = new_width.clamp(panel_config.min_width, panel_config.max_width);
                        node.width = Val::Px(clamped_width);
                    }
                }
            }
        } else {
             // Stop dragging
             *resizing_state = None;
             is_resizing.0 = false;
        }
    }
}

pub fn ui_cursor_system(
    mut windows: Query<&mut Window>,
    resize_handles: Query<&Interaction, With<ResizeHandle>>,
) {
    let Some(mut window) = windows.iter_mut().next() else { return };

    let mut any_interacting = false;
    for interaction in resize_handles.iter() {
        if matches!(interaction, Interaction::Hovered | Interaction::Pressed) {
            any_interacting = true;
            break;
        }
    }


    // Cursor change logic temporarily removed due to API incompatibility
}
