#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameRoute {
    DeadMansDraw,
    BeastyBar,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameCategory {
    Card,
    Dice,
    Tile,
    Board,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Ready,
    ComingSoon,
}

#[derive(Clone)]
pub struct GameManifest {
    pub route: GameRoute,
    pub title: &'static str,
    pub description: &'static str,
    pub category: GameCategory,
    pub status: GameStatus,
    pub min_players: usize,
    pub max_players: usize,
}

impl GameManifest {
    pub fn player_count_label(&self) -> String {
        format!("{}–{} players", self.min_players, self.max_players)
    }
}

impl GameCategory {
    pub fn label(self) -> &'static str {
        match self {
            GameCategory::Card => "Card Game",
            GameCategory::Dice => "Dice Game",
            GameCategory::Tile => "Tile Game",
            GameCategory::Board => "Board Game",
        }
    }
}

impl GameStatus {
    pub fn label(self) -> &'static str {
        match self {
            GameStatus::Ready => "Ready to Play",
            GameStatus::ComingSoon => "Coming Soon",
        }
    }

    pub fn badge_class(self) -> &'static str {
        match self {
            GameStatus::Ready => {
                "rounded-full bg-emerald-600 px-3 py-1 text-xs font-bold text-white"
            }

            GameStatus::ComingSoon => {
                "rounded-full bg-amber-500 px-3 py-1 text-xs font-bold text-slate-900"
            }
        }
    }

    pub fn action_label(self) -> &'static str {
        match self {
            GameStatus::Ready => "Play",
            GameStatus::ComingSoon => "Coming soon",
        }
    }
}
