use pico_dep_graph_solver::find_tasks_to_rerun;
use std::env;

fn main() {
    let mut args = env::args();
    let tasks_file = args.next().expect("Expected the tasks file.");
    let changed_files = args.next().expect("Expected a changed files list.");

    let _tasks_to_rerun = find_tasks_to_rerun(&tasks_file, &changed_files);
}
