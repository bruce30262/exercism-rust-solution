
// Bowling scoring: http://www.fryes4fun.com/Bowling/scoring.htm

#[derive(Debug, PartialEq, Eq)]                                                                                                                                                                                 
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    round: usize,
    pins: u16,
    end: bool,
    frame_score: [u16; 21],
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { round: 0, pins:10, end:false, frame_score: [100; 21] }    
    }   

    fn is_spare(&self, cur_round: usize) -> bool {
        if cur_round & 1 == 1 && self.frame_score[ cur_round - 1 ] + self.frame_score[ cur_round ] == 10 {
            true
        } else {
            false
        }
    }
    
    fn is_strike(&self, cur_round: usize) -> bool {
        if (cur_round & 1 == 0 || cur_round == 19) && self.frame_score[ cur_round ] == 10 {
            true
        } else {
            false
        }
    }
    
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.end {
            return Err(Error::GameComplete);
        }
        
        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        
        let mut refill:bool = false;
        self.frame_score[self.round] = pins;
        self.pins -= pins;
        
        if ( self.round == 19 && self.frame_score[18] != 10 && !self.is_spare(self.round) ) || self.round == 20 {
            self.end = true;
        } else if self.is_strike(self.round) {
            self.round += if self.round < 18 { 2 } else { 1 };
            refill = true;
        } else if self.is_spare(self.round) {
            self.round += 1;
            refill = true;
        } else {
            self.round += 1
        }

        if (self.round <= 18 && self.round & 1 == 0) || refill {
            self.pins = 10;
        }
        
        Ok(())
    }   

    fn get_next_two(&self, mut cur_round: usize) -> u16 {
        let mut ret:u16 = 0;
        for _ in 0..2 {
            while cur_round < 21 {
                if self.frame_score[cur_round] != 100 {
                    ret += self.frame_score[cur_round];
                    break;
                }
                cur_round += 1;
            }
            cur_round += 1;
        }
        ret
    }
    
    pub fn score(&self) -> Option<u16> {
        if !self.end || self.round == 0 { return None; }
        
        let mut cur_round: usize = 0;
        let mut total: u16 = 0;
        while cur_round < 18 { // first 9 frame
            total += self.frame_score[cur_round];
            if self.is_strike(cur_round) {
                total += self.get_next_two(cur_round + 2);
                cur_round += 1;
            } else if self.is_spare(cur_round) {
                total += self.frame_score[cur_round + 1];
            }
            cur_round += 1;
        }
        // add score in 10th frame
        for i in 18..21 {
            total += if self.frame_score[i] != 100 { self.frame_score[i] } else { 0 };
        }
        Some(total)
    }   
}
