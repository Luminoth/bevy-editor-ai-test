use bevy::prelude::*;

#[derive(Component)]
pub struct EditorRoot;

#[derive(Component)]
pub struct HierarchyPanel;

#[derive(Component)]
pub struct InspectorPanel;

#[derive(Component)]
pub struct ViewportPanel;

#[derive(Component)]
pub struct HierarchyEntityRow {
    pub entity: Entity,
}
