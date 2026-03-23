pub mod app;
pub mod screenshots;

use crate::{
    app::{settings::Settings, shader::get_shader::get_shader},
    screenshots::get_outputs,
};
use app::Screenland;
use clap::Parser;
use iced::application::BootFn;
use iced_aw::ICED_AW_FONT_BYTES;
use iced_layershell::{self, reexport::Anchor, settings::{LayerShellSettings, StartMode}};

#[derive(Parser, Clone)]
#[command(name = "Screenland")]
#[command(about = "Screenland is a program for creating and editing screenshots", long_about = None)]
pub struct Args {
    /// Generate config for the supported system (hypr | hyprland)
    #[arg(short, long)]
    support_config: Option<String>,
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
}

fn main() -> Result<(), iced_layershell::Error> {
    let args = Args::parse();
    let arg_config = Settings::get_path(args.path.clone().map(Into::into));

    if let Some(sys) = args.support_config {
        match sys.as_str() {
            "hypr" | "hyprland" => {
                println!(
                    r"

# screenland stings
{}
windowrule = match:class screenland, float on
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
        Ok(())
    } else if args.generate_config {
        Settings::new(arg_config).save();
        Ok(())
    } else if args.output_shader {
        println!("{}", get_shader(None));
        Ok(())
    } else {
        let settings = Settings::load(Some(args), Some(arg_config));
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
