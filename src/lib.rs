#![doc = include_str!("../README.md")]

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, Clone, TypePath)]
#[uuid = "777889bc-fb29-42bf-af78-da68fb5ba42d"]
pub struct Palette(pub Vec<Color>);

impl From<Vec<Color>> for Palette {
    fn from(colors: Vec<Color>) -> Self {
        Self(colors)
    }
}

const DEFAULT_PALETTE: [Color; 6] = [
    Color::PINK,
    Color::AZURE,
    Color::AQUAMARINE,
    Color::GOLD,
    Color::BLACK,
    Color::WHITE,
];

impl Default for Palette {
    fn default() -> Self {
        Self(DEFAULT_PALETTE.into())
    }
}

fn lightness(color: &Color) -> u32 {
    match color {
        Color::Rgba {
            red,
            green,
            blue,
            alpha: _,
        } => ((red + green + blue) * 256.0) as u32,
        _ => todo!(),
    }
}

fn manhattan_distance(lhs: &Color, rhs: &Color) -> u32 {
    match (lhs, rhs) {
        (
            Color::Rgba {
                red: lhs_red,
                green: lhs_green,
                blue: lhs_blue,
                alpha: _,
            },
            Color::Rgba {
                red: rhs_red,
                green: rhs_green,
                blue: rhs_blue,
                alpha: _,
            },
        ) => {
            let distance = (lhs_red - rhs_red).abs()
                + (lhs_green - rhs_green).abs()
                + (lhs_blue - rhs_blue).abs();
            (distance * 256.) as u32
        }
        _ => todo!(),
    }
}

impl Palette {
    pub fn lightest(&self) -> Color {
        *self.0.iter().max_by_key(|c| lightness(c)).unwrap()
    }

    pub fn darkest(&self) -> Color {
        *self.0.iter().min_by_key(|c| lightness(c)).unwrap()
    }

    pub fn closest(&self, color: Color) -> (usize, Color) {
        self.0
            .iter()
            .enumerate()
            .min_by_key(|(_i, c)| manhattan_distance(c, &color))
            .map(|(i, c)| (i, c.to_owned()))
            .unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Color> {
        self.0.iter()
    }
}

#[derive(Default)]
pub struct PaletteAssetLoader;

impl AssetLoader for PaletteAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let lospec: LospecJson = serde_json::from_slice(bytes)?;
            let palette = Palette::from(lospec);
            load_context.set_default_asset(LoadedAsset::new(palette));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(Default)]
pub struct PalettePlugin;

impl Plugin for PalettePlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Palette>()
            .init_asset_loader::<PaletteAssetLoader>();
    }
}

#[derive(Deserialize)]
struct LospecJson {
    pub colors: Vec<String>,
}

// TODO: should be TryFrom
impl From<LospecJson> for Palette {
    fn from(response: LospecJson) -> Self {
        response
            .colors
            .iter()
            .map(|c| match Color::hex(c) {
                Ok(color) => color,
                Err(_) => panic!("failed to parse color {} with length: {}", c, c.len()),
            })
            .collect::<Vec<Color>>()
            .into()
    }
}
