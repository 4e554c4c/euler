use std::vec::Vec;

/// Iterator returning the number of numbers over `limit` on pascal's triangle
/// at each iteration
struct Pascal {
    curr_row: Vec<usize>,
    len: usize,
    limit: usize,
    truncated: bool,
}

impl Pascal {
    fn new(limit: usize) -> Pascal {
        Pascal {
            curr_row: vec![1],
            len: 1,
            limit: limit,
            truncated: false,
        }
    }
}

impl Iterator for Pascal {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        Some({
            self.len += 1;
            let len = self.curr_row.len();
            let mut last = self.curr_row[0];
            let mut curr;
            let mut truncate = None;
            for i in 1..len {
                curr = self.curr_row[i];
                self.curr_row[i] += last;
                last = curr;

                if self.curr_row[i] > self.limit {
                    truncate = Some(i);
                    break;
                }
            }

            if let Some(i) = truncate {
                self.curr_row.truncate(i);
                self.truncated = true;
            } else if !self.truncated && 
                    self.len % 2 != 0 {
                if last * 2 > self.limit {
                    self.truncated = true;
                } else {
                    self.curr_row.push(last * 2);
                }
            }

            // Return the amount over
            if !self.truncated {
                0
            } else {
                self.len - self.curr_row.len() * 2
            }
        })
    }
}

fn main() {
    println!("The answer is: {}", Pascal::new(1_000_000)
             .take(100)
             .sum::<usize>());
}
