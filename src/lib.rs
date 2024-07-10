#![doc = include_str!("../README.md")]

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    color::HexColorError,
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

/// A lospec palette, a collection of colors
#[derive(Asset, Debug, Deserialize, Clone, TypePath)]
pub struct Palette(pub Vec<Color>);

impl From<Vec<Color>> for Palette {
    fn from(colors: Vec<Color>) -> Self {
        Self(colors)
    }
}

const DEFAULT_PALETTE: [Color; 6] = [
    Color::srgb(1.0, 0.08, 0.58), // pink
    Color::srgb(0.94, 1.0, 1.0),  // azure
    Color::srgb(0.49, 1.0, 0.83), // aquamarine
    Color::srgb(1.0, 0.84, 0.0),  // gold
    Color::BLACK,
    Color::WHITE,
];

impl Default for Palette {
    fn default() -> Self {
        Self(DEFAULT_PALETTE.into())
    }
}

fn lightness(color: &Color) -> u32 {
    (color.luminance() * 256.0) as u32
}

fn manhattan_distance(lhs: &Color, rhs: &Color) -> u32 {
    let lhs = lhs.to_srgba();
    let rhs = rhs.to_srgba();
    let Srgba {
        red: lhs_red,
        green: lhs_green,
        blue: lhs_blue,
        ..
    } = lhs;
    let Srgba {
        red: rhs_red,
        green: rhs_green,
        blue: rhs_blue,
        ..
    } = rhs;

    let distance =
        (lhs_red - rhs_red).abs() + (lhs_green - rhs_green).abs() + (lhs_blue - rhs_blue).abs();
    (distance * 256.) as u32
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
pub struct PaletteLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PaletteLoaderError {
    #[error("Couldn't load palette: {0}")]
    Io(#[from] std::io::Error),
    #[error("Couldn't parse palette json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Couldn't parse hex color: {0}")]
    HexColor(#[from] HexColorError),
}

impl AssetLoader for PaletteLoader {
    type Asset = Palette;
    type Settings = ();
    type Error = PaletteLoaderError;
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Palette, PaletteLoaderError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let lospec: LospecJson = serde_json::from_slice(&bytes)?;
        let palette = Palette::try_from(lospec)?;
        Ok(palette)
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(Default)]
pub struct PalettePlugin;

impl Plugin for PalettePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Palette>()
            .init_asset_loader::<PaletteLoader>();
    }
}

#[derive(Deserialize)]
struct LospecJson {
    pub colors: Vec<String>,
}

impl TryFrom<LospecJson> for Palette {
    type Error = HexColorError;

    fn try_from(value: LospecJson) -> Result<Self, Self::Error> {
        let colors: Result<Vec<Color>, HexColorError> = value
            .colors
            .iter()
            .map(|c| Srgba::hex(c).map(Color::from))
            .collect();

        Ok(Self(colors?))
    }
}
