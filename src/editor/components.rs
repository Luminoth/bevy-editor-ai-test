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

#[derive(Component)]
pub struct MenuBar;

#[derive(Component)]
pub struct FileMenuButton;

#[derive(Component)]
pub struct FileMenuDropdown;

#[derive(Component)]
pub struct MenuButtonAction {
    pub action: MenuAction,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    Save,
    Load,
    Exit,
}
