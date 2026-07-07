use super::helpers::*;

use crate::games::dead_mans_draw::{
    state::PendingAbility,
    player::Player,
    state::{GameConfig, GamePhase, GameState},
};

#[test]
fn drawing_last_card_does_not_end_game_before_banking() {
    let mut state = GameState::empty();

    state.deck.clear();
    state.deck.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert_eq!(state.play_area.len(), 1);

    handle_action(&mut state, GameAction::Bank);

    assert!(state.game_over);
    assert_eq!(state.players[0].bank.len(), 1);
}

#[test]
fn drawing_last_card_that_busts_ends_game_after_bust_resolution() {
    let mut state = GameState::empty();

    setup_play_area(
        &mut state,
        vec![card(Suit::Cannon, 3)],
        None,
    );

    state.deck.clear();
    state.deck.push(card(Suit::Cannon, 8));

    handle_action(&mut state, GameAction::Draw);

    assert!(state.game_over);
    assert_eq!(state.phase, GamePhase::GameOver);

    assert!(state.play_area.is_empty());
    assert_eq!(state.discard.len(), 2);
    assert!(state.discard.iter().any(|c| c.suit == Suit::Cannon && c.value == 3));
    assert!(state.discard.iter().any(|c| c.suit == Suit::Cannon && c.value == 8));
}

#[test]
fn drawing_last_map_card_waits_for_map_selection_before_game_over() {
    let mut state = GameState::empty();

    state.discard.push(card(Suit::Oracle, 5));

    state.deck.clear();
    state.deck.push(card(Suit::Map, 7));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::WaitingForMapTarget);
    assert_eq!(state.pending_ability, Some(PendingAbility::Map));
}

#[test]
fn drawing_last_hook_card_waits_for_hook_selection_before_game_over() {
    let mut state = GameState::empty();

    state.players[0].bank.push(card(Suit::Oracle, 5));

    state.deck.clear();
    state.deck.push(card(Suit::Hook, 7));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::WaitingForHookTarget);
    assert_eq!(state.pending_ability, Some(PendingAbility::Hook));
}

#[test]
fn drawing_last_cannon_card_waits_for_cannon_selection_before_game_over() {
    let mut state = GameState::empty();

    state.players[1].bank.push(card(Suit::Mermaid, 9));

    state.deck.clear();
    state.deck.push(card(Suit::Cannon, 7));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::WaitingForCannonTarget);
    assert_eq!(state.pending_ability, Some(PendingAbility::Cannon));
}

#[test]
fn drawing_last_sword_card_waits_for_sword_selection_before_game_over() {
    let mut state = GameState::empty();

    state.players[1].bank.push(card(Suit::Mermaid, 9));

    state.deck.clear();
    state.deck.push(card(Suit::Sword, 7));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::WaitingForSwordTarget);
    assert_eq!(state.pending_ability, Some(PendingAbility::Sword));
}

#[test]
fn new_game_uses_default_two_players() {
    let state = GameState::empty();

    assert_eq!(state.players.len(), 2);
    assert_eq!(state.players[0].name, "You");
    assert_eq!(state.players[1].name, "AI");
}

#[test]
fn new_game_with_config_supports_custom_players() {
    let state = GameState::new_with_config(GameConfig {
        players: vec![
            Player::new("Player 1", false),
            Player::new("AI 1", true),
            Player::new("AI 2", true),
        ],
    });

    assert_eq!(state.players.len(), 3);
    assert_eq!(state.players[0].name, "Player 1");
    assert_eq!(state.players[1].name, "AI 1");
    assert_eq!(state.players[2].name, "AI 2");
}

#[test]
#[should_panic(expected = "Dead Man's Draw requires 2 to 4 players.")]
fn new_game_with_config_rejects_one_player() {
    GameState::new_with_config(GameConfig {
        players: vec![Player::new("Solo", false)],
    });
}

#[test]
#[should_panic(expected = "Dead Man's Draw requires 2 to 4 players.")]
fn new_game_with_config_rejects_five_players() {
    GameState::new_with_config(GameConfig {
        players: vec![
            Player::new("P1", false),
            Player::new("P2", true),
            Player::new("P3", true),
            Player::new("P4", true),
            Player::new("P5", true),
        ],
    });
}

#[test]
fn next_player_rotates_through_three_players() {
    let mut state = GameState::new_with_config(GameConfig {
        players: vec![
            Player::new("P1", false),
            Player::new("P2", true),
            Player::new("P3", true),
        ],
    });

    assert_eq!(state.current_player_index, 0);

    state.next_player();
    assert_eq!(state.current_player_index, 1);

    state.next_player();
    assert_eq!(state.current_player_index, 2);

    state.next_player();
    assert_eq!(state.current_player_index, 0);
}

#[test]
fn start_new_game_action_uses_configured_players() {
    let mut state = GameState::empty();

    handle_action(
        &mut state,
        GameAction::StartNewGame {
            config: GameConfig {
                players: vec![
                    Player::new("You", false),
                    Player::new("Bob", false),
                    Player::new("AI 1", true),
                ],
            },
        },
    );

    assert_eq!(state.players.len(), 3);
    assert_eq!(state.players[0].name, "You");
    assert_eq!(state.players[1].name, "Bob");
    assert_eq!(state.players[2].name, "AI 1");
    assert!(!state.players[1].is_ai);
    assert!(state.players[2].is_ai);
}