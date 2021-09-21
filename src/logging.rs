use yansi::{Color, Style};

// base logging function that is used by all others
fn log(logtype: &str, style: yansi::Style, content: &str) {
  println!("[{}] {}", style.paint(logtype), content);
}

pub fn info(input: &str) {
  log("info", Style::new(Color::Cyan), input)
}

pub fn logrequest() {}
