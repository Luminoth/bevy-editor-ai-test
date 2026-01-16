use bevy::prelude::*;

#[derive(Component)]
pub struct EditorRoot;

#[derive(Component)]
pub struct MenuBar;

#[derive(Component)]
pub struct FileMenuButton;

#[derive(Component)]
pub struct FileMenuDropdown;

#[derive(Component)]
pub enum MenuAction {
    Save,
    SaveAs,
    Load,
    Exit,
}

#[derive(Component)]
pub struct MenuButtonAction {
    pub action: MenuAction,
}

#[derive(Component)]
pub struct HierarchyPanel;

#[derive(Component)]
pub struct ViewportPanel;

#[derive(Component)]
pub struct InspectorPanel;

#[derive(Component)]
pub struct AddEntityButton;

#[derive(Component)]
pub struct HierarchyEntityRow {
    pub entity: Entity,
}

#[derive(Component)]
pub struct DeleteEntityButton;

#[derive(Component)]
pub struct AddComponentButton;

#[derive(Component)]
pub struct RemoveComponentButton(pub String); // Stores component type name

#[derive(Component)]
pub struct ComponentSearchInput;

#[derive(Component)]
pub struct ComponentAddButton(pub String); // Stores component name to add

#[derive(Component)]
pub struct PropertyInput {
    pub entity: Entity,
    pub component_type_id: std::any::TypeId,
    pub field_name: String,
    pub current_value: String,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum ResizeDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct ResizeHandle {
    pub direction: ResizeDirection,
    pub target_panel: Entity,
}

#[derive(Component)]
pub struct ResizablePanel {
     pub min_width: f32,
     pub max_width: f32,
     pub min_height: f32,
     pub max_height: f32,
}

impl Default for ResizablePanel {
    fn default() -> Self {
        Self {
            min_width: 150.0,
            max_width: 500.0,
            min_height: 50.0,
            max_height: 500.0,
        }
    }
}
