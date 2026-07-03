use serde::{Deserialize, Serialize};

use super::card::Card;
use super::deck::create_deck;
use super::player::Player;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    PlayerTurn,
    WaitingForCannonTarget,
    WaitingForHookTarget,
    WaitingForMapTarget,
    WaitingForSwordTarget,
    GameOver,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetType {
    OpponentBank,
    OwnBank,
    Discard,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PendingAbility {
    Cannon,
    Hook,
    Map,
    Sword,
}

impl PendingAbility {
    pub fn target_type(&self) -> TargetType {
        match self {
            PendingAbility::Cannon => TargetType::OpponentBank,
            PendingAbility::Sword => TargetType::OpponentBank,
            PendingAbility::Hook => TargetType::OwnBank,
            PendingAbility::Map => TargetType::Discard,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionOwner {
    CurrentPlayer,
    Opponent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionSource {
    PlayerBank { owner: SelectionOwner },
    Discard,
    MapChoices,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingSelection {
    pub source: SelectionSource,
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub deck: Vec<Card>,
    pub discard: Vec<Card>,
    pub play_area: Vec<Card>,
    pub players: Vec<Player>,
    pub current_player_index: usize,
    pub message: String,
    pub game_over: bool,
    pub revealed_next_card: Option<Card>,
    pub phase: GamePhase,
    pub pending_ability: Option<PendingAbility>,
    pub map_choices: Vec<Card>,
    pub treasure_cards: Vec<Card>,
    pub game_log: Vec<String>,
    pub pending_selection: Option<PendingSelection>,
    pub kraken_required_cards: usize,
    pub anchor_index: Option<usize>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            deck: create_deck(),
            discard: Vec::new(),
            play_area: Vec::new(),
            players: vec![
                Player::new("You", false),
                Player::new("AI", true),
            ],
            current_player_index: 0,
            message: "Game started. Draw a card.".to_string(),
            game_over: false,
            revealed_next_card: None,
            phase: GamePhase::PlayerTurn,
            pending_ability: None,
            map_choices: Vec::new(),
            treasure_cards: Vec::new(),
            game_log: vec!["Game started.".to_string()],
            pending_selection: None,
            kraken_required_cards: 0,
            anchor_index: None,
        }
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player_index]
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player_index]
    }

    pub fn next_player(&mut self) {
        self.play_area.clear();
        self.map_choices.clear();
        self.anchor_index = None;
        self.pending_selection = None;
        self.pending_ability = None;
        self.kraken_required_cards = 0;

        self.current_player_index =
            (self.current_player_index + 1) % self.players.len();

        self.add_log(format!("{}'s turn.", self.current_player().name));
    }

    pub fn add_log(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.message = message.clone();
        self.game_log.push(message);
    }

    pub fn is_waiting_for_opponent_bank_target(&self) -> bool {
        matches!(
            self.phase,
            GamePhase::WaitingForCannonTarget
                | GamePhase::WaitingForSwordTarget
        )
    }

    pub fn can_select_player_bank(&self, player_index: usize) -> bool {
        match &self.pending_selection {
            Some(PendingSelection {
                source: SelectionSource::PlayerBank { owner },
                ..
            }) => match owner {
                SelectionOwner::CurrentPlayer => {
                    player_index == self.current_player_index
                }
                SelectionOwner::Opponent => {
                    player_index != self.current_player_index
                }
                
            },

            _ => false,
        }
    }

    pub fn can_select_bank_card(
        &self,
        player_index: usize,
        card_index: usize,
    ) -> bool {
        if !self.can_select_player_bank(player_index) {
            return false;
        }

        match self.pending_ability {
            Some(PendingAbility::Cannon) => {
                self.is_top_card_of_suit_stack(player_index, card_index)
            }
            Some(PendingAbility::Sword) => {
                let Some(card) = self.players[player_index].bank.get(card_index) else {
                    return false;
                };

                !self.player_bank_has_suit(self.current_player_index, card.suit)
            }
            Some(PendingAbility::Hook) => {
                player_index == self.current_player_index
                    && self.is_top_card_of_suit_stack(player_index, card_index)
            }
            _ => false,
        }
    }

    pub fn can_select_map_choices(&self) -> bool {
        matches!(
            self.pending_selection,
            Some(PendingSelection {
                source: SelectionSource::MapChoices,
                ..
            })
        )
    }

    pub fn player_bank_has_suit(
        &self,
        player_index: usize,
        suit: crate::games::dead_mans_draw::card::Suit,
    ) -> bool {
        self.players[player_index]
            .bank
            .iter()
            .any(|card| card.suit == suit)
    }

    pub fn is_top_card_of_suit_stack(
        &self,
        player_index: usize,
        card_index: usize,
    ) -> bool {
        let Some(card) = self.players[player_index].bank.get(card_index) else {
            return false;
        };

        let highest_value = self.players[player_index]
            .bank
            .iter()
            .filter(|c| c.suit == card.suit)
            .map(|c| c.value)
            .max();

        highest_value == Some(card.value)
    }

    pub fn opponent_indices(&self) -> Vec<usize> {
        (0..self.players.len())
            .filter(|&index| index != self.current_player_index)
            .collect()
    }

    pub fn next_opponent_index(&self) -> Option<usize> {
        self.opponent_indices().into_iter().next()
    }
}