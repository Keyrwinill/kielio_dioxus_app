use crate::models::game_manifest::GameManifest;

use super::game_data::{
    beasty_bar::BEASTY_BAR,
    dead_mans_draw::DEAD_MANS_DRAW,
};

pub const GAMES: &[GameManifest] = &[
    DEAD_MANS_DRAW,
    BEASTY_BAR,
];