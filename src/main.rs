pub mod app;
pub mod screenshots;

use std::{fs, path::PathBuf};

use crate::{
    app::{end::End, settings::Settings},
    screenshots::get_outputs,
};
use app::Screenland;
use clap::Parser;
use directories::UserDirs;
use iced_aw::ICED_AW_FONT_BYTES;
use serde_yaml::from_reader;

#[derive(Parser)]
#[command(name = "Screenland")]
#[command(about = "Screenland is a program for creating and editing screenshots", long_about = None)]
struct Args {
    /// generate config for the supported system (hypr | hyprland)
    #[arg(short, long)]
    support_config: Option<String>,
    /// generate config
    #[arg(short, long)]
    generate_config: bool,
    /// path to config. By default: `~/.config/screenland/config.yaml`
    #[arg(short, long)]
    config: Option<String>,
    /// file name format. To add the date and time, use https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    #[arg(short, long)]
    format: Option<String>,
    /// path to the folder where screenshots will be saved
    #[arg(short, long)]
    path: Option<String>,
    /// complete the screenshot immediately after selection (s | save | Save; c | copy | Copy)
    #[arg(short, long)]
    end: Option<String>,
}

fn main() -> iced::Result {
    let args = Args::parse();
    let arg_config = args.config.clone().map(PathBuf::from).unwrap_or(
        if let Some(user_dirs) = UserDirs::new() {
            user_dirs.home_dir().join(".config/screenland/config.yaml")
        } else {
            PathBuf::from("config.txt")
        },
    );

    if let Some(sys) = args.support_config {
        match sys.as_str() {
            "hypr" | "hyprland" => {
                println!(
                    r"

# screenland stings
{}

windowrule = match:class screenland, no_anim on
windowrule = match:title Save As, float on
",
                    get_outputs()
                        .iter()
                        .map(|outputs| format!(
                            "windowrule = match:title screenland-{}, monitor {}",
                            outputs.name, outputs.name
                        ))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
            _ => {
                eprintln!("{sys} unsupported")
            }
        }
        iced::Result::Ok(())
    } else if args.generate_config {
        Settings::new(arg_config).save();
        iced::Result::Ok(())
    } else {
        let mut settings = fs::OpenOptions::new()
            .read(true)
            .open(&arg_config)
            .map(|file| {
                let result = from_reader::<_, Settings>(file);
                if let Err(err) = &result {
                    eprintln!("Configuration parsing error:\n{err}");
                }
                result.ok()
            })
            .inspect_err(|err| {
                if let Some(path) = args.config {
                    eprintln!("Unable to open file: {path}. Error: {err}")
                }
            })
            .ok()
            .flatten()
            .unwrap_or_else(|| Settings::new(arg_config));

        if let Some(path) = args.path {
            settings.path = PathBuf::from(path);
            settings.cli_path = true;
        }

        if let Some(format) = args.format {
            settings.format = format;
            settings.cli_format = true;
        }

        if let Some(end) = args.end {
            match end.as_str() {
                "s" | "save" | "Save" => {
                    settings.base_end = Some(End::Save);
                    settings.cli_base_end = true;
                }
                "c" | "copy" | "Copy" => {
                    settings.base_end = Some(End::Copy);
                    settings.cli_base_end = true;
                }
                _ => eprintln!("{end} unsupported termination method"),
            };
        }

        iced::daemon(settings, Screenland::update, Screenland::view)
            .title(Screenland::title)
            .font(ICED_AW_FONT_BYTES)
            .theme(Screenland::theme)
            .subscription(Screenland::subscription)
            .run()
    }
}
