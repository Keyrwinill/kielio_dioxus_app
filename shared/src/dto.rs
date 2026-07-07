use serde::{Deserialize, Serialize};

use crate::games::dead_mans_draw::state::{GameState, GameConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponse {
    pub state: GameState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameAction {
    Draw,
    Bank,
    AiTurn,
    NewGame,
    StartNewGame {
        config: GameConfig,
    },
    SelectCannonTarget {
        target_player_index: usize,
        target_card_index: usize,
    },
    SelectHookTarget {
        target_card_index: usize,
    },
    SelectMapTarget {
        target_card_index: usize,
    },
    SelectSwordTarget {
        target_player_index: usize,
        target_card_index: usize,
    },
}