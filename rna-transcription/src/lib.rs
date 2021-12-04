use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    seq: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let result = Dna {
            seq: String::new(),
        };

        dna.char_indices().try_fold(result, |mut acc, (i, ch)| {
            let a = HashMap::new()
            match ch {
                'G' | 'C' | 'T' | 'A' =>  acc.seq.push(ch),
                _ => return Err(i)
            }
            Ok(acc)
        })
    }

    pub fn into_rna(self) -> Rna {
        let result = Rna {
            seq: String::new(),
        };

        self.seq.chars().fold(result, |mut acc, ch| {
            match ch {
                'G' => acc.seq.push('C'),
                'C' => acc.seq.push('G'),
                'T' => acc.seq.push('A'),
                'A' => acc.seq.push('U'),
                _ => panic!("Shoulda never been here!")
            }

            acc
        })
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let result = Rna {
            seq: String::new(),
        };

        rna.char_indices().try_fold(result, |mut acc, (i, ch)| {
            match ch {
                'G' | 'C' | 'U' | 'A' =>  acc.seq.push(ch),
                _ => return Err(i)
            }
            Ok(acc)
        })
    }
}
