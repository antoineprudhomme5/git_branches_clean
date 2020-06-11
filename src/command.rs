use std::process::Command;

pub fn exec_command(full_command: String) -> String {
  let parts: Vec<&str> = full_command.split(" ").collect();
  let (command, args) = parts.split_first().unwrap();

  let mut cmd = Command::new(command);
  cmd.args(args);
  let bytes_output_result = cmd.output();

  let bytes_output = match bytes_output_result {
    Ok(bytes) => bytes,
    Err(error) => panic!("An error occured executing the command {:?}", error),
  };

  let output_result = String::from_utf8(bytes_output.stdout);

  let output = match output_result {
    Ok(string) => string,
    Err(error) => panic!(
      "An error occured while converting the output to String {:?}",
      error
    ),
  };

  output
}
