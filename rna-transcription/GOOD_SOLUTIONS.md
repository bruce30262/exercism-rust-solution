* [emiliolg](https://exercism.org/tracks/rust/exercises/rna-transcription/solutions/emiliolg)  

```rust
#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: String
}
#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: String
}
const RNA: [char; 4] = [ 'C', 'G', 'A', 'U' ];
const DNA: [char; 4] = [ 'G', 'C', 'T', 'A' ];
fn validate(s: &str, chars: [char; 4]) -> Result<String, usize> {
    match s.chars().position(|c| !chars.contains(&c)) {
        Some(x) => Err(x),
        None => Ok(s.to_string())
    }
}
fn transcribe(nucleotide: char) -> char {
    RNA[DNA.iter().position(|&c| c == nucleotide).unwrap()]
}
impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate(dna, DNA).map(|nucleotides| Dna { nucleotides })
    }
    pub fn into_rna(self) -> Rna {
        Rna { nucleotides: self.nucleotides.chars().map(|c| transcribe(c)).collect() }
    }
}
impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate(rna, RNA).map(|nucleotides| Rna { nucleotides })
    }
}
```

* [slowbyrne](https://exercism.org/tracks/rust/exercises/rna-transcription/solutions/slowbyrne)  

```rust
#[derive(Debug, PartialEq)]
pub struct Dna(String);
#[derive(Debug, PartialEq)]
pub struct Rna(String);
impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        Ok(Dna(validate(dna, "GCTA")?))
    }
    pub fn into_rna(self) -> Rna {
        let transform = |c: char| match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => c,
        };
        Rna(self.0.chars().map(transform).collect())
    }
}
impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        Ok(Rna(validate(rna, "CGAU")?))
    }
}
fn validate(seq: &str, valid_chars: &str) -> Result<String, usize> {
    match seq.chars().position(|c| !valid_chars.contains(c)) {
        Some(i) => Err(i),
        None => Ok(seq.to_owned()),
    }
}
```
