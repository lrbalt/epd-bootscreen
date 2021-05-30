use clap::Clap;
use log::{info, LevelFilter};

mod color;
mod draw;
#[cfg(feature = "emulator")]
mod emulator;
#[cfg(feature = "e-paper")]
mod epd;
#[cfg(not(feature = "e-paper"))]
mod tri_color;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Reinier Balt <lrbalt@gmail.com>")]
struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Show bootscreen on e-paper screen
    #[cfg(feature = "e-paper")]
    EPaper,
    /// Show bootscreen on emulator
    #[cfg(feature = "emulator")]
    Emulator,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut builder = env_logger::builder();
    if opts.verbose > 0 {
        let level = match opts.verbose {
            0 => LevelFilter::Off,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };
        builder.filter_level(level);
    }
    builder.init();

    info!("log level: {}", opts.verbose);
    match opts.subcmd {
        #[cfg(feature = "e-paper")]
        SubCommand::EPaper => epd::epd(),
        #[cfg(feature = "emulator")]
        SubCommand::Emulator => emulator::emulator(),
    }
}
