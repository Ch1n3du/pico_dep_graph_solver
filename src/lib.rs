use std::collections::{HashMap, HashSet};

#[derive(serde::Deserialize)]
pub struct Task {
    name: String,
    files: HashSet<String>,
    deps: HashSet<String>,
}
pub fn find_tasks_to_rerun(tasks_path: &str, changed_files_path: &str) -> HashSet<(String, usize)> {
    let raw_tasks = std::fs::read_to_string(tasks_path).unwrap_or_else(|e| panic!("{e}"));
    let tasks: Vec<Task> = serde_json::from_str(&raw_tasks).expect("Error deserializing tasks");

    let raw_changed_files =
        std::fs::read_to_string(changed_files_path).expect("Error reading tasks from JSON.");
    let changed_files: Vec<String> =
        serde_json::from_str(&raw_changed_files).expect("Error deserializing tasks");

    let to_rerun = tasks_to_rerun(&tasks, changed_files);
    println!("Files to rerun: {to_rerun:?}");

    to_rerun
}

pub fn tasks_to_rerun(tasks: &[Task], changed_files: Vec<String>) -> HashSet<(String, usize)> {
    let dep_graph = build_dep_graph(tasks);

    let tasks_modified = tasks
        .iter()
        .filter(|t| t.files.iter().any(|f| changed_files.contains(f)))
        .map(|t| t.name.to_string());

    let mut all_tasks = HashSet::new();
    for id in tasks_modified {
        if let Some(sub_dependents) = get_dependents(&dep_graph, &id, 1) {
            all_tasks.extend(sub_dependents.into_iter())
        }
        all_tasks.insert((id, 0));
    }

    all_tasks
}

fn get_dependents(
    dep_graph: &HashMap<String, HashSet<String>>,
    id: &str,
    depth: usize,
) -> Option<HashSet<(String, usize)>> {
    if let Some(base_deps) = dep_graph.get(id) {
        let mut sub_deps = HashSet::new();

        for id in base_deps.iter() {
            if let Some(sub_sub_deps) = get_dependents(dep_graph, id, depth + 1) {
                sub_deps.extend(sub_sub_deps.into_iter())
            }
        }
        println!("{sub_deps:?}");

        let all_deps: HashSet<(String, usize)> = base_deps
            .iter()
            .map(|d| (d.to_owned(), depth))
            .chain(sub_deps.into_iter())
            .collect();

        println!("{all_deps:?}");
        Some(all_deps)
    } else {
        None
    }
}

fn build_dep_graph(tasks: &[Task]) -> HashMap<String, HashSet<String>> {
    let mut dep_graph: HashMap<String, HashSet<String>> = HashMap::new();

    for task in tasks {
        for dep in &task.deps {
            if let Some(hash_set) = dep_graph.get_mut(dep) {
                hash_set.insert(task.name.to_owned());
            } else {
                dep_graph.insert(dep.to_owned(), HashSet::from([task.name.to_owned()]));
            }
        }
    }

    dep_graph
}

#[cfg(test)]
mod tests {
    use crate::find_tasks_to_rerun;

    #[test]
    fn it_works() {
        find_tasks_to_rerun("test_tasks.json", "changed_files.json");
    }
}
