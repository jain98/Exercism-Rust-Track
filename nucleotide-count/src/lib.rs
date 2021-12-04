use std::collections::{HashMap, HashSet};

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let allowed = "ACGT".chars().collect::<HashSet<char>>();

    if !allowed.contains(&nucleotide) {
        return Err(nucleotide);
    }

    dna.chars().try_fold(0, |acc, val| {
        if !allowed.contains(&val) {
            Err(val)
        } else {
            let inc = if val == nucleotide { 1 } else { 0 };
            Ok(acc+inc)
        }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let result = "ACGT".chars().map(|c| (c, 0)).collect::<HashMap<char, usize>>();

    "ACGT".chars().try_fold(result, |mut acc, val| {
        let inc = count(val, dna)?;
        acc.entry(val).and_modify(|v| *v += inc);
        Ok(acc)
    })
}

