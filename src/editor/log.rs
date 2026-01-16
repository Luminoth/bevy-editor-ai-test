use bevy::prelude::*;
use tracing::{self, Subscriber};
use tracing_subscriber::{Layer, registry::LookupSpan};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;


/// A log message captured from the application.
#[derive(Clone, Debug)]
pub struct LogMessage {
    pub level: tracing::Level,
    pub message: String,
    pub timestamp: f64, // simplified timestamp (seconds since startup)
}

/// Resource to store logs for display in the UI.
#[derive(Resource, Default)]
pub struct EditorLogs {
    pub messages: VecDeque<LogMessage>,
}

/// Shared buffer for captured logs before they are synced to the ECS resource.
/// We use a Mutex here because the interaction is between a tracing thread (any thread)
/// and the main Bevy thread.
#[derive(Clone, Default)]
pub struct SharedLogBuffer {
    pub buffer: Arc<Mutex<Vec<LogMessage>>>,
}

/// A Tracing Layer that captures logs into a shared buffer.
pub struct CaptureLayer {
    pub shared_buffer: SharedLogBuffer,
    pub start_time: std::time::Instant,
}

impl Default for CaptureLayer {
    fn default() -> Self {
        Self {
            shared_buffer: SharedLogBuffer::default(),
            start_time: std::time::Instant::now(),
        }
    }
}

impl<S> Layer<S> for CaptureLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut visitor = MessageVisitor::default();
        event.record(&mut visitor);

        let message = LogMessage {
            level: *event.metadata().level(),
            message: visitor.message,
            timestamp: self.start_time.elapsed().as_secs_f64(),
        };

        if let Ok(mut buffer) = self.shared_buffer.buffer.lock() {
            buffer.push(message);
        }
    }
}

#[derive(Default)]
struct MessageVisitor {
    message: String,
}

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }
}

/// System to transfer logs from the shared buffer to the Bevy Resource.
pub fn transfer_logs_system(
    mut editor_logs: ResMut<EditorLogs>,
    capture_layer: Option<Res<SharedLogBufferResource>>, // We'll put the shared buffer in a resource too
) {
    if let Some(shared) = capture_layer {
        if let Ok(mut buffer) = shared.0.buffer.lock() {
            if !buffer.is_empty() {
                editor_logs.messages.extend(buffer.drain(..));
                // Optional: Limit log history
                while editor_logs.messages.len() > 1000 {
                    editor_logs.messages.pop_front();
                }
            }
        }
    }
}

/// Wrapper resource to hold the shared buffer so systems can access it.
#[derive(Resource)]
pub struct SharedLogBufferResource(pub SharedLogBuffer);


/// System to render the log panel UI.
pub fn log_panel_ui_system(
    editor_logs: Res<EditorLogs>,
    log_panel_query: Query<Entity, With<LogPanelContent>>,


    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Some(panel) = log_panel_query.iter().next() else { return };

    if !editor_logs.is_changed() {
        return;
    }

    // Clear existing logs
    commands.entity(panel).replace_children(&[]);


    commands.entity(panel).with_children(|p| {
        for log in editor_logs.messages.iter().rev() { // Show newest first? Or scroll? Let's show newest at bottom usually...
             // Standard logs: oldest at top.
             // If we use Column with FlexStart, we need to scroll.
             // For now, let's reverse and show newest at top for visibility if scrolling isn't implemented.
             // Let's stick to newest at top for this simple panel so we don't need complex scroll logic yet.

            let color = match log.level {
                tracing::Level::ERROR => Color::srgb(1.0, 0.3, 0.3),
                tracing::Level::WARN => Color::srgb(1.0, 0.8, 0.3),
                tracing::Level::INFO => Color::srgb(0.9, 0.9, 0.9),
                _ => Color::srgb(0.7, 0.7, 0.7),
            };

            p.spawn((
                Text::new(format!("[{:.3}] {}", log.timestamp, log.message)),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(color),
            ));
        }
    });
}

// Components
#[derive(Component)]
pub struct LogPanel;

#[derive(Component)]
pub struct LogPanelContent;

/// Helper to create a LogPlugin configured with the editor's CaptureLayer.
/// Use this with `DefaultPlugins.set(editor::log::log_plugin())`.
pub fn log_plugin() -> bevy::log::LogPlugin {
    bevy::log::LogPlugin {
        custom_layer: |app| {
            let capture_layer = CaptureLayer::default();
            let shared_buffer = capture_layer.shared_buffer.clone();
            app.insert_resource(SharedLogBufferResource(shared_buffer));
            Some(Box::new(capture_layer))
        },
        ..default()
    }
}
