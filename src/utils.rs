use bevy::prelude::*;

pub struct DelayedDestroy {
    pub(crate) time_to_destroy: f32,
}

pub struct PluginUtils;

impl Plugin for PluginUtils {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(destroy_after.system());
    }
}

pub(super) fn destroy_after(
    mut commands: Commands,
    time: Res<Time>,
    q_destroy: Query<(Entity, &DelayedDestroy)>,
) {
    for (e, d) in q_destroy.iter() {
        if d.time_to_destroy < time.time_since_startup().as_secs_f32() {
            commands.entity(e).despawn();
        }
    }
}
