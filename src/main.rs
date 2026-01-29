pub mod app;
pub mod screenshots;

use crate::screenshots::get_outputs;
use app::Screenland;
use clap::Parser;
use iced_aw::ICED_AW_FONT_BYTES;

#[derive(Parser)]
#[command(name = "Screenland")]
#[command(about = "Screenland is a program for creating and editing screenshots", long_about = None)]
struct Args {
    /// generate hyprland config
    #[arg(short, long)]
    generate_config: bool,
}

fn main() -> iced::Result {
    let args = Args::parse();

    if args.generate_config {
        println!(
            r"

# screenland stings
{}

windowrule = match:class screenland, no_anim on
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
        iced::Result::Ok(())
    } else {
        iced::daemon(Screenland::new, Screenland::update, Screenland::view)
            .title(Screenland::title)
            .font(ICED_AW_FONT_BYTES)
            .theme(Screenland::theme)
            .subscription(Screenland::subscription)
            .run()
    }
}
