use clap::CommandFactory;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    process::exit,
};
use tiny_skia::IntSize;

use clap::Parser;
use mathjax_svg::Converter;
use resvg::usvg::{self, Tree};
use usvg::{fontdb, TreeParsing, TreeTextToPath};

mod args {
    use clap::Parser;
    use clap_complete::Shell;
    use rustyline::error::ReadlineError;
    use std::{borrow::Cow, path::PathBuf};

    /// Convert mathematical expressions into images
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    #[clap(disable_help_flag = true)]
    pub struct Args {
        /// Print help
        #[arg(long, action = clap::ArgAction::Help)]
        help: Option<bool>,

        /// Mathjax expression
        math: Option<String>,

        /// Output filename
        #[arg(short, long)]
        pub output: Option<PathBuf>,

        /// Output as PNG.
        /// If the argument `output` has a `.png` extension, it will be set automatically.
        #[arg(long, default_value_t = false)]
        png: bool,

        /// Generate shell completion
        #[arg(long, value_name = "SHELL")]
        pub completion: Option<Shell>,

        /// Number of horizontal dots in PNG image
        #[arg(short, long)]
        pub width: Option<u32>,

        /// Number of vertical dots in PNG image
        #[arg(short, long)]
        pub height: Option<u32>,
    }

    impl Args {
        /// Whether the output file format is PNG or not
        pub fn is_png(&self) -> bool {
            self.png || {
                let Some(output) = &self.output else { return false };
                let Some(ext) = output.extension() else { return false };
                ext == "png"
            }
        }

        /// Get math expression
        pub fn get_math(&self) -> Option<Cow<'_, String>> {
            Some(if self.math.is_some() {
                Cow::Borrowed(unsafe { self.math.as_ref().unwrap_unchecked() })
            } else {
                let Ok(mut rl) = rustyline::DefaultEditor::new() else { return None; };
                let mut string = String::new();
                loop {
                    match rl.readline("> ") {
                        Ok(line) => {
                            string.push_str(&line);
                            string.push('\n');
                        }
                        Err(ReadlineError::Eof) => {
                            break;
                        }
                        Err(_) => {
                            return None;
                        }
                    }
                }
                Cow::Owned(string)
            })
        }
    }
}

fn main() {
    let args = args::Args::parse();

    // Shell completion
    if let Some(shell) = args.completion {
        shell_completion(shell);
        return;
    }

    let data = {
        // Create Svg
        let svg_data = Converter::new()
            .convert_to_svg(
                {
                    let Some(math) = args.get_math() else {
                        // Terminates when input is interrupted.
                        exit(1);
                    };
                    math
                }
                .as_ref(),
            )
            .into_bytes();
        if !args.is_png() {
            // Substitute Svg.
            svg_data
        } else {
            // Convert to PNG
            let rtree = {
                let opt = usvg::Options::default();

                let fontdb = {
                    let mut fdb = fontdb::Database::new();
                    fdb.load_system_fonts();
                    fdb
                };

                let mut tree = Tree::from_data(&svg_data, &opt).unwrap();
                tree.convert_text(&fontdb);
                resvg::Tree::from_usvg(&tree)
            };

            let (mut pixmap, scale_x, scale_y) = {
                let original_size = rtree.size;
                let target_size = if args.width.is_some() && args.height.is_some() {
                    IntSize::from_wh(args.width.unwrap(), args.height.unwrap()).unwrap()
                } else if let Some(width) = args.width {
                    original_size.to_int_size().scale_to_width(width).unwrap()
                } else if let Some(height) = args.height {
                    original_size.to_int_size().scale_to_height(height).unwrap()
                } else {
                    original_size.to_int_size().scale_to_height(0x100).unwrap()
                };
                (
                    tiny_skia::Pixmap::new(target_size.width(), target_size.height()).unwrap(),
                    target_size.width() as f32 / original_size.width(),
                    target_size.height() as f32 / original_size.height(),
                )
            };
            rtree.render(
                tiny_skia::Transform::from_scale(scale_x, scale_y),
                &mut pixmap.as_mut(),
            );
            pixmap.encode_png().unwrap()
        }
    };

    macro_rules! write_to {
        ($to: expr) => {{
            let mut writer = BufWriter::new($to);
            writer.write_all(&data).unwrap();
            writer.flush().unwrap();
        }};
    }
    if let Some(path) = args.output {
        write_to!(File::create(path).unwrap());
    } else {
        write_to!(io::stdout());
    }
}

#[cold]
fn shell_completion(shell: clap_complete::Shell) {
    let mut stdout = BufWriter::new(io::stdout());
    let mut cmd = args::Args::command();
    let name = cmd.get_name().to_string();
    clap_complete::generate(shell, &mut cmd, name, &mut stdout);
}
