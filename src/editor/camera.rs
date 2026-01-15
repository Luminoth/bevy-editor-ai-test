use bevy::prelude::*;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};

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

pub fn editor_camera_controls(
    mut windows: Query<&mut Window>,
    mut query: Query<(&EditorCamera, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
) {
    let Ok(_window) = windows.single_mut() else {
        return;
    };

    let rmb_held = mouse.pressed(MouseButton::Right);
    let lmb_held = mouse.pressed(MouseButton::Left);

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
            let rotation_move = mouse_motion.delta;

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

            if keys.pressed(KeyCode::KeyW) {
                velocity += *local_forward;
            }
            if keys.pressed(KeyCode::KeyS) {
                velocity -= *local_forward;
            }
            if keys.pressed(KeyCode::KeyA) {
                velocity -= *local_right;
            }
            if keys.pressed(KeyCode::KeyD) {
                velocity += *local_right;
            }
            if keys.pressed(KeyCode::KeyE) {
                velocity += *local_up;
            }
            if keys.pressed(KeyCode::KeyQ) {
                velocity -= *local_up;
            }

            // Normalize velocity
            if velocity.length_squared() > 0.0 {
                velocity = velocity.normalize();
            }

            // Apply speed
            let mut current_speed = camera.speed;
            if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
                current_speed *= 2.0;
            }

            let translation = velocity * current_speed * time.delta_secs();
            transform.translation += translation;

        } else if lmb_held {
            // 2. Pan Controls (LMB Held)
            // "Pulling the world" style:
            // Drag Left (negative X) -> Camera moves Right (positive local X)
            // Drag Up (positive Y) -> Camera moves Down (negative local Y)
            let pan_move = mouse_motion.delta;

            if pan_move.length_squared() > 0.0 {
                // Scaling factor for pan
                let pan_sensitivity = camera.speed * 0.002;
                let local_right = transform.right();
                let local_up = transform.up();

                transform.translation -= *local_right * pan_move.x * pan_sensitivity;
                transform.translation += *local_up * pan_move.y * pan_sensitivity;
            }
        }

        // 3. Zoom (Always Active)
        // Simple zoom: move forward/back
        let scroll = mouse_scroll.delta.y;
        if scroll != 0.0 {
            // Zoom speed multiplier
            let zoom_speed = 2.0;
            let zoom_move = *transform.forward() * scroll * zoom_speed;
            transform.translation += zoom_move;
        }
    }
}
