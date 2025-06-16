use clap::{CommandFactory, Parser};

fn main() -> std::io::Result<()> {
    match Opts::try_parse() {
        // Only show help if --help is passed
        Ok(opts) if opts.help => {
            let _ = Opts::command().print_long_help();
        }
        // Handle --version flag
        Err(e) => match e.kind() {
            clap::error::ErrorKind::DisplayVersion => {
                let _ = e.print();
            }
            _ => {}
        },
        // Ignore any other flags/args
        Ok(_) => {}
    };

    Ok(())
}

#[derive(Parser)]
#[command(
    version,
    disable_help_flag = true,
    about = "do nothing, successfully (and educationally)",
    long_about = "rust reimplementation of POSIX `true` for educational purposes"
)]
struct Opts {
    #[arg(long)]
    help: bool,
}
