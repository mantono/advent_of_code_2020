use std::collections::VecDeque;

pub fn first(input: String) -> String {
    let cards: Vec<u8> = input
        .lines()
        .filter_map(|d| d.trim().parse::<u8>().ok())
        .collect();
    let hand_size: usize = cards.len() / 2;
    let mut deck0: VecDeque<u8> = cards[..hand_size].iter().map(|c| *c).collect();
    let mut deck1: VecDeque<u8> = cards[hand_size..].iter().map(|c| *c).collect();
    let win_deck: VecDeque<u8> = play(deck0, deck1);
    score(win_deck).to_string()
}

fn play(mut deck0: VecDeque<u8>, mut deck1: VecDeque<u8>) -> VecDeque<u8> {
    if deck0.is_empty() {
        deck1
    } else if deck1.is_empty() {
        deck0
    } else {
        let card0: u8 = deck0.pop_front().unwrap();
        let card1: u8 = deck1.pop_front().unwrap();
        if card0 > card1 {
            deck0.push_back(card0);
            deck0.push_back(card1);
        } else if card1 > card0 {
            deck1.push_back(card1);
            deck1.push_back(card0);
        }
        play(deck0, deck1)
    }
}

fn score(deck: VecDeque<u8>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * (*card as usize))
        .sum()
}

pub fn second(input: String) -> String {
    input
}

#[cfg(test)]
mod tests {
    use super::first;

    #[test]
    fn test_part1() {
        let input = r"
            Player 1:
            9
            2
            6
            3
            1

            Player 2:
            5
            8
            4
            7
            10
        ";

        assert_eq!("306", &first(input.to_string()))
    }
}
