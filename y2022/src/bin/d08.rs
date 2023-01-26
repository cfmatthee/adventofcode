use std::fs;

fn main() {
    let data = fs::read_to_string("src/files/d08").unwrap();
    let data = data.split("\n").collect::<Vec<_>>();
    println!("{}", do_part_1(&data));
    println!("{}", do_part_2(&data));
}

#[derive(Debug, Default, Clone)]
struct Tree {
    height: u8,
    visible: bool,
    scenery: usize,
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Tree>,
    cols: usize,
    rows: usize,
}

fn build_forest(input: &[&str]) -> Forest {
    let mut trees: Vec<Tree> = vec![];
    let rows = input.len();
    let cols = input[0].len();
    for row in input {
        for col in row.chars() {
            let height: u8 = col.to_string().parse().unwrap();
            trees.push(Tree {
                height,
                visible: false,
                scenery: 0,
            });
        }
    }
    Forest { trees, cols, rows }
}

impl Forest {
    fn get(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn set(&mut self, pos: usize, value: bool) {
        self.trees[pos].visible = value;
    }

    fn set_visibility(&mut self) {
        // look at rows from left to right and right to left
        for r in 0..self.rows {
            let start = self.get(r, 0);
            let stop = self.get(r, self.cols);
            let idx: Vec<usize> = (start..stop).collect();
            self.do_set_visibility(idx);

            let idx = (start..stop).rev().collect();
            self.do_set_visibility(idx);
        }

        // look at cols from top to bottom and bottom to top
        for c in 0..self.cols {
            let start = self.get(0, c);
            let stop = self.get(self.rows, c);
            let idx: Vec<usize> = (start..stop).step_by(self.cols).collect();
            self.do_set_visibility(idx);

            let idx = (start..stop).step_by(self.cols).rev().collect();
            self.do_set_visibility(idx);
        }
    }

    fn do_set_visibility(&mut self, idx: Vec<usize>) {
        let mut max = self.trees[idx[0]].height;
        self.set(idx[0], true);
        for i in idx.iter().skip(1) {
            let cur = &self.trees[*i].height.clone();
            if cur > &max {
                self.set(*i, true);
                max = *cur;
            }
        }
    }

    fn get_scenery(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let row: Vec<usize> = (self.get(r, 0)..self.get(r, self.cols)).collect();
                let col: Vec<usize> = (self.get(0, c)..self.get(self.rows, c))
                    .step_by(self.cols)
                    .collect();
                let pos = self.get(r, c);
                let cur = &self.trees[pos];
                let left = &row[..c];
                let right = &row[(c + 1)..];
                let up = &col[..r];
                let down = &col[(r + 1)..];
                let left = left.iter().rev().map(|&i| i).collect::<Vec<_>>();
                let up = up.iter().rev().map(|&i| i).collect::<Vec<_>>();
                let mut scenery = self.calc_scenery(cur.height, left);
                scenery *= self.calc_scenery(cur.height, right.to_vec());
                scenery *= self.calc_scenery(cur.height, up);
                scenery *= self.calc_scenery(cur.height, down.to_vec());
                self.trees[pos].scenery = scenery;
            }
        }
    }

    fn calc_scenery(&self, height: u8, idx: Vec<usize>) -> usize {
        let mut score: usize = 0;
        for i in &idx {
            let cur = &self.trees[*i];
            score += 1;
            if cur.height >= height {
                break;
            }
        }
        score
    }
}

fn do_part_1(input: &[&str]) -> usize {
    let mut forest = build_forest(input);
    forest.set_visibility();
    forest.trees.iter().filter(|t| t.visible).count()
}

fn do_part_2(input: &[&str]) -> usize {
    let mut forest = build_forest(input);
    forest.get_scenery();
    forest.trees.iter().fold(0usize, |max, tree| {
        if tree.scenery > max {
            tree.scenery
        } else {
            max
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [&str; 5] = ["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn part1() {
        let result = do_part_1(&INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn part2() {
        let result = do_part_2(&INPUT);
        assert_eq!(result, 8);
    }
}
