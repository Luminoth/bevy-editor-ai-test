#![allow(clippy::type_complexity)]
use crate::editor::components::*;
use crate::editor::styles::*;
use bevy::prelude::*;
use std::fs::File;
use std::io::Write;
use bevy::ecs::relationship::Relationship;
use std::path::PathBuf;

#[derive(Resource, Default)]
pub struct SceneInfo {
    pub file_path: Option<PathBuf>,
    pub is_dirty: bool,
}

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
    mut scene_info: ResMut<SceneInfo>,
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
                        std::process::exit(0);
                    }
                    MenuAction::Save => {
                         if scene_info.file_path.is_some() {
                             commands.insert_resource(SaveRequest);
                         } else {
                             // Treat as Save As
                             let dir = std::env::current_dir().unwrap_or_default();
                             if let Some(path) = rfd::FileDialog::new()
                                 .set_directory(&dir)
                                 .add_filter("Scene", &["scn.ron"])
                                 .save_file()
                             {
                                 scene_info.file_path = Some(path);
                                 commands.insert_resource(SaveRequest);
                             }
                         }
                    }
                    MenuAction::SaveAs => {
                        let dir = std::env::current_dir().unwrap_or_default();
                        if let Some(path) = rfd::FileDialog::new()
                            .set_directory(&dir)
                            .add_filter("Scene", &["scn.ron"])
                            .save_file()
                        {
                             scene_info.file_path = Some(path);
                             commands.insert_resource(SaveRequest);
                         }
                    }
                    MenuAction::Load => {
                        let dir = std::env::current_dir().unwrap_or_default();
                        if let Some(path) = rfd::FileDialog::new()
                            .set_directory(&dir)
                            .add_filter("Scene", &["scn.ron"])
                            .pick_file()
                        {
                             scene_info.file_path = Some(path);
                             commands.insert_resource(LoadRequest);
                        }
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

    // Reset dirty flag
    if let Some(mut info) = world.get_resource_mut::<SceneInfo>() {
        info.is_dirty = false;
    }
}

pub fn save_to_file_system(
    saved_scene: Res<LastSavedScene>,
    scene_info: Res<SceneInfo>,
) {
    if saved_scene.is_changed() {
        if saved_scene.0.is_empty() {
            return;
        }

        if let Some(path) = &scene_info.file_path {
            if let Ok(mut file) = File::create(path) {
                let _ = file.write_all(saved_scene.0.as_bytes());
                info!("Scene saved to {:?}", path);
            } else {
                error!("Failed to create scene file at {:?}", path);
            }
        } else {
            error!("Cannot save: No file path set in SceneInfo");
        }
    }
}

pub fn load_system(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    query: Query<Entity, (Without<EditorRoot>, Without<ChildOf>, Without<bevy::window::PrimaryWindow>, Without<crate::editor::camera::EditorCamera>)>,
    load_request: Option<Res<LoadRequest>>,
    mut scene_info: ResMut<SceneInfo>,
    type_registry: Res<AppTypeRegistry>,
    mut dynamic_scene_assets: ResMut<Assets<DynamicScene>>,
) {
    if load_request.is_none() {
        return;
    }

    // Need a path. Clone it to end the borrow of scene_info immediately.
    let path = if let Some(p) = &scene_info.file_path {
        p.clone()
    } else {
        error!("Cannot load: No file path set in SceneInfo");
        commands.remove_resource::<LoadRequest>();
        return;
    };

    // Clear current world
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // Manual deserialization to support absolute paths
    let Ok(scene_ron) = std::fs::read_to_string(&path) else {
        error!("Failed to read scene file: {:?}", path);
        commands.remove_resource::<LoadRequest>();
        return;
    };

    let mut deserializer = ron::Deserializer::from_str(&scene_ron).expect("Failed to create deserializer");
    let type_registry = type_registry.read();
    let scene_deserializer = bevy::scene::serde::SceneDeserializer {
        type_registry: &type_registry,
    };

    let Ok(dynamic_scene) = serde::de::DeserializeSeed::deserialize(scene_deserializer, &mut deserializer) else {
        error!("Failed to deserialize scene from {:?}", path);
        commands.remove_resource::<LoadRequest>();
        return;
    };

    // Add to assets to get a handle (needed for scene spawner)
    let scene_handle = dynamic_scene_assets.add(dynamic_scene);

    // Spawn new scene
    scene_spawner.spawn_dynamic(scene_handle);

    commands.remove_resource::<LoadRequest>();
    scene_info.is_dirty = false;
    info!("Scene loaded from {:?}", path);
}

pub fn update_window_title(
    scene_info: Res<SceneInfo>,
    mut window_query: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    if !scene_info.is_changed() {
        return;
    }

    if let Some(mut window) = window_query.iter_mut().next() {
        let title_base = "Bevy Editor AI Test";
        let path_str = if let Some(path) = &scene_info.file_path {
            path.file_name().and_then(|n| n.to_str()).unwrap_or("Untitled")
        } else {
            "Untitled"
        };

        let dirty_marker = if scene_info.is_dirty { "*" } else { "" };

        window.title = format!("{} - {}{}", title_base, path_str, dirty_marker);
    }
}
