use super::game_state::GameState;
use super::rules::has_busted;

pub fn draw_card(state: &mut GameState) {
    if state.game_over {
        return;
    }

    state.revealed_next_card = None;

    let Some(card) = state.deck.pop() else {
        end_game(state);
        return;
    };

    state.play_area.push(card.clone());

    if has_busted(state) {
        state.discard.append(&mut state.play_area);
        state.message = format!("{} busted!", state.current_player().name);
        move_to_next_player_or_end(state);
        return;
    }

    let mut message = format!(
        "{} drew {:?} {}.",
        state.current_player().name,
        card.suit,
        card.value
    );

    if let Some(extra_message) = apply_card_ability(state, &card) {
        message.push(' ');
        message.push_str(&extra_message);
    }

    state.message = message;

    if state.deck.is_empty() {
        end_game(state);
    }
}

pub fn bank_cards(state: &mut GameState) {
    if state.game_over {
        return;
    }

    let cards = std::mem::take(&mut state.play_area);
    state.current_player_mut().bank.extend(cards);

    state.message = format!("{} banked cards.", state.current_player().name);

    if state.deck.is_empty() {
        state.game_over = true;
        state.message = "Deck is empty. Game over.".to_string();
    } else {
        state.next_player();
    }
}