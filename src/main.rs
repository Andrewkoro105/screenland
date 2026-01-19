pub mod app;
pub mod move_window;
pub mod screenshots;

use app::Screenland;
use clap::Parser;
use iced_aw::ICED_AW_FONT_BYTES;

use crate::screenshots::get_outputs;

#[derive(Parser)]
#[command(name = "Screenland")]
#[command(about = "Screenland is a program for creating and editing screenshots", long_about = None)]
struct Args {
    /// Входной файл
    #[arg(short, long)]
    ganerate_config: bool,
}

fn main() -> iced::Result {
    let args = Args::parse();

    if args.ganerate_config {
        println!(
            "```\n{}\n```",
            get_outputs()
                .iter()
                .map(|name| format!("windowrule = match:title screenland-{name}, monitor {name}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        iced::Result::Ok(())
    } else {
        iced::daemon(Screenland::new, Screenland::update, Screenland::view)
            .title(Screenland::titel)
            .font(ICED_AW_FONT_BYTES)
            .theme(Screenland::theme)
            .subscription(Screenland::subscription)
            .run()
    }
}
