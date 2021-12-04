
pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        std::iter::s
        if self.row_count < 1 {
            vec![]
        } else {
            (1..self.row_count).fold(vec![vec![1]], |mut acc, _| {
                let prev = acc.last().unwrap();
                let mut temp = Vec::with_capacity(prev.len() +1);

                temp.push(*prev.first().unwrap());
                prev.windows(2).for_each(|w| temp.push(w.iter().sum()));
                temp.push(*prev.last().unwrap());

                acc.push(temp);
                acc
            })
        }
    }
}
