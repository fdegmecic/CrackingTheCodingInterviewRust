// 4.7 - Build Order
#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Debug,
    hash::Hash,
};

fn topological_sort<T, Id>(deps: T) -> Result<Vec<Id>, Box<dyn Error>>
where
    Id: Eq + Hash + Copy + Debug,
    State<Id>: From<T>,
{
    let mut res = Vec::new();
    let mut state = State::from(deps);

    while let Some(node) = state.no_deps.pop() {
        res.push(node);
        if let Some(dependents) = state.get_dependents(&node) {
            for dependent in dependents.clone() {
                state.resolve(&dependent, &node);
            }
        }
    }

    if !state.is_resolved() {
        return Err(format!(
            "cycle detected: {:?}",
            state.unresolved().collect::<Vec<_>>()
        )
        .into());
    }

    Ok(res)
}

type Graph<T> = HashMap<T, HashSet<T>>;

fn add_edge<T>(graph: &mut Graph<T>, from: T, to: Option<T>)
where
    T: Eq + Hash + Copy,
{
    graph
        .entry(from)
        .and_modify(|pointees| {
            if let Some(node) = to {
                pointees.insert(node);
            }
        })
        .or_insert_with(|| {
            let mut s = HashSet::new();
            if let Some(node) = to {
                s.insert(node);
            }
            s
        });
}

struct State<T> {
    depends_on: Graph<T>,
    dependents: Graph<T>,
    no_deps: Vec<T>,
}

impl<T> Default for State<T> {
    fn default() -> Self {
        Self {
            depends_on: HashMap::new(),
            dependents: HashMap::new(),
            no_deps: vec![],
        }
    }
}

impl<T> State<T>
where
    T: Eq + Hash,
{
    fn get_dependents(self: &Self, dependency: &T) -> Option<&HashSet<T>> {
        self.dependents.get(dependency)
    }

    fn is_resolved(self: &Self) -> bool {
        self.depends_on.is_empty()
    }
}

impl<T> State<T>
where
    T: Copy + Eq + Hash,
{
    fn resolve(self: &mut Self, dependent: &T, dependency: &T) {
        if let Some(dependencies) = self.depends_on.get_mut(dependent) {
            dependencies.remove(&dependency);

            if dependencies.is_empty() {
                self.no_deps.push(*dependent);

                self.depends_on.remove(dependent);
            }
        }
    }

    fn unresolved(self: &Self) -> impl Iterator<Item = &T> {
        self.depends_on.keys()
    }
}

impl<T> From<&Graph<T>> for State<T>
where
    T: Eq + Hash + Copy,
{
    fn from(graph: &Graph<T>) -> Self {
        let mut state = State::default();

        for (&node, dependencies) in graph.iter() {
            if dependencies.is_empty() {
                state.no_deps.push(node);
            } else {
                for &dependency in dependencies {
                    add_edge(&mut state.depends_on, node, Some(dependency));
                    add_edge(&mut state.dependents, dependency, Some(node));
                }
            }
        }

        state
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter4::q7::*;

    #[test]
    fn should_topological_sort() {
        let mut graph: Graph<char> = HashMap::new();
        add_edge(&mut graph, 'f', Some('c'));
        add_edge(&mut graph, 'f', Some('b'));
        add_edge(&mut graph, 'f', Some('a'));
        add_edge(&mut graph, 'c', Some('a'));
        add_edge(&mut graph, 'b', Some('a'));
        add_edge(&mut graph, 'b', Some('e'));
        add_edge(&mut graph, 'a', Some('e'));
        add_edge(&mut graph, 'd', Some('g'));
        add_edge(&mut graph, 'e', None);
        add_edge(&mut graph, 'g', None);

        let result = topological_sort(&graph).unwrap();
        let sorted = result.iter().rev().collect::<Vec<&char>>();

        assert_eq!(
            "['f', 'c', 'b', 'a', 'e', 'd', 'g']",
            format!("{:?}", sorted)
        );
    }
}
