use std::process::Command;

pub fn move_window(name: &str) {
    move_hypr_window(name);
}

pub fn move_hypr_window(name: &str) {
    let _ = Command::new("hyprctl").args(["dispatch", "movewindow", format!("mon:{name}").as_str()]).status();
}
