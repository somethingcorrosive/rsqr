mod qr;
mod render;
mod cli;

use clap::Parser;
use cli::{Args, Charset};

fn main() {
    let args = Args::parse();

    if args.charset != Charset::Block && args.png.is_none() {
        eprintln!(
            "⚠️  Warning: The {:?} charset may not be scannable by phone cameras.",
            args.charset
        );
    }

    let content = if args.me {
        "https://github.com/somethingcorrosive"
    } else if let Some(ref txt) = args.text {
        txt
    } else {
        eprintln!("Error: Please provide input text or use --me.");
        std::process::exit(1);
    };

    if let Some(png_path) = args.png.as_deref() {
        if let Err(e) = qr::export_png(content, png_path, args.scale) {
            eprintln!("❌ PNG export failed: {e}");
            std::process::exit(1);
        } else {
            println!("✅ PNG saved to {png_path}");
        }
    } else {
        match qr::generate_qr_matrix(content, !args.no_quiet) {
            Ok(matrix) => {
                let ascii = render::render_ascii(&matrix, args.invert, args.charset);
                println!("{ascii}");
            }
            Err(e) => {
                eprintln!("❌ Failed to generate QR: {e}");
                std::process::exit(1);
            }
        }
    }
}
