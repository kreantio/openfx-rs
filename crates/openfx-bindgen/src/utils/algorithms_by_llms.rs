//! Authors:
//! - DeepSeek V4 Flash (0731, I guess)

use std::collections::{HashMap, HashSet, LinkedList};

/// Sorts the things based on their dependencies. Things that include other
/// things will be placed after those things in the sorted list, so that a
/// thing's dependencies always come first. This ensures that when generating
/// bindings, the dependencies are processed first, allowing for proper
/// resolution of types and functions.
///
/// ## Panics
///
/// - …, if there are circular dependencies.
/// - …, if there are unknown dependencies.
pub fn sort_by_dependencies<T: DependencySortable>(things: Vec<T>) -> Vec<T> {
    let mut things_by_name: HashMap<String, T> = things
        .into_iter()
        .map(|thing| (thing.name().to_string(), thing))
        .collect();

    // `dependents[name]` lists the things that directly depend on `name`.
    // `remaining_dependencies[name]` is the set of `name`'s dependencies that
    // haven't been emitted yet.
    let mut dependents: HashMap<String, Vec<String>> = HashMap::new();
    let mut remaining_dependencies: HashMap<String, HashSet<String>> = HashMap::new();

    for thing in things_by_name.values() {
        for included in thing.dependencies() {
            if !things_by_name.contains_key(included) {
                panic!(
                    "Unknown dependency: thing `{}` includes `{}`, which is not among the things to generate bindings for",
                    thing.name(),
                    included
                );
            }
            if included == thing.name() {
                continue;
            }
            dependents
                .entry(included.clone())
                .or_default()
                .push(thing.name().to_string());
            remaining_dependencies
                .entry(thing.name().to_string())
                .or_default()
                .insert(included.clone());
        }
    }

    let mut sorted_thing_list = LinkedList::<T>::new();

    // Kahn's algorithm: repeatedly emit things whose dependencies are all
    // satisfied.
    let mut ready: Vec<String> = things_by_name
        .keys()
        .filter(|name| {
            remaining_dependencies
                .get(*name)
                .is_none_or(|deps| deps.is_empty())
        })
        .cloned()
        .collect();

    while let Some(name) = ready.pop() {
        let thing = things_by_name.remove(&name).expect("duplicate name");
        sorted_thing_list.push_back(thing);

        if let Some(dependents_of) = dependents.remove(&name) {
            for dependent in dependents_of {
                let deps = remaining_dependencies
                    .get_mut(&dependent)
                    .expect("dependent should have an entry");
                deps.remove(&name);
                if deps.is_empty() {
                    ready.push(dependent);
                }
            }
        }
    }

    if !things_by_name.is_empty() {
        panic!(
            "Circular dependency detected among things: {:?}",
            things_by_name.keys().collect::<Vec<_>>()
        );
    }

    sorted_thing_list.into_iter().collect()
}

pub trait DependencySortable {
    fn name(&self) -> &str;
    fn dependencies(&self) -> &HashSet<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Thing {
        name: &'static str,
        deps: HashSet<String>,
    }

    impl Thing {
        fn new(name: &'static str, deps: &[&str]) -> Self {
            Thing {
                name,
                deps: deps.iter().map(|s| s.to_string()).collect(),
            }
        }
    }

    impl DependencySortable for Thing {
        fn name(&self) -> &str {
            self.name
        }

        fn dependencies(&self) -> &HashSet<String> {
            &self.deps
        }
    }

    fn names(things: &[Thing]) -> Vec<&'static str> {
        things.iter().map(|t| t.name).collect()
    }

    /// Asserts the core invariant: every dependency of a thing appears strictly
    /// before it in `sorted`. Exact ordering isn't asserted for graphs with
    /// multiple "ready" nodes at once, since `sort_by_dependencies` only
    /// guarantees the partial order, not a specific total order.
    fn assert_dependencies_before(sorted: &[Thing]) {
        for (i, thing) in sorted.iter().enumerate() {
            let names_before: HashSet<&'static str> = sorted[..i].iter().map(|t| t.name).collect();
            for dep in &thing.deps {
                assert!(
                    names_before.contains(dep.as_str()),
                    "dependency `{}` of `{}` must appear before it (got: {:?})",
                    dep,
                    thing.name,
                    names(sorted)
                );
            }
        }
    }

    #[test]
    fn empty_list() {
        let sorted = sort_by_dependencies::<Thing>(vec![]);
        assert!(sorted.is_empty());
    }

    #[test]
    fn single_thing() {
        let sorted = sort_by_dependencies(vec![Thing::new("a", &[])]);
        assert_eq!(names(&sorted), vec!["a"]);
    }

    #[test]
    fn simple_chain() {
        // c -> b -> a
        let sorted = sort_by_dependencies(vec![
            Thing::new("c", &["b"]),
            Thing::new("a", &[]),
            Thing::new("b", &["a"]),
        ]);
        assert_eq!(names(&sorted), vec!["a", "b", "c"]);
        assert_dependencies_before(&sorted);
    }

    #[test]
    fn transitive_chain() {
        // d -> c -> b -> a
        let sorted = sort_by_dependencies(vec![
            Thing::new("d", &["c"]),
            Thing::new("b", &["a"]),
            Thing::new("c", &["b"]),
            Thing::new("a", &[]),
        ]);
        assert_eq!(names(&sorted), vec!["a", "b", "c", "d"]);
        assert_dependencies_before(&sorted);
    }

    #[test]
    fn diamond_dependencies() {
        //        top
        //       /    \
        //    left    right
        //       \    /
        //       bottom
        let sorted = sort_by_dependencies(vec![
            Thing::new("top", &["left", "right"]),
            Thing::new("left", &["bottom"]),
            Thing::new("right", &["bottom"]),
            Thing::new("bottom", &[]),
        ]);
        assert_dependencies_before(&sorted);
        assert_eq!(names(&sorted).len(), 4);
    }

    #[test]
    fn disconnected_components() {
        // b -> a, and c on its own
        let sorted = sort_by_dependencies(vec![
            Thing::new("b", &["a"]),
            Thing::new("c", &[]),
            Thing::new("a", &[]),
        ]);
        assert_dependencies_before(&sorted);
        assert_eq!(names(&sorted).len(), 3);
    }

    #[test]
    fn self_dependency_is_ignored() {
        let sorted = sort_by_dependencies(vec![Thing::new("a", &["a"])]);
        assert_eq!(names(&sorted), vec!["a"]);
    }

    #[test]
    fn duplicate_dependencies_are_handled() {
        // b lists `a` twice in a HashSet — a HashSet can't hold dupes, but
        // verify a thing depending on two things that each satisfy part of it.
        let sorted = sort_by_dependencies(vec![
            Thing::new("c", &["a", "b"]),
            Thing::new("b", &["a"]),
            Thing::new("a", &[]),
        ]);
        assert_dependencies_before(&sorted);
        assert_eq!(names(&sorted).len(), 3);
    }

    #[test]
    #[should_panic(expected = "Circular dependency")]
    fn circular_dependency_panics() {
        sort_by_dependencies(vec![Thing::new("a", &["b"]), Thing::new("b", &["a"])]);
    }

    #[test]
    #[should_panic(expected = "Circular dependency")]
    fn self_referential_cycle_panics_when_part_of_cycle() {
        // a <-> b, with c depending on a (a is the cycle entry point).
        sort_by_dependencies(vec![
            Thing::new("a", &["b"]),
            Thing::new("b", &["a"]),
            Thing::new("c", &["a"]),
        ]);
    }

    #[test]
    #[should_panic(expected = "Unknown dependency")]
    fn unknown_dependency_panics() {
        sort_by_dependencies(vec![Thing::new("a", &["b"])]);
    }
}
