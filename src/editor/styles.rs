#![allow(dead_code)]
use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const PANEL_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HEADER_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
pub const BUTTON_COLOR_NORMAL: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BUTTON_COLOR_HOVER: Color = Color::srgb(0.35, 0.35, 0.35);
pub const BUTTON_COLOR_PRESSED: Color = Color::srgb(0.45, 0.45, 0.45);
pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const SELECTED_COLOR: Color = Color::srgb(0.2, 0.4, 0.6);

pub fn root_node_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        ..default()
    }
}

pub fn sidebar_style() -> Node {
    Node {
        width: Val::Px(250.0), // Fixed width sidebar
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        border: UiRect::all(Val::Px(1.0)),
        padding: UiRect::all(Val::Px(4.0)),
        ..default()
    }
}

pub fn viewport_style() -> Node {
    Node {
        flex_grow: 1.0,
        height: Val::Percent(100.0),
        ..default()
    }
}

pub fn header_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Px(30.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::bottom(Val::Px(4.0)),
        ..default()
    }
}

pub fn text_style(asset_server: &AssetServer) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"), // Default bevy font usually
        font_size: 14.0,
        ..default()
    }
}
