use bevy::prelude::*;
use bevy_lospec::{Palette, PalettePlugin};

#[derive(Resource, Deref)]
struct RusticPalette(Handle<Palette>);

// see https://github.com/NiklasEi/bevy_asset_loader for a better way to load
impl FromWorld for RusticPalette {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("no asset server");
        Self(asset_server.load("rustic-gb.json"))
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PalettePlugin))
        .init_resource::<RusticPalette>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_sprites)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_sprites(
    mut spawned: Local<bool>,
    mut commands: Commands,
    palettes: Res<Assets<Palette>>,
    rustic_palette: Res<RusticPalette>,
) {
    if *spawned {
        return;
    }

    if let Some(palette) = palettes.get(rustic_palette.id()) {
        let tile_size = 40.;
        let num_colors = palette.iter().count();
        let left_side = -((num_colors - 1) as f32 / 2.) * tile_size;
        for (i, &color) in palette.iter().enumerate() {
            info!("{color:?}");
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(tile_size)),
                    ..default()
                },
                Transform::from_xyz(left_side + i as f32 * tile_size, 0., 0.),
            ));
        }
        *spawned = true;
    }
}
