use crate::models::game_manifest::*;

pub const DEAD_MANS_DRAW: GameManifest = GameManifest {
    route: GameRoute::DeadMansDraw,
    title: "Dead Man's Draw",
    description: "Push your luck, bank your treasure, and avoid busting.",
    category: GameCategory::Card,
    status: GameStatus::Ready,
    min_players: 2,
    max_players: 4,
};
