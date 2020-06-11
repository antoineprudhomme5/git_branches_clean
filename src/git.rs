use crate::command::*;

pub fn get_active_branch() -> String {
  exec_command(String::from("git rev-parse --abbrev-ref HEAD"))
    .trim_end()
    .to_string()
}

fn remove_active_indicator_on_branch(branch_with_indicator: &str) -> &str {
  let split_output = branch_with_indicator.split_at(2);
  split_output.1
}

pub fn get_branches() -> Vec<String> {
  let output = exec_command("git branch".to_string());

  let trimmed_lines: Vec<&str> = output.lines().map(|line| line.trim()).collect();
  let branches: Vec<&str> = trimmed_lines
    .iter()
    .map(|line| {
      if line.starts_with("* ") {
        remove_active_indicator_on_branch(line)
      } else {
        line
      }
    })
    .collect();

  branches.iter().map(|branch| branch.to_string()).collect()
}

pub fn delete_branches(branches: &Vec<String>) {
  let branches_list = branches.join(" ");
  let command = format!("git branch -D {}", branches_list);
  exec_command(command);
}
