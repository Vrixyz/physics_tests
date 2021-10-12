use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::{na::Vector2, prelude::*};

pub struct PluginPhysics;

impl Plugin for PluginPhysics {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(DefaultPlugins)
            .add_startup_system(init_assets.system())
            .add_startup_system(setup_physics.system())
            .add_system(input_aim_system.system());
    }
}

pub struct MainCamera;

pub struct UnitMaterial {
    pub mat: Handle<ColorMaterial>,
}

pub fn setup_physics(mut configuration: ResMut<RapierConfiguration>) {
    configuration.gravity = Vector2::zeros();
    configuration.scale = 1f32;
}
fn init_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("icon.png");
    let mat = materials.add(texture_handle.into());
    commands.insert_resource(UnitMaterial { mat });
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation.z = 10f32;
    commands.spawn_bundle(camera).insert(MainCamera);
}

fn input_aim_system(
    time: Res<Time>,
    mut commands: Commands,
    window: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut ev_cursor: EventReader<CursorMoved>,
    q_camera: Query<&Transform, With<MainCamera>>,
    assets: Res<UnitMaterial>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        if let Some(pos) = ev_cursor.iter().last() {
            let time_since_startup = time.time_since_startup().as_secs_f32();

            let camera_transform = q_camera.iter().next().unwrap();
            let wnd = window.get(pos.id).unwrap();
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = pos.position - size / 2.0;

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

            spawn_bullet(
                Vec2::new(pos_wld.x, pos_wld.y),
                time_since_startup + 0.5f32,
                &mut commands,
                &assets.mat,
            );
        }
    }
}

pub fn spawn_bullet<'a, 'b>(
    origin: Vec2,
    time_to_destroy: f32,
    commands: &'b mut Commands<'a>,
    mat: &Handle<ColorMaterial>,
) -> EntityCommands<'a, 'b> {
    let collider_size = 0.5;

    let mut body = RigidBodyBundle {
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };

    body.mass_properties.flags = RigidBodyMassPropsFlags::ROTATION_LOCKED;
    let spawn_position = origin;
    body.position = spawn_position.into();
    let mut ret = commands.spawn();
    ret.insert_bundle(body)
        .insert_bundle(SpriteBundle {
            material: mat.clone(),
            sprite: Sprite::new(Vec2::splat(collider_size * 2f32 * 32f32)),
            ..Default::default()
        })
        .insert(Transform::from_translation(spawn_position.extend(0.0)))
        .insert(GlobalTransform::from_translation(
            spawn_position.extend(0.0),
        ))
        .insert_bundle(ColliderBundle {
            flags: (ActiveEvents::CONTACT_EVENTS).into(),
            position: [0.0, 0.0].into(),
            shape: ColliderShape::ball(collider_size / 2.0),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(0))
        .insert(crate::utils::DelayedDestroy { time_to_destroy });

    ret
}
