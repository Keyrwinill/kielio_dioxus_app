use crate::models::game_manifest::*;

pub const BEASTY_BAR: GameManifest = GameManifest {
    route: GameRoute::BeastyBar,
    title: "Beasty Bar",
    description: "Coming Soon",
    category: GameCategory::Card,
    status: GameStatus::ComingSoon,
    min_players: 2,
    max_players: 4,
};
