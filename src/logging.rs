use yansi::{Color, Style};

// base logging function that is used by all others
fn log(logtype: &str, style: yansi::Style, content: &str) {
  println!("[{}] {}", style.paint(logtype), content);
}

// logs info to screen
pub fn info(input: String) {
  log("info", Style::new(Color::Cyan), &input)
}

pub fn error(input: Option<String>) {
  if let Some(payload) = input {
    log("error", Style::new(Color::Red), &payload)
  } else {
    log(
      "error",
      Style::new(Color::Red),
      "an undefined error occured",
    )
  }
}

// only compiles on debug builds, not in --release
#[cfg(debug_assertions)]
pub fn debug(input: &str) {
  log("debug", Style::new(Color::Magenta), &input)
}

#[cfg(not(debug_assertions))]
pub fn debug(_input: &str) {
  return;
}
