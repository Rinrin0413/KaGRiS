//! Constants containing sizes, lengths, and dimensions etc. for KaGRiS

/// The board height as a percentage of the window height.
pub const BOARD_HEIGHT_RATIO: f32 = 0.797;

/// The board width as a percentage of the window HEIGHT.
pub const BOARD_WIDTH_RATIO: f32 = BOARD_HEIGHT_RATIO / 2.;

/// Board Y-offset percentage of the window height.
///
/// # Examples
///
/// - 0.0: the board bottom is at the window bottom.
/// - 0.5: the board bottom is at the window center.
/// - 1.0: the board bottom is at the window top.
pub const BOARD_OFFSET_RATIO_Y: f32 = 0.067;

/// The thickness of the grids.
pub const GRID_THICKNESS: f32 = 2.;

/// The thickness of the frame of the board.
pub const FRAME_THICKNESS: f32 = 5.;
