use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Record {
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

impl Record {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn get_mp(&self) -> u32 {
        self.w + self.d + self.l
    }

    pub fn update_score(&mut self, score: u32) {
        self.p += score;
    }

    pub fn update_wdl(&mut self, res: &char) {
        match res {
            'W' => self.w += 1,
            'D' => self.d += 1,
            _ => self.l += 1,
        }
    }
}

// watch out the lifetime of the variables
fn update_score<'a>(table: &mut HashMap<&'a str, Record>, win_team: &'a str, lose_team: &'a str, score:u32) {
    let (win_score, win_record) = if score == 3 { (3, 'W') } else { (1, 'D') };
    table.entry(win_team).or_insert(Record::new());
    table.get_mut(win_team).map(|val| val.update_wdl(&win_record)); // use get_mut so value is mutable
    table.get_mut(win_team).map(|val| val.update_score(win_score));
    
    let (lose_score, lose_record) = if score == 3 { (0, 'L') } else { (1, 'D') };
    table.entry(lose_team).or_insert(Record::new());
    table.get_mut(lose_team).map(|val| val.update_wdl(&lose_record));
    table.get_mut(lose_team).map(|val| val.update_score(lose_score));
}

pub fn tally(match_results: &str) -> String {
    let mut table: HashMap<&str, Record> = HashMap::new();
    let matches = match_results.split("\n");
    
    for m in matches {
        if m.len() > 0 {
            let mut iter = m.split(';');
            let t1 = iter.next().unwrap();
            let t2 = iter.next().unwrap();
            let res = iter.next().unwrap();

            match res {
                "win" => update_score(&mut table, t1, t2, 3), 
                "loss" => update_score(&mut table, t2, t1, 3), 
                _ => update_score(&mut table, t1, t2, 1),
            }
        }
    }

    let mut table_vec: Vec<(&&str, &Record)> = table.iter().collect();
    table_vec.sort_by(|a, b| b.1.p.cmp(&a.1.p).then(a.0.cmp(&b.0))); // first record.p, then team name

    let mut result = "Team                           | MP |  W |  D |  L |  P".to_owned();
    for (team, record) in table_vec {
        result += &format!("\n{: <31}| {: >2} |  {} |  {} |  {} |  {}", team, record.get_mp(), record.w, record.d, record.l, record.p);
    }

    result
}

