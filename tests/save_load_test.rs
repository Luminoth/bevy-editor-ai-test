#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy_editor_ai_test::editor::menu::{save_system, LastSavedScene, SaveRequest};

    #[test]
    fn test_save_flow_emits_resource_logic() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_resource::<AppTypeRegistry>();
        app.init_resource::<LastSavedScene>();

        // Add save system
        app.add_systems(PostUpdate, save_system);

        // Spawn an entity to save (something valid)
        app.world_mut().spawn((
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            ViewVisibility::default(),
            InheritedVisibility::default(),
        ));

        // Trigger save
        app.world_mut().insert_resource(SaveRequest);

        // Run app
        app.update();

        // Check resource
        let saved = app.world().resource::<LastSavedScene>();
        assert!(!saved.0.is_empty(), "Should have saved scene data to resource");
        assert!(saved.0.contains("Transform"), "Should contain Transform component data");
    }
}
