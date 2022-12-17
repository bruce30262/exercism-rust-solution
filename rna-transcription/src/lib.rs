#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (idx, c) in dna.chars().enumerate() {
            if !"ACGT".contains(c) { return Err(idx); }
        }
        Ok(Dna(dna.to_owned()))
    }

    pub fn into_rna(self) -> Rna {
        let mut rna:Vec<char> = vec![];
        for c in self.0.chars() {
            match c {
                'A' => rna.push('U'),
                'C' => rna.push('G'),
                'G' => rna.push('C'),
                'T' => rna.push('A'),
                _ => panic!(),
            }
        }
        Rna::new(&rna.iter().collect::<String>()).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (idx, c) in rna.chars().enumerate() {
            if !"ACGU".contains(c) { return Err(idx); }
        }
        Ok(Rna(rna.to_owned()))
    }
}
