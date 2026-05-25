mod clipboard;
mod color;
mod loupe;
mod screenshot;

use anyhow::{Context, Result};
use clap::Parser;
use color::Format;
use screenshot::Screenshot;

#[derive(Parser)]
#[command(name = "wlpick", about = "Wayland color picker")]
struct Cli {
    /// Output format for the picked color
    #[arg(long, short, value_enum, default_value_t = Format::Hex)]
    format: Format,

    /// Do not copy the result to clipboard
    #[arg(long)]
    no_clipboard: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let screen = Screenshot::capture().context("Screenshot failed")?;
    let color = match loupe::run(screen) {
        Some(c) => c,
        None => return Ok(()),
    };

    let output = color::format_color(color, &cli.format);
    println!("{output}");

    if !cli.no_clipboard {
        clipboard::copy(&output).context("Clipboard copy failed")?;
    }

    Ok(())
}
