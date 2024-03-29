#![doc = include_str!("../README.md")]

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    render::color::HexColorError,
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
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<Palette, PaletteLoaderError>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let lospec: LospecJson = serde_json::from_slice(&bytes)?;
            let palette = Palette::try_from(lospec)?;
            Ok(palette)
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
        let colors: Result<Vec<Color>, HexColorError> =
            value.colors.iter().map(Color::hex).collect();

        Ok(Self(colors?))
    }
}
