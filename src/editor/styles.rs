#![allow(dead_code)]
use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const PANEL_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HEADER_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
pub const BUTTON_COLOR_NORMAL: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BUTTON_COLOR_HOVER: Color = Color::srgb(0.35, 0.35, 0.35);
pub const BUTTON_COLOR_PRESSED: Color = Color::srgb(0.45, 0.45, 0.45);
pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const RESIZE_HANDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
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
        width: Val::Px(250.0),
        min_width: Val::Px(250.0),
        flex_basis: Val::Px(250.0),
        flex_grow: 0.0,
        flex_shrink: 0.0,
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        border: UiRect::all(Val::Px(1.0)),
        padding: UiRect::all(Val::Px(4.0)),
        ..default()
    }
}

pub fn viewport_style() -> Node {
    Node {
        width: Val::Px(0.0), // Start from 0
        min_width: Val::Px(0.0),
        flex_basis: Val::Px(0.0),
        flex_grow: 1.0,
        flex_shrink: 1.0, // Absorb overflow if necessary
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
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 14.0,
        ..default()
    }
}

pub fn menu_bar_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Px(25.0),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::FlexStart,
        padding: UiRect::axes(Val::Px(5.0), Val::Px(0.0)),
        ..default()
    }
}

pub fn menu_button_style() -> Node {
    Node {
        padding: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn dropdown_style() -> Node {
    Node {
        position_type: PositionType::Absolute,
        top: Val::Px(25.0),
        left: Val::Px(0.0),
        width: Val::Px(100.0),
        flex_direction: FlexDirection::Column,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

pub fn resize_handle_style() -> Node {
    Node {
        width: Val::Px(8.0),
        height: Val::Percent(100.0),
        // cursor: CursorIcon::ColResize, // Cursor not supported on Node in this Bevy version
        ..default()
    }
}
