use super::helpers::*;

#[test]
fn can_select_bank_card_respects_cannon_and_sword_rules() {
    let mut state = GameState::empty();

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.players[1].bank.push(Card {
        suit: Suit::Anchor,
        value: 3,
    });

    state.players[1].bank.push(Card {
        suit: Suit::Anchor,
        value: 7,
    });

    // Cannon can target opponent's top Anchor only.
    state.pending_ability = Some(crate::games::dead_mans_draw::state::PendingAbility::Cannon);
    state.pending_selection = Some(crate::games::dead_mans_draw::state::PendingSelection {
        source: crate::games::dead_mans_draw::state::SelectionSource::PlayerBank {
            owner: crate::games::dead_mans_draw::state::SelectionOwner::Opponent,
        },
        prompt: "Choose target.".to_string(),
    });

    assert!(!state.can_select_bank_card(1, 0));
    assert!(state.can_select_bank_card(1, 1));

    // Sword cannot target Anchor because current player already has Anchor.
    state.pending_ability = Some(crate::games::dead_mans_draw::state::PendingAbility::Sword);

    assert!(!state.can_select_bank_card(1, 1));
}

#[test]
fn scoring_uses_highest_card_per_suit() {
    let mut state = GameState::empty();

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 7,
    });

    state.players[0].bank.push(Card {
        suit: Suit::Oracle,
        value: 5,
    });

    let score = crate::games::dead_mans_draw::scoring::score_player(&state, 0);

    assert_eq!(score, 12);
}

#[test]
fn winner_is_player_with_highest_score_when_game_over() {
    let mut state = GameState::empty();

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.players[1].bank.push(Card {
        suit: Suit::Mermaid,
        value: 9,
    });

    state.game_over = true;

    let winner = crate::games::dead_mans_draw::scoring::winner_index(&state);

    assert_eq!(winner, Some(1));
}

#[test]
fn duplicate_suit_in_play_area_causes_bust() {
    let mut state = GameState::empty();

    state.play_area.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.deck.clear();

    // This is drawn first because deck.pop() takes from the end.
    state.deck.push(Card {
        suit: Suit::Oracle,
        value: 5,
    });

    // This is the actual duplicate card drawn.
    state.deck.push(Card {
        suit: Suit::Anchor,
        value: 5,
    });

    handle_action(&mut state, GameAction::Draw);

    assert_eq!(state.play_area.len(), 0);
    assert_eq!(state.discard.len(), 2);
    assert_eq!(state.current_player_index, 1);
    assert!(!state.game_over);
}

#[test]
fn bust_card_does_not_enter_play_area_or_activate_ability() {
    let mut state = GameState::empty();

    state.play_area.push(Card {
        suit: Suit::Cannon,
        value: 2,
    });

    state.deck.clear();

    state.deck.push(Card {
        suit: Suit::Cannon,
        value: 7,
    });

    handle_action(&mut state, GameAction::Draw);

    assert_eq!(state.play_area.len(), 0);
    assert_eq!(state.discard.len(), 2);
    assert!(
        state
            .discard
            .iter()
            .any(|c| c.suit == Suit::Cannon && c.value == 2)
    );
    assert!(
        state
            .discard
            .iter()
            .any(|c| c.suit == Suit::Cannon && c.value == 7)
    );
}
