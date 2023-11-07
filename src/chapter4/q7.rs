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

fn add_edge<T>(graph: &mut Graph<T>, from: T, to: T)
where
    T: Eq + Hash + Copy,
{
    graph
        .entry(from)
        .and_modify(|pointees| {
            pointees.insert(to);
        })
        .or_insert_with(|| {
            let mut s = HashSet::new();
            s.insert(to);
            s
        });
}

struct State<T> {
    depends_on: Graph<T>,
    dependents: Graph<T>,
    no_deps: Vec<T>,
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

#[cfg(test)]
mod tests {

    #[test]
    fn toplogical_sort() {}
}
