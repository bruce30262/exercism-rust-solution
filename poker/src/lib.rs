use std::collections::BTreeMap;
use std::collections::HashSet;
use std::cmp::{max, Ordering};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Hand {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfKind,
    Straight,
    Flush,
    FullHouse,
    FourOfKind,
    StraightFlush,
}

#[derive(Debug)]
pub struct Poker {
    nums: BTreeMap<u8, u8>,
    hand: Hand,
    main_num: u8,
}

impl Poker {
    fn to_num(s: &str) -> u8 {
        match s {
            "A" => 1,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            _ => panic!(),
        }
    }

    fn is_straight(nums: &BTreeMap<u8, u8>) -> bool {
        if nums.keys().len() != 5 { return false }
        let num_vec = nums.keys().cloned().collect::<Vec<u8>>();
        // check last 4
        for i in 1..4 {
            if num_vec[i] + 1 != num_vec[i+1] { 
                return false;
            }
        }
        // check 1st & 2nd
        if num_vec[0] + 1 != num_vec[1] {
            if num_vec[0] == 1 && num_vec[1] == 10 { return true; } // 10, J, Q, K, A
            return false;
        }
        true
    }

    fn cal_hand(shapes: &HashSet<char>, nums: &BTreeMap<u8, u8>) -> (u8 ,Hand) {
        let main_num:u8;
        let hand: Hand;

        let (card_num, card_num_max_cnt) = nums.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap(); // get max count in nums
        // Detect FourOfKind / ThreeOfKind / TwoPair / OnePair
        if *card_num_max_cnt == 4 {
            main_num = *card_num;
            hand = Hand::FourOfKind;
        } else if *card_num_max_cnt == 3 {
            main_num = *card_num;
            match nums.keys().len() {
                2 => hand = Hand::FullHouse,
                _ => hand = Hand::ThreeOfKind,
            }
        } else if *card_num_max_cnt == 2 {
            let mut max_num:u8 = 0;
            for (n, _cnt) in nums.iter().filter(|&(_, v)| *v == 2) {
                if *n == 1 {
                    max_num = 1;
                    break;
                } else {
                    max_num = max(max_num, *n);
                }
            }
            main_num = max_num;
            match nums.keys().len() {
                3 => hand = Hand::TwoPair,
                _ => hand = Hand::OnePair,
            }
        } else { 
            // *card_num_max_cnt == 1
            // Straight Flush, Flush, Straight, HighCard
            let tmp_min = nums.iter().next().unwrap().0; // get min value in nums
            let tmp_max = nums.iter().next_back().unwrap().0; // get max value in nums
            if Self::is_straight(nums) {
                if *tmp_min == 1 && *tmp_max == 13 {
                    main_num = 1;
                } else {
                    main_num = *tmp_max;
                }
                if shapes.len() == 1 {
                    hand = Hand::StraightFlush;
                } else {
                    hand = Hand::Straight;
                }
            } else {
                if *tmp_min == 1 {
                    main_num = 1;
                } else {
                    main_num = *tmp_max;
                }
                if shapes.len() == 1 {
                    hand = Hand::Flush;
                } else {
                    hand = Hand::HighCard;
                }
            }
        }
        (main_num, hand)
    }

    pub fn new(poker: &str) -> Self {
        let mut shapes = HashSet::new();
        let mut nums = BTreeMap::new();
        for card in poker.split(' ') {
            let shape: char = card.chars().last().unwrap();
            let num: u8 = match (card[0..card.len()-1]).parse::<u8>() {
                Ok(n) => n,
                Err(_) => Self::to_num(&card[0..card.len()-1]),// A, J, Q, K
            };
            shapes.insert(shape);
            nums.entry(num).and_modify(|n| *n += 1).or_insert(1);
        }
        let (main_num, hand) = Self::cal_hand(&shapes, &nums);
        Self{ 
            nums,
            hand,
            main_num,
        }
    }
}

impl PartialEq for Poker {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.nums == other.nums
    }
}

impl Eq for Poker {}

fn rank(n: u8) -> u8 {
    match n {
        1 => 14,
        _ => n,
    }
}

fn rank_map(old_map: &BTreeMap<u8, u8>) -> BTreeMap<u8, u8> {
    let mut ret: BTreeMap<u8, u8> = old_map.clone();
    if ret.contains_key(&1) {
        let v = ret.remove(&1); // remove key 1 and move old value to v
        ret.insert(14, v.unwrap());
    }
    ret
}

impl PartialOrd for Poker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Poker {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand == other.hand {
            if self.main_num == other.main_num {
                let self_rank_map = rank_map(&self.nums);
                let other_rank_map = rank_map(&other.nums);
                match self.hand {
                    Hand::TwoPair=> {
                        let self_2nd_pair = self_rank_map.iter().find(|&(k, v)| *k != self.main_num && *v == 2).unwrap().0;
                        let other_2nd_pair = other_rank_map.iter().find(|&(k, v)| *k != other.main_num && *v == 2).unwrap().0;
                        if self_2nd_pair == other_2nd_pair { // find max
                            let self_max = self_rank_map.iter().find(|&(_, v)| *v == 1).unwrap().0;
                            let other_max = other_rank_map.iter().find(|&(_, v)| *v == 1).unwrap().0;
                            self_max.cmp(other_max)

                        } else { // cmp 2nd pair
                            self_2nd_pair.cmp(other_2nd_pair)
                        }
                    }
                    Hand::ThreeOfKind | Hand::OnePair => { // find max
                        let self_max = self_rank_map.keys().filter(|&x| *x != self.main_num).max().unwrap();
                        let other_max = other_rank_map.keys().filter(|&x| *x != other.main_num).max().unwrap();
                        self_max.cmp(other_max)
                    }
                    _ => self_rank_map.cmp(&other_rank_map), // can cmp BTreeMap directly
                }
            } else { // cmp main_num
                rank(self.main_num).cmp(&(rank(other.main_num)))
            }
        } else { // cmp hands
            let (self_hand, other_hand) = (self.hand as u8, other.hand as u8);
            self_hand.cmp(&other_hand)
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut pv: Vec<Poker> = Vec::new();
    for poker in hands {
        pv.push(Poker::new(poker));
    }
    let mut res: Vec<&str> = Vec::new();
    for (idx, poker) in pv.iter().enumerate() {
        if *poker == *(pv.iter().max().unwrap()) { res.push(hands[idx]); }
    }
    res
}
