pub mod app;
pub mod screenshots;

use std::{
    fs::{self, File},
    time::{Duration, SystemTime},
};

use crate::app::{settings::Settings, shader::get_shader::get_shader};
use app::Screenland;
use chrono::Local;
use clap::Parser;
use iced::application::BootFn;
use iced_aw::ICED_AW_FONT_BYTES;
use iced_layershell::{
    self,
    reexport::Anchor,
    settings::{LayerShellSettings, StartMode},
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    Layer, filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

#[derive(Parser, Clone)]
#[command(name = "Screenland")]
#[command(about = "Screenland is a program for creating and editing screenshots", long_about = None)]
pub struct Args {
    /// The placement of the color channels in the screenshot (rgba -> 0123; bgra -> 2103)
    #[arg(short, long)]
    color_format: Option<String>,
    /// Generate config
    #[arg(short, long)]
    generate_config: bool,
    /// Displays the shader with the current settings. Best used in conjunction with `bat`, for example: `-o | bat -l wgsl`
    #[arg(short, long)]
    output_shader: bool,
    /// Displays the shader with the current settings and run screenland
    #[arg(long)]
    output_shader_and_run: bool,
    /// Path to config. By default: `~/.config/screenland/config.yaml`
    #[arg(long)]
    config: Option<String>,
    /// File name format. To add the date and time, use https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    #[arg(long)]
    format: Option<String>,
    /// Path to the folder where screenshots will be saved
    #[arg(long)]
    path: Option<String>,
    /// Complete the screenshot immediately after selection (s | save | Save; c | copy | Copy)
    #[arg(short, long)]
    end: Option<String>,
    /// Disables overlay mode
    #[arg(long)]
    disables_overlay: bool,
    /// Input log
    #[arg(long)]
    input_log: bool,
}

fn main() -> Result<(), iced_layershell::Error> {
    let xdg_dirs = Settings::get_xdg_dir();
    let args = Args::parse();
    let arg_config = args
        .path
        .clone()
        .map(Into::into)
        .unwrap_or_else(|| Settings::get_path(Some(&xdg_dirs)));

    let logs_dir = xdg_dirs
        .create_state_directory("logs")
        .unwrap_or(".".into());
    let now = SystemTime::now();
    let max_age = Duration::from_hours(2 * 24);

    for entry in fs::read_dir(&logs_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        let modified = metadata.modified().unwrap();
        if let Ok(elapsed) = now.duration_since(modified) {
            if elapsed > max_age {
                fs::remove_file(&path).unwrap();
            }
        }
    }

    let file = File::create(logs_dir.join(Local::now().format("%F_%T.log").to_string())).unwrap();

    let console_layer = fmt::Layer::new().with_writer(std::io::stdout).with_filter(
        Targets::new()
            .with_default(if args.input_log {
                LevelFilter::INFO
            } else {
                LevelFilter::WARN
            })
            .with_target(
                env!("CARGO_PKG_NAME"),
                if args.input_log {
                    LevelFilter::DEBUG
                } else {
                    LevelFilter::WARN
                },
            ),
    );
    let file_layer = fmt::Layer::new().with_writer(file).with_filter(
        Targets::new()
            .with_default(LevelFilter::INFO)
            .with_target(env!("CARGO_PKG_NAME"), LevelFilter::DEBUG),
    );

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    if args.generate_config {
        Settings::new(arg_config, xdg_dirs).save();
        Ok(())
    } else if args.output_shader {
        println!("{}", get_shader(None));
        Ok(())
    } else {
        let settings = Settings::load(Some(args), Some(arg_config), Some(xdg_dirs));
        if settings.disables_overlay {
            iced::daemon(settings, Screenland::update, Screenland::view)
                .title(Screenland::title)
                .font(ICED_AW_FONT_BYTES)
                .theme(Screenland::theme)
                .subscription(Screenland::subscription)
                .run()
                .map_err(|err| match err {
                    iced::Error::ExecutorCreationFailed(err) => {
                        iced_layershell::Error::ExecutorCreationFailed(err)
                    }
                    iced::Error::WindowCreationFailed(err) => {
                        iced_layershell::Error::WindowCreationFailed(err)
                    }
                    iced::Error::GraphicsCreationFailed(err) => {
                        iced_layershell::Error::GraphicsCreationFailed(err)
                    }
                })
        } else {
            iced_layershell::daemon(
                move || settings.boot(),
                || "Screenland".to_string(),
                Screenland::update,
                Screenland::view,
            )
            .title(|app, id| Some(Screenland::title(app, id)))
            .font(ICED_AW_FONT_BYTES)
            .theme(Screenland::theme)
            .subscription(Screenland::subscription)
            .settings(iced_layershell::settings::Settings {
                layer_settings: LayerShellSettings {
                    size: Some((0, 0)),
                    exclusive_zone: 0,
                    anchor: Anchor::all(),
                    start_mode: StartMode::AllScreens,
                    ..Default::default()
                },
                ..Default::default()
            })
            .run()
        }
    }
}
