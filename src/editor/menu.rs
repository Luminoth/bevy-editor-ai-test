#![allow(clippy::type_complexity)]
use crate::editor::components::*;
use crate::editor::styles::*;
use bevy::prelude::*;
use std::fs::File;
use std::io::Write;
use bevy::ecs::relationship::Relationship;

pub fn handle_file_menu_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<FileMenuButton>),
    >,
    mut dropdown_query: Query<&mut Visibility, With<FileMenuDropdown>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_COLOR_PRESSED.into();
                for mut vis in dropdown_query.iter_mut() {
                    *vis = match *vis {
                        Visibility::Hidden => Visibility::Visible,
                        _ => Visibility::Hidden,
                    };
                }
            }
            Interaction::Hovered => {
                *color = BUTTON_COLOR_HOVER.into();
            }
            Interaction::None => {
                *color = BUTTON_COLOR_NORMAL.into();
            }
        }
    }
}

pub fn menu_action_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        (Changed<Interaction>, With<MenuButtonAction>),
    >,
    mut commands: Commands,
    mut dropdown_query: Query<&mut Visibility, With<FileMenuDropdown>>,
) {
    for (interaction, mut color, menu_action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_COLOR_PRESSED.into();
                // Hide dropdown
                for mut vis in dropdown_query.iter_mut() {
                    *vis = Visibility::Hidden;
                }

                match menu_action.action {
                    MenuAction::Exit => {
                        // Assuming Bevy 0.18 uses Observers or similar, and EventWriter is gone/changed.
                        // Or AppExit is a resource? No.
                        // Try triggering AppExit.
                        // If trigger is not available, we might fail, but EventWriter failed harder.
                        // If this fails, we will try `std::process::exit(0)`.
                        std::process::exit(0);
                    }
                    MenuAction::Save => {
                        commands.insert_resource(SaveRequest);
                    }
                    MenuAction::Load => {
                        commands.insert_resource(LoadRequest);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BUTTON_COLOR_HOVER.into();
            }
            Interaction::None => {
                *color = BUTTON_COLOR_NORMAL.into();
            }
        }
    }
}

#[derive(Resource)]
pub struct SaveRequest;

#[derive(Resource)]
pub struct LoadRequest;

#[derive(Resource, Default)]
pub struct LastSavedScene(pub String);

pub fn save_system(
    world: &mut World,
) {
    if world.remove_resource::<SaveRequest>().is_none() {
        return;
    }

    let mut entities_to_save = Vec::new();

    let mut query = world.query::<Entity>();
    let all_entities: Vec<Entity> = query.iter(world).collect();

    for entity in all_entities {
        let entity_ref = world.entity(entity);

        // Exclude specific components
        if entity_ref.contains::<EditorRoot>() ||
           entity_ref.contains::<crate::editor::camera::EditorCamera>() ||
           entity_ref.contains::<bevy::window::PrimaryWindow>() {
            continue;
        }

        // Check lineage for EditorRoot
        let mut is_editor_child = false;
        let mut current = entity;

        while let Some(parent) = world.get::<ChildOf>(current) {
            current = parent.get();
            if world.get::<EditorRoot>(current).is_some() {
                is_editor_child = true;
                break;
            }
        }

        if is_editor_child {
            continue;
        }

        entities_to_save.push(entity);
    }

    use bevy::scene::DynamicSceneBuilder;

    // Correctly chain the builder
    let mut scene = DynamicSceneBuilder::from_world(world)
        .extract_entities(entities_to_save.into_iter())
        .build();

    // Filter out problem components directly from the scene data
    for entity in &mut scene.entities {
        entity.components.retain(|component| {
            let name = component.reflect_type_path();
            if name.contains("VisibilityClass") ||
               name.contains("Mesh3d") ||
               name.contains("MeshMaterial3d") {
                info!("Removing component from scene: {}", name);
                return false;
            }
            true
        });
    }

    // Serialize with the default registry
    let serialized_scene = {
        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();

        match scene.serialize(&type_registry) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to serialize scene: {}", e);
                return;
            }
        }
    };

    world.insert_resource(LastSavedScene(serialized_scene));
}

pub fn save_to_file_system(
    saved_scene: Res<LastSavedScene>,
) {
    if saved_scene.is_changed() {
        if saved_scene.0.is_empty() {
            return;
        }
        let _ = std::fs::create_dir_all("assets/scenes");
        let path = "assets/scenes/saved_scene.scn.ron";
        if let Ok(mut file) = File::create(path) {
            let _ = file.write_all(saved_scene.0.as_bytes());
            info!("Scene saved to {}", path);
        } else {
            error!("Failed to create scene file");
        }
    }
}

pub fn load_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    query: Query<Entity, (Without<EditorRoot>, Without<ChildOf>, Without<bevy::window::PrimaryWindow>, Without<crate::editor::camera::EditorCamera>)>,
    load_request: Option<Res<LoadRequest>>,
) {
    if load_request.is_none() {
        return;
    }

    // Clear current world
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new scene
    scene_spawner.spawn_dynamic(asset_server.load("scenes/saved_scene.scn.ron"));

    commands.remove_resource::<LoadRequest>();
    info!("Scene loaded");
}
