use yansi::{Color, Style};

// base logging function that is used by all others
fn log(logtype: &str, style: yansi::Style, content: &str) {
  println!("[{}] {}", style.paint(logtype), content);
}

// logs info to screen
pub fn info(input: String) {
  log("info", Style::new(Color::Cyan), &input)
}

// only compiles on debug builds, not in --release
#[cfg(debug_assertions)]
pub fn debug(input: &str) {
  log("debug", Style::new(Color::Magenta), &input)
}

#[cfg(not(debug_assertions))]
pub fn debug(_input: &str) {
  return
}
