use crate::register_problem;
use std::cmp::Ordering;
use std::path::Path;

#[derive(Ord, Eq, Clone, Debug)]
struct Card {
    value: usize,
    color: char,
}

impl Card {
    fn new(value: usize, color: char) -> Card {
        Card { value, color }
    }

    fn from(s: &String) -> Card {
        let mut chars = s.chars();
        let c_value = chars.next().unwrap();
        let color = chars.next().unwrap();
        match c_value {
            '2' => Card::new(2, color),
            '3' => Card::new(3, color),
            '4' => Card::new(4, color),
            '5' => Card::new(5, color),
            '6' => Card::new(6, color),
            '7' => Card::new(7, color),
            '8' => Card::new(8, color),
            '9' => Card::new(9, color),
            'T' => Card::new(10, color),
            'J' => Card::new(11, color),
            'Q' => Card::new(12, color),
            'K' => Card::new(13, color),
            'A' => Card::new(14, color),
            _ => None.unwrap(),
        }
    }
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.color == other.color
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.value.cmp(&other.value);
        if cmp != Ordering::Equal {
            Some(cmp)
        } else {
            Some(self.color.cmp(&other.color))
        }
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {

    fn from(
        card1: &String,
        card2: &String,
        card3: &String,
        card4: &String,
        card5: &String,
    ) -> Hand {
        let mut hand = Vec::new();
        hand.push(Card::from(card1));
        hand.push(Card::from(card2));
        hand.push(Card::from(card3));
        hand.push(Card::from(card4));
        hand.push(Card::from(card5));
        hand.sort();
        Hand { cards: hand }
    }

    fn royal_flush(&self) -> usize {
        if self.straight_flush() == 14 {
            return 14;
        }
        0
    }

    fn straight_flush(&self) -> usize {
        if self.flush() != 0 {
            return self.straight();
        }
        0
    }

    fn flush(&self) -> usize {
        let card0 = &self.cards[0];
        let card1 = &self.cards[1];
        let card2 = &self.cards[2];
        let card3 = &self.cards[3];
        let card4 = &self.cards[4];
        if card0.color == card1.color
            && card0.color == card2.color
            && card0.color == card3.color
            && card0.color == card4.color
        {
            card4.value;
        }
        0
    }

    fn straight(&self) -> usize {
        let card0 = &self.cards[0];
        let card1 = &self.cards[1];
        let card2 = &self.cards[2];
        let card3 = &self.cards[3];
        let card4 = &self.cards[4];
        if card0.value + 1 == card1.value
            && card0.value + 2 == card2.value
            && card0.value + 3 == card3.value
            && card0.value + 4 == card4.value
        {
            return card4.value;
        }
        0
    }

    fn full_house(&self) -> usize {
        let card0 = &self.cards[0];
        let card1 = &self.cards[1];
        let card2 = &self.cards[2];
        let card3 = &self.cards[3];
        let card4 = &self.cards[4];
        if card0.value == card1.value
            && card0.value == card2.value
            && card3.value == card4.value {
            return card0.value;
        }
        if card0.value == card1.value
            && card2.value == card3.value
            && card2.value == card4.value {
            return card2.value;
        }
        0
    }

    fn carre(&self) -> usize {
        let card0 = &self.cards[0];
        let card1 = &self.cards[1];
        let card2 = &self.cards[2];
        let card3 = &self.cards[3];
        let card4 = &self.cards[4];
        if (card0.value == card1.value
            && card0.value == card2.value
            && card0.value == card3.value)
            || (card1.value == card2.value
                && card1.value == card3.value
                && card1.value == card4.value)
        {
            return card2.value;
        }
        0
    }

    fn brelan(&self) -> usize {
        let card0 = &self.cards[0];
        let card1 = &self.cards[1];
        let card2 = &self.cards[2];
        let card3 = &self.cards[3];
        let card4 = &self.cards[4];
        if (card0.value == card1.value && card0.value == card2.value)
            || (card1.value == card2.value && card1.value == card3.value)
            || (card2.value == card3.value && card2.value == card4.value)
        {
            return card2.value;
        }
        0
    }

    fn paire(&self) -> usize {
        let card0 = &self.cards[0];
        let card1 = &self.cards[1];
        let card2 = &self.cards[2];
        let card3 = &self.cards[3];
        let card4 = &self.cards[4];
        if card0.value == card1.value || card1.value == card2.value {
            return card1.value;
        }
        if card1.value == card2.value || card2.value == card3.value {
            return card2.value;
        }
        if card2.value == card3.value || card3.value == card4.value {
            return card3.value;
        }
        0
    }

    fn double_paire(&self) -> (usize, usize) {
        let card0 = &self.cards[0];
        let card1 = &self.cards[1];
        let card2 = &self.cards[2];
        let card3 = &self.cards[3];
        let card4 = &self.cards[4];
        if card0.value == card1.value &&
            (card2.value == card3.value || card3.value == card4.value) {
            return (card3.value, card1.value);
        }

        if card1.value == card2.value && card3.value == card4.value {
            return (card3.value, card1.value);
        }
        (0, 0)
    }

    fn cmp(&self, main: &Self) -> Ordering {
        let ordering = self.royal_flush().cmp(&main.royal_flush());
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = self.straight_flush().cmp(&main.straight_flush());
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = self.carre().cmp(&main.carre());
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = self.full_house().cmp(&main.full_house());
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = self.flush().cmp(&main.flush());
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = self.straight().cmp(&main.straight());
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = self.brelan().cmp(&main.brelan());
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = self.double_paire().cmp(&main.double_paire());
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = self.paire().cmp(&main.paire());
        if ordering != Ordering::Equal {
            return ordering;
        }

        let ordering = self.cards[4].value.cmp(&main.cards[4].value);
        if ordering != Ordering::Equal {
            return ordering;
        }
        let ordering = self.cards[3].value.cmp(&main.cards[3].value);
        if ordering != Ordering::Equal {
            return ordering;
        }
        let ordering = self.cards[2].value.cmp(&main.cards[2].value);
        if ordering != Ordering::Equal {
            return ordering;
        }
        let ordering = self.cards[1].value.cmp(&main.cards[1].value);
        if ordering != Ordering::Equal {
            return ordering;
        }
        self.cards[0].value.cmp(&main.cards[0].value)
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, main: &Self) -> Option<Ordering> {
        Some(self.cmp(main))
    }
}

register_problem!(54, "Poker hands", problem054);

pub fn problem054() -> String {
    // In the card game poker, a hand consists of five cards and are ranked, from lowest to highest,
    // in the following way:
    //
    //      High Card: Highest value card.
    //      One Pair: Two cards of the same value.
    //      Two Pairs: Two different pairs.
    //      Three of a Kind: Three cards of the same value.
    //      Straight: All cards are consecutive values.
    //      Flush: All cards of the same suit.
    //      Full House: Three of a kind and a pair.
    //      Four of a Kind: Four cards of the same value.
    //      Straight Flush: All cards are consecutive values of same suit.
    //      Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
    // The cards are valued in the order:
    //      2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
    //
    // If two players have the same ranked hands then the rank made up of the highest value wins;
    // for example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie,
    // for example, both players have a pair of queens, then highest cards in each hand are compared
    // (see example 4 below); if the highest cards tie then the next highest cards are compared, and so on.
    //
    // Consider the following five hands dealt to two players:
    //
    //  Hand	Player 1	 	    Player 2	 	    Winner
    //  1	    5H 5C 6S 7S KD      2C 3S 8S 8D TD      Player 2
    //          Pair of Fives       Pair of Eights
    //
    //  2	 	5D 8C 9S JS AC      2C 5C 7D 8S QH      Player 1
    //          Highest card Ace    Highest card Queen
    //
    //  3	    2D 9C AS AH AC      3D 6D 7D TD QD      Player 2
    //          Three Aces          Flush with Diamonds
    //
    //  4	    4D 6S 9H QH QC      3D 6D 7H QD QS      Player 1
    //          Pair of Queens      Pair of Queens
    //          Highest card Nine   Highest card Seven
    //
    // 5	 	2H 2D 4C 4D 4S      3C 3D 3S 9S 9D      Player 1
    //          Full House          Full House
    //          With Three Fours    with Three Threes
    //
    // The file, poker.txt, contains one-thousand random hands dealt to two players.
    // Each line of the file contains ten cards (separated by a single space): the first five are
    // Player 1's cards and the last five are Player 2's cards. You can assume that all hands are valid
    // (no invalid characters or repeated cards), each player's hand is in no specific order, and in each
    // hand there is a clear winner.
    //
    // How many hands does Player 1 win?
    let path = Path::new("data/p054_poker.txt");
    let lines: Vec<String> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut count = 0;

    for line in lines {
        let cards: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        let hand1 = Hand::from(&cards[0], &cards[1], &cards[2], &cards[3], &cards[4]);
        let hand2 = Hand::from(&cards[5], &cards[6], &cards[7], &cards[8], &cards[9]);
        if hand2 < hand1 {
            count += 1;
        }
    }

    count.to_string()
}
