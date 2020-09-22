pub enum ExitCode {
  Ok,
  Error,
}

impl Into<i32> for ExitCode {
  fn into(self) -> i32 {
    match self {
      ExitCode::Ok => 0,
      ExitCode::Error => 1,
    }
  }
}
