use std::collections::{HashMap};
use std::io::{BufRead, BufReader};

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

fn cardvalue(card: char) -> u32 {
    "23456789TJQKA".find(card).expect("bad card value") as u32 + 2
}

#[derive(Debug,PartialOrd,PartialEq)]
enum HandType {
    HighCard,//(u32),
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug)]
struct Hand(String);
impl Hand {
    fn count_cards(&self) -> HandType {
        let mut counts = HashMap::<char, u32>::new();
        for card in self.0.chars() {
            counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        match counts.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                for (_,count) in counts.iter() {
                    if *count == 2 {
                        return HandType::TwoPair;
                    }
                }
                HandType::ThreeKind
            }
            2 => { 
                match counts.iter().nth(0).unwrap().1 {
                    4 | 1 => HandType::FourKind,
                    _ => HandType::FullHouse,
                }
            },
            1 => HandType::FiveKind,
            _ => panic!("couldn't count cards in {:?}", self),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, _other: &Self) -> bool {
        panic!("Shouldn't compare hands directly");
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = self.count_cards();
        let right = other.count_cards();
        if left == right {
            // actually.... HighCard doesn't win on the value of the HighCard.
            // if let (HandType::HighCard(lvalue),HandType::HighCard(rvalue)) = (left,right) {
            //     if lvalue != rvalue {
            //         return lvalue.partial_cmp(&rvalue);
            //     }
            // }

            // all else equal, compare card values in order
            return Some(self.0.chars().map(cardvalue).cmp(other.0.chars().map(cardvalue)));
        } else {
            return left.partial_cmp(&right);
        }
    }
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // camel cards poker
    // puzzle input, list of hands
    // output is ordered list of hands by strength

    let mut lines = BufReader::new(input).lines();
    let mut hands = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        //32T3K 765
        let (hand,bid_str) = line.split_once(' ').unwrap();
        let bid = bid_str.parse::<u32>().unwrap();
        hands.push((Hand(hand.to_owned()),bid));
    }

    hands.sort_by(|(left,_),(right,_)| {
        let ord = left.partial_cmp(&right).unwrap();
        if DEBUG { eprintln!("{left:?} {:?} {right:?}", ord) };
        ord
    });
    if DEBUG { eprintln!("hands: {:?}", &hands) };
    
    let mut winnings = 0;
    for (i,(hand,bid)) in hands.iter().enumerate() {
        winnings += (i+1) as u32 *bid;
        if DEBUG { eprintln!("{i} {:?}{:?} => {}", hand, hand.count_cards(), (i+1)as u32*bid) };
    }

    println!("{winnings}");

    // PART TWO. The seed numbers are ranges, with the start and length in each pair
    eprintln!("PART TWO");

    return Ok(());
}

fn main() -> Result<(),Error> {
    go(&mut std::io::stdin().lock())
}

#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    go(&mut testinput.as_bytes())
}

#[test]
fn test1() {
    let testhand = Hand("32T3K".to_owned());
    assert_eq!(testhand.count_cards(), HandType::OnePair);

    let testhand = Hand("T55J5".to_owned());
    assert_eq!(testhand.count_cards(), HandType::ThreeKind);
}

