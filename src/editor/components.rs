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
    SaveAs,
    Load,
    Exit,
}

#[derive(Component)]
pub struct AddEntityButton;

#[derive(Component)]
pub struct DeleteEntityButton;

#[derive(Component)]
pub struct AddComponentButton;

#[derive(Component)]
pub struct RemoveComponentButton(pub String);

#[derive(Component)]
pub struct ComponentSearchInput;


#[derive(Component)]
pub struct ComponentAddButton(pub String);

#[derive(Component)]
pub struct PropertyInput {
    pub entity: Entity,
    pub component_type_id: std::any::TypeId,
    pub field_name: String,
    pub current_value: String,
}
