//! Color constants

use bevy::prelude::*;

/// Background color (#5e4531)
pub const BG_COL: Color = Color::rgb(0.3686, 0.2706, 0.1922);

/// Grid color (#ffffff)
pub const GRID_COL: Color = Color::rgb(1., 1., 1.);

/// Frame color of the board (#e6d5b8)
pub const FRAME_COL: Color = Color::rgb(0.9019, 0.8392, 0.7216);

pub mod mino_color {
    //! Color constants for the minos

    use bevy::prelude::Color;

    /// Color of the I mino (#55ddff)
    pub const I: Color = Color::rgb(0.3333, 0.8667, 1.);

    /// Color of the O mino (#ffff55)
    pub const O: Color = Color::rgb(1., 1., 0.3333);

    /// Color of the L mino (#ffaa33)
    pub const L: Color = Color::rgb(1., 0.6667, 0.2);

    /// Color of the J mino (#1165b5)
    pub const J: Color = Color::rgb(0.0667, 0.3961, 0.7059);

    /// Color of the Z mino (#ff6688)
    pub const Z: Color = Color::rgb(1., 0.4, 0.5333);

    /// Color of the S mino (#66ee66)
    pub const S: Color = Color::rgb(0.4, 0.9333, 0.4);

    /// Color of the T mino (#983ca3)
    pub const T: Color = Color::rgb(0.6, 0.2353, 0.6392);

    /// Color of the garbage mino (#aaaaaa)
    pub const GARBAGE: Color = Color::rgb(0.6667, 0.6667, 0.6667);
}
