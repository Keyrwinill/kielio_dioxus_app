use super::helpers::*;

#[test]
fn banking_chest_without_key_gets_no_bonus() {
    let mut state = GameState::new();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Chest, 3),
            card(Suit::Map, 5),
        ],
        None,
    );

    state.discard.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Bank);

    assert_eq!(state.players[0].bank.len(), 2);
    assert_eq!(state.discard.len(), 1);
}

#[test]
fn banking_key_without_chest_gets_no_bonus() {
    let mut state = GameState::new();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Key, 3),
            card(Suit::Map, 5),
        ],
        None,
    );

    state.discard.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Bank);

    assert_eq!(state.players[0].bank.len(), 2);
    assert_eq!(state.discard.len(), 1);
}

#[test]
fn banking_chest_and_key_claims_bonus_from_discard() {
    let mut state = GameState::new();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Chest, 3),
            card(Suit::Key, 4),
            card(Suit::Map, 5),
        ],
        None,
    );

    state.discard.push(card(Suit::Mermaid, 9));
    state.discard.push(card(Suit::Oracle, 6));
    state.discard.push(card(Suit::Sword, 7));

    handle_action(&mut state, GameAction::Bank);

    // 3 collected + 3 bonus
    assert_eq!(state.players[0].bank.len(), 6);
    assert_eq!(state.discard.len(), 0);
}

#[test]
fn chest_and_key_claims_only_available_discard_cards() {
    let mut state = GameState::new();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Chest, 3),
            card(Suit::Key, 4),
            card(Suit::Map, 5),
        ],
        None,
    );

    state.discard.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Bank);

    // 3 collected + only 1 available bonus
    assert_eq!(state.players[0].bank.len(), 4);
    assert_eq!(state.discard.len(), 0);
}

#[test]
fn chest_and_key_do_not_claim_bonus_when_saved_by_anchor_on_bust() {
    let mut state = GameState::new();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Key, 3),
            card(Suit::Chest, 4),
            card(Suit::Anchor, 5),
        ],
        Some(2),
    );

    state.discard.push(card(Suit::Mermaid, 9));

    state.deck.clear();
    state.deck.push(card(Suit::Anchor, 6));

    handle_action(&mut state, GameAction::Draw);

    assert_eq!(state.players[0].bank.len(), 2);
    assert_eq!(state.discard.len(), 3); // Mermaid + Anchor 5 + Anchor 6
}