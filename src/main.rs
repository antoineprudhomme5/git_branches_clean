mod command;
mod git;

extern crate dialoguer;

use crate::git::*;
use std::collections::HashSet;
use std::env;
use crate::command::*;

use dialoguer::{theme::ColorfulTheme, MultiSelect};

// FIXME: don't use clone here
fn ask_branches_to_delete(active_branch: &str) -> Vec<String> {
    let branches: Vec<String> = get_branches()
        .into_iter()
        .filter(|branch| branch != active_branch)
        .collect();
    let branches_to_keep: HashSet<String> = vec!["master".to_string(), "staging".to_string()]
        .into_iter()
        .collect();

    let branches_to_delete: Vec<String> = branches
        .clone()
        .into_iter()
        .filter(|branch| !branches_to_keep.contains(branch))
        .collect();

    let selected_branches_by_default: HashSet<String> = branches_to_delete.into_iter().collect();

    let defaults: Vec<bool> = branches
        .clone()
        .into_iter()
        .map(|branch| selected_branches_by_default.contains(&branch))
        .collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the branches to delete (with space)")
        .items(&branches[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    let mut selected_branches: Vec<String> = vec![];
    for selection in selections {
        // FIXME: this is ugly, but needed to find solution because
        // String is not Copy
        selected_branches.push(branches[selection].get(0..).unwrap().to_string());
    }

    selected_branches
}

fn main() {
    let path = match env::current_dir() {
        Ok(b) => b,
        Err(err) => panic!(err),
    };
    exec_command(format!("cd {}", path.into_os_string().into_string().unwrap()));

    let active_branch = get_active_branch();
    println!(
        "You are on branch \"{}\": can not deleted this branch",
        active_branch
    );

    let selected_branches = ask_branches_to_delete(&active_branch);
    delete_branches(&selected_branches);
    println!("{} branches deleted", selected_branches.len());
}
