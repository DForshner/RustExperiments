use bevy::prelude::*;
use bevy_diagnostic::{Diagnostic, DiagnosticId, Diagnostics};

// Purposely creating the pitfall where you can miss events because the
// receiver doesn't handle the emitted events within 2 frames.
//
// From Bevy Cookbook:
// Events don't persist. They are stored until the end of the next frame, after which they are lost.
// If your systems do not handle events every frame, you could miss some.

pub const EMITTED_COUNT: DiagnosticId =
    DiagnosticId::from_u128(21302464753369276838568507794995836440);
pub const RECEIVED_COUNT: DiagnosticId =
    DiagnosticId::from_u128(21302838753369276838568507794995836880);

const TIMESTEP_SLOW: f64 = 5.0 / 60.0;
const TIMESTEP_FAST: f64 = 1.0 / 60.0;

fn main() {
    use bevy::core::FixedTimestep;
    use bevy_diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin};

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_event::<MyEvent>()
        .init_resource::<EventTriggerState>()
        .init_resource::<DiagnosticState>()
        .add_startup_system(startup_system)
        .add_system_set(
            SystemSet::new()
                // Check for events every 5/60 sec
                .with_run_criteria(FixedTimestep::step(TIMESTEP_FAST))
                .with_system(event_trigger_system),
        )
        .add_system_set(
            SystemSet::new()
                // Emit events every 1/60 sec
                .with_run_criteria(FixedTimestep::step(TIMESTEP_SLOW))
                .with_system(event_listener_system),
        )
        .run();
}

fn startup_system(mut diagnostics: ResMut<Diagnostics>) {
    diagnostics.add(Diagnostic::new(EMITTED_COUNT, "emitted", 3));
    diagnostics.add(Diagnostic::new(RECEIVED_COUNT, "received", 3));
}

struct MyEvent {
    pub message: String,
}

struct EventTriggerState {
    event_timer: Timer,
}

impl Default for EventTriggerState {
    fn default() -> Self {
        EventTriggerState {
            event_timer: Timer::from_seconds(1.0, true),
        }
    }
}

#[derive(Default)]
struct DiagnosticState {
    events_emitted: usize,
    events_received: usize,
}

fn event_trigger_system(
    mut diagnostics: ResMut<Diagnostics>,
    time: Res<Time>,
    mut state: ResMut<EventTriggerState>,
    mut diagnostic_state: ResMut<DiagnosticState>,
    mut my_events: EventWriter<MyEvent>,
) {
    //info!("event_trigger_system");
    if state.event_timer.tick(time.delta()).finished() {
        diagnostic_state.events_emitted += 1;

        diagnostics.add_measurement(EMITTED_COUNT, diagnostic_state.events_emitted as f64);

        my_events.send(MyEvent {
            message: format!(
                "The event! total emitted: [{}]",
                diagnostic_state.events_emitted,
            ),
        });
    }
}

// prints events as they come in
fn event_listener_system(
    mut diagnostics: ResMut<Diagnostics>,
    mut diagnostic_state: ResMut<DiagnosticState>,
    mut events: EventReader<MyEvent>,
) {
    diagnostics.add_measurement(RECEIVED_COUNT, diagnostic_state.events_emitted as f64);

    for my_event in events.iter() {
        diagnostic_state.events_received += 1;

        info!(
            "{} total received: [{}]",
            my_event.message, diagnostic_state.events_received
        );
    }
}