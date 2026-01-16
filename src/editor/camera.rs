use bevy::prelude::*;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use crate::editor::resources::IsResizing;
use bevy::ecs::system::SystemParam;

#[derive(Component)]
pub struct EditorCamera {
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for EditorCamera {
    fn default() -> Self {
        Self {
            speed: 10.0,
            sensitivity: 0.003,
        }
    }
}

#[derive(SystemParam)]
pub struct EditorInput<'w> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    mouse: Res<'w, ButtonInput<MouseButton>>,
    mouse_motion: Res<'w, AccumulatedMouseMotion>,
    mouse_scroll: Res<'w, AccumulatedMouseScroll>,
    is_resizing: Res<'w, IsResizing>,
}

pub fn editor_camera_controls(
    mut windows: Query<&mut Window>,
    mut query: Query<(&EditorCamera, &mut Transform)>,
    input: EditorInput,
    time: Res<Time>,
) {
    if input.is_resizing.0 {
        return;
    }

    let Ok(_window) = windows.single_mut() else {
        return;
    };

    let rmb_held = input.mouse.pressed(MouseButton::Right);
    let lmb_held = input.mouse.pressed(MouseButton::Left);

    // Cursor handling (commented out due to API issues)
    // if rmb_held {
    //    window.cursor_options.visible = false;
    //    window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
    // } else {
    //    window.cursor_options.visible = true;
    //    window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
    // }

    for (camera, mut transform) in query.iter_mut() {
        // 1. Fly Controls (RMB Held)
        if rmb_held {
            // Rotation
            let rotation_move = input.mouse_motion.delta;

            if rotation_move.length_squared() > 0.0 {
                let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
                yaw -= rotation_move.x * camera.sensitivity;
                pitch -= rotation_move.y * camera.sensitivity;

                // Clamp pitch to avoid flipping over
                pitch = pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
            }

            // Movement
            let mut velocity = Vec3::ZERO;
            let local_forward = transform.forward();
            let local_right = transform.right();
            let local_up = transform.up();

            if input.keys.pressed(KeyCode::KeyW) {
                velocity += *local_forward;
            }
            if input.keys.pressed(KeyCode::KeyS) {
                velocity -= *local_forward;
            }
            if input.keys.pressed(KeyCode::KeyA) {
                velocity -= *local_right;
            }
            if input.keys.pressed(KeyCode::KeyD) {
                velocity += *local_right;
            }
            if input.keys.pressed(KeyCode::KeyE) {
                velocity += *local_up;
            }
            if input.keys.pressed(KeyCode::KeyQ) {
                velocity -= *local_up;
            }

            // Normalize velocity
            if velocity.length_squared() > 0.0 {
                velocity = velocity.normalize();
            }

            // Apply speed
            let mut current_speed = camera.speed;
            if input.keys.pressed(KeyCode::ShiftLeft) || input.keys.pressed(KeyCode::ShiftRight) {
                current_speed *= 2.0;
            }

            let translation = velocity * current_speed * time.delta_secs();
            transform.translation += translation;

        } else if lmb_held {
            // 2. Pan Controls (LMB Held)
            // "Pulling the world" style:
            // Drag Left (negative X) -> Camera moves Right (positive local X)
            // Drag Up (positive Y) -> Camera moves Down (negative local Y)
            let pan_move = input.mouse_motion.delta;

            if pan_move.length_squared() > 0.0 {
                // Scaling factor for pan - Reduced from 0.002 to 0.001 for slower panning
                let pan_sensitivity = camera.speed * 0.001;
                let local_right = transform.right();
                let local_up = transform.up();

                transform.translation -= *local_right * pan_move.x * pan_sensitivity;
                transform.translation += *local_up * pan_move.y * pan_sensitivity;
            }
        }

        // 3. Zoom (Always Active)
        // Simple zoom: move forward/back
        let scroll = input.mouse_scroll.delta.y;
        if scroll != 0.0 {
            // Zoom speed multiplier
            let zoom_speed = 2.0;
            let forward = transform.forward();
            transform.translation += *forward * scroll * zoom_speed;
        }
    }
}

use crate::editor::components::ViewportPanel;
// use bevy::ui::Node as UINode;
// use bevy::render::camera::Viewport;

pub fn sync_camera_viewport(
    mut camera_q: Query<(&mut Camera, &mut bevy::prelude::Projection), With<EditorCamera>>,
    viewport_q: Query<(&GlobalTransform, &ComputedNode), With<ViewportPanel>>,
    windows: Query<&Window>, // Relaxed query
) {
    // The `CameraProjection` trait is required to call `update` and `get_clip_from_view`.
    // Since we cannot easily import it (private/re-exported issues), we will query for `Projection`
    // and use it knowing that `Projection` implements `CameraProjection`.
    // However, to satisfy the compiler about the method existence, we might need the trait in scope.
    // Bevy seems to expose `bevy::render::camera::CameraProjection` but previous attempts failed.
    // If the methods are inherent on `Projection` implementation, we don't need the trait.
    // Let's assume they are inherent or available via `bevy::prelude`.

    let Some((mut camera, mut projection)) = camera_q.iter_mut().next() else {
        return;
    };
    let Some((global_transform, computed_node)) = viewport_q.iter().next() else {
        return;
    };
    let Some(window) = windows.iter().next() else {
        return;
    };

    let transform = global_transform.compute_transform();
    let translation = transform.translation;

    let size = computed_node.size();
    if size == Vec2::ZERO {
        camera.is_active = false;
        return;
    }

    let scale_factor = window.resolution.scale_factor();
    let window_physical_width = window.resolution.physical_width();
    let window_physical_height = window.resolution.physical_height();
    let window_physical_size = UVec2::new(window_physical_width, window_physical_height);

    // 1. Get Panel Center in World Space (0,0 is window center, Y-Up)
    let position_center_world = translation.truncate();

    // 2. Convert to Logical Screen Space (0,0 is Bottom-Left of window)
    let window_logical_size = Vec2::new(window.width(), window.height());
    let half_window = window_logical_size / 2.0;

    // Center in Screen Space = World Center + Half Window
    let position_center_screen = position_center_world + half_window;

    // 3. Calculate Bottom-Left Corner of Viewport in Screen Space
    let half_panel_size = size / 2.0;
    let bottom_left_screen = position_center_screen - half_panel_size;

    // 4. Convert to Physical Coordinates
    let physical_position = (bottom_left_screen * scale_factor).max(Vec2::ZERO).as_uvec2();
    let physical_size = (size * scale_factor).as_uvec2();

    // 5. Clamp to Window Size to prevent Scissor Rect panic
    let max_bound = physical_position + physical_size;

    let clamped_size = if max_bound.x > window_physical_size.x || max_bound.y > window_physical_size.y {
         let available_x = window_physical_size.x.saturating_sub(physical_position.x);
         let available_y = window_physical_size.y.saturating_sub(physical_position.y);
         UVec2::new(physical_size.x.min(available_x), physical_size.y.min(available_y))
    } else {
        physical_size
    };

    if clamped_size.x == 0 || clamped_size.y == 0 {
        camera.is_active = false;
        return;
    }

    camera.is_active = true;

    // Use take().unwrap_or_default() to avoid naming the private Viewport struct
    let mut viewport = camera.viewport.take().unwrap_or_default();
    viewport.physical_position = physical_position;
    viewport.physical_size = clamped_size;

    camera.viewport = Some(viewport);

    // Manually update projection to avoid one-frame lag
    if let Some(size) = camera.logical_viewport_size()
        && size.x != 0.0
        && size.y != 0.0
    {
        // Update aspect ratio
        projection.update(size.x, size.y);

        // Implement "Constant Pixel Scale" behavior.
        // We want objects to remain the same size in pixels regardless of viewport size.
        // If Viewport Height changes, we change FOV to show more/less world vertically.
        // If Viewport Width changes, standard aspect ratio handling (with fixed V-FOV) shows more/less horizontally.

        // Reference: 60 degrees Vertical FOV at 600px height.
        // tan(60/2) = tan(30) = 0.57735
        // C = 0.57735 / 600.0 = 0.00096225

        // const REFERENCE_FOV_Y: f32 = std::f32::consts::FRAC_PI_3; // 60 degrees - Unused
        const REFERENCE_HEIGHT: f32 = 600.0;
        const PIXEL_SCALE_FACTOR: f32 = (0.577350269) / REFERENCE_HEIGHT; // tan(30deg) / 600


        if let Projection::Perspective(ref mut pers) = *projection {
            // New Vertical FOV depends on current height
            pers.fov = 2.0 * (PIXEL_SCALE_FACTOR * size.y).atan();
        }

        let matrix = projection.get_clip_from_view();
        camera.computed.clip_from_view = matrix;
    }
}



// Spawn the necessary cameras for the editor
pub fn setup_editor_cameras(mut commands: Commands) {
    // 1. UI Camera (Renders the Editor UI)
    // Order 1 ensures it draws ON TOP of the 3D scene.
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));

    // 2. Editor Scene Camera (Renders the 3D World)
    // This camera's viewport is controlled by the `sync_camera_viewport` system.
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0, // Draw first (background)
            ..default()
        },
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        EditorCamera::default(),
    ));
}


#[cfg(test)]
mod tests {
    use super::*;

    use bevy::window::PrimaryWindow;
    use bevy::ui::{ComputedNode, Node};

    #[test]
    fn test_sync_camera_viewport() {
        let mut app = App::new();

        // Minimal setup
        app.add_plugins(MinimalPlugins);

        // We don't strictly need to register types for this simple system test

        // Spawn Window
        app.world_mut().spawn((
            Window {
                resolution: bevy::window::WindowResolution::new(800, 600).with_scale_factor_override(1.0),
                ..default()
            },
            PrimaryWindow,
        ));

        // Spawn Camera
        let camera_entity = app.world_mut().spawn((
            Camera::default(),
            EditorCamera::default(),
        )).id();

        // Spawn Viewport Panel with Mocked Data
        app.world_mut().spawn((
            Node::default(),
            ViewportPanel,
            // GlobalTransform::default() is Identity (0,0,0).
            // In World Space (Center Origin), (0,0) is the center of the window.
            // For an 800x600 window, screen center is (400, 300).
            GlobalTransform::default(),
            // Fields are public?
            ComputedNode { size: Vec2::new(400.0, 300.0), ..default() },
        ));

        // Add System
        app.add_systems(Update, sync_camera_viewport);

        // Run update
        app.update();

        // Check Camera Viewport
        let camera = app.world().get::<Camera>(camera_entity).unwrap();

        // If size is 0, viewport is not set (None).
        if camera.viewport.is_none() {
            // Panic or return with failure
            panic!("Camera viewport was not set! ComputedNode size might be 0 or unreachable.");
        }

        let viewport = camera.viewport.as_ref().unwrap();

        // Window 800x600. Panel 400x300 Centered.
        // Expected Viewport:
        // Left = (800 - 400)/2 = 200.
        // Bottom = (600 - 300)/2 = 150.
        assert_eq!(viewport.physical_position.x, 200);
        assert_eq!(viewport.physical_position.y, 150);
        assert_eq!(viewport.physical_size.x, 400);
        assert_eq!(viewport.physical_size.y, 300);
    }
}
