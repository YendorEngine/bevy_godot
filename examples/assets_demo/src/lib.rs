use bevy_asset_loader::*;
use bevy_godot::prelude::*;

fn init(_handle: &InitHandle) {}

fn build_app(app: &mut App) {
    AssetLoader::new(GameState::Loading)
        .with_collection::<GameAssets>()
        .continue_to_state(GameState::Playing)
        .build(app);

    app.add_state(GameState::Loading)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_cube_asset));
}

bevy_godot_init!(init, build_app);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    Playing,
}

#[derive(AssetCollection, Debug)]
struct GameAssets {
    #[asset(path = "simple_scene.tscn")]
    player: Handle<GodotResource>,
}

fn spawn_cube_asset(
    game_assets: Res<GameAssets>,
    mut assets: ResMut<Assets<GodotResource>>,
    mut scene_tree: SceneTreeRef,
    entities: Query<(&Name, &Transform)>,
) {
    let player_scn = assets.get_mut(&game_assets.player).unwrap();
    let player = player_scn
        .get::<PackedScene>()
        .unwrap()
        .instance(0)
        .unwrap();

    let spawn_location = entities
        .iter()
        .find_map(|(name, transform)| {
            if name.as_str() == "SpawnPosition" {
                Some(*transform)
            } else {
                None
            }
        })
        .unwrap();

    unsafe {
        player
            .assume_unique()
            .cast::<Spatial>()
            .unwrap()
            .set_transform(*spawn_location.as_godot());
    }

    unsafe {
        let current_scene = scene_tree.get().current_scene().unwrap();
        current_scene.assume_safe().add_child(player, true);
    };
}