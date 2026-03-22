pub mod app;
pub mod screenshots;

use crate::{
    app::{settings::Settings, shader::get_shader::get_shader},
    screenshots::get_outputs,
};
use app::Screenland;
use clap::Parser;
use iced_aw::ICED_AW_FONT_BYTES;

#[derive(Parser)]
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
}

fn main() -> iced::Result {
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
        iced::Result::Ok(())
    } else if args.generate_config {
        Settings::new(arg_config).save();
        iced::Result::Ok(())
    } else if args.output_shader {
        println!("{}", get_shader(None));
        iced::Result::Ok(())
    } else {
        iced::daemon(
            Settings::load(Some(args), Some(arg_config)),
            Screenland::update,
            Screenland::view,
        )
        .title(Screenland::title)
        .font(ICED_AW_FONT_BYTES)
        .theme(Screenland::theme)
        .subscription(Screenland::subscription)
        .run()
    }
}
