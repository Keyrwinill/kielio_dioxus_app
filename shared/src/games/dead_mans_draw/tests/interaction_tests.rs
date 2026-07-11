use super::helpers::*;

use crate::games::dead_mans_draw::state::PendingAbility;

#[test]
fn map_replaying_cannon_opens_cannon_target_selection() {
    let mut state = GameState::empty();

    state.map_choices.push(Card {
        suit: Suit::Cannon,
        value: 5,
    });

    state.players[1].bank.push(Card {
        suit: Suit::Mermaid,
        value: 9,
    });

    state.phase = GamePhase::WaitingForMapTarget;
    state.pending_ability = Some(crate::games::dead_mans_draw::state::PendingAbility::Map);

    handle_action(
        &mut state,
        GameAction::SelectMapTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.phase, GamePhase::WaitingForCannonTarget);
    assert_eq!(
        state.pending_ability,
        Some(crate::games::dead_mans_draw::state::PendingAbility::Cannon)
    );
    assert!(state.pending_selection.is_some());
}

#[test]
fn hook_replaying_cannon_opens_cannon_target_selection() {
    let mut state = GameState::empty();

    state.players[0].bank.push(Card {
        suit: Suit::Cannon,
        value: 5,
    });

    state.players[1].bank.push(Card {
        suit: Suit::Mermaid,
        value: 9,
    });

    state.phase = GamePhase::WaitingForHookTarget;
    state.pending_ability = Some(crate::games::dead_mans_draw::state::PendingAbility::Hook);

    handle_action(
        &mut state,
        GameAction::SelectHookTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.phase, GamePhase::WaitingForCannonTarget);
    assert_eq!(
        state.pending_ability,
        Some(crate::games::dead_mans_draw::state::PendingAbility::Cannon)
    );
    assert!(state.pending_selection.is_some());
}

#[test]
fn hook_replays_cannon_then_cannon_discards_opponent_card() {
    let mut state = GameState::empty();

    state.players[0].bank.push(card(Suit::Cannon, 5));
    state.players[1].bank.push(card(Suit::Mermaid, 9));

    state.phase = GamePhase::WaitingForHookTarget;
    state.pending_ability = Some(crate::games::dead_mans_draw::state::PendingAbility::Hook);

    handle_action(
        &mut state,
        GameAction::SelectHookTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.phase, GamePhase::WaitingForCannonTarget);

    handle_action(
        &mut state,
        GameAction::SelectCannonTarget {
            target_player_index: 1,
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[1].bank.len(), 0);
    assert_eq!(state.discard.len(), 1);
    assert_eq!(state.discard[0].suit, Suit::Mermaid);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
}

#[test]
fn map_replays_cannon_then_cannon_discards_opponent_card() {
    let mut state = GameState::empty();

    state.map_choices.push(card(Suit::Cannon, 5));
    state.players[1].bank.push(card(Suit::Mermaid, 9));

    state.phase = GamePhase::WaitingForMapTarget;
    state.pending_ability = Some(PendingAbility::Map);

    handle_action(
        &mut state,
        GameAction::SelectMapTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.phase, GamePhase::WaitingForCannonTarget);

    handle_action(
        &mut state,
        GameAction::SelectCannonTarget {
            target_player_index: 1,
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[1].bank.len(), 0);
    assert_eq!(state.discard.len(), 1);
    assert_eq!(state.discard[0].suit, Suit::Mermaid);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
}

#[test]
fn hook_replays_anchor_and_sets_anchor_index() {
    let mut state = GameState::empty();

    setup_play_area(
        &mut state,
        vec![card(Suit::Oracle, 5), card(Suit::Hook, 4)],
        None,
    );

    state.players[0].bank.push(card(Suit::Anchor, 6));

    state.phase = GamePhase::WaitingForHookTarget;
    state.pending_ability = Some(PendingAbility::Hook);

    handle_action(
        &mut state,
        GameAction::SelectHookTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.anchor_index, Some(2));
    assert_eq!(state.play_area.len(), 3);
    assert_eq!(state.play_area[2].suit, Suit::Anchor);
}

#[test]
fn map_replays_anchor_and_sets_anchor_index() {
    let mut state = GameState::empty();

    setup_play_area(
        &mut state,
        vec![card(Suit::Oracle, 5), card(Suit::Map, 4)],
        None,
    );

    state.map_choices.push(card(Suit::Anchor, 6));

    state.phase = GamePhase::WaitingForMapTarget;
    state.pending_ability = Some(PendingAbility::Map);

    handle_action(
        &mut state,
        GameAction::SelectMapTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.anchor_index, Some(2));
    assert_eq!(state.play_area.len(), 3);
    assert_eq!(state.play_area[2].suit, Suit::Anchor);
}
