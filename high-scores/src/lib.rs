// Highest score
// Last added score
// Top 3 highest scores

#[derive(Debug)]
pub struct HighScores<'a> {
    highest: &'a u32,
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            highest: scores.iter().max().unwrap_or(&u32::MIN),
            scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.highest == &u32::MIN {
            None
        } else {
            Some(*self.highest)
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort_by(|a,b| b.cmp(a));
        scores.truncate(3);
        scores
    }
}
