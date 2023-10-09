// 4.1 - Route Between Nodes
#![allow(dead_code)]

use std::collections::VecDeque;

struct AdjecencyMatrix {
    matrix: Vec<usize>,
    length: usize,
}

impl AdjecencyMatrix {
    fn new(length: usize) -> Self {
        AdjecencyMatrix {
            matrix: vec![0; length * length],
            length,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x * self.length + y
    }

    fn add_value(&mut self, x: usize, y: usize, value: usize) {
        let index = self.get_index(x, y);
        self.matrix[index] = value
    }

    fn get_row(&self, index: usize) -> &[usize] {
        let start = index * self.length;
        let end = start + self.length;
        &self.matrix[start..end]
    }

    fn bfs(&self, source: usize, target: usize) -> Option<Vec<usize>> {
        let mut prev = vec![None; self.length];
        let mut visited = vec![false; self.length];

        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(node) = queue.pop_front() {
            for (neighbour, &value) in self.get_row(node).iter().enumerate() {
                if !visited[neighbour] && value != 0 {
                    queue.push_back(neighbour);
                    visited[neighbour] = true;
                    prev[neighbour] = Some(node);
                }
            }
        }

        if !visited[target] {
            return None;
        }

        let mut path =
            std::iter::successors(Some(target), |&node| prev[node]).collect::<Vec<usize>>();
        path.reverse();

        Some(path)
    }

    fn has_route(&self, source: usize, target: usize) -> bool {
        self.bfs(source, target)
            .map_or(false, |path| path.iter().any(|&node| node == target))
    }

    fn print(&self) {
        for row in 0..self.length {
            let values = self.get_row(row);
            for &value in values {
                print!("{} ", value);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter4::q1::AdjecencyMatrix;

    #[test]
    fn should_find_route() {
        let length = 6;
        let mut matrix = AdjecencyMatrix::new(length);

        matrix.add_value(0, 1, 1);
        matrix.add_value(0, 2, 1);
        matrix.add_value(1, 3, 1);
        matrix.add_value(2, 4, 1);
        matrix.add_value(3, 5, 1);
        matrix.add_value(4, 5, 1);

        matrix.print();
        assert!(matrix.has_route(0, 5));
    }
}
