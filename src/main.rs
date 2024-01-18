mod term_progress;

use std::{
    io::{self, IsTerminal},
    thread,
    time::{Instant, SystemTime},
};

use clap::Parser;

use term_progress::TermProgress;

/// Simple timer in terminal
#[derive(Parser, Debug)]
struct Cli {
    /// Set time
    #[arg(short = 'T', long)]
    time: humantime::Duration,

    /// Description for timer
    desc: Option<String>,
}

fn main() -> io::Result<()> {
    let arg = Cli::parse();
    let start_time = Instant::now();
    let start_system_now = SystemTime::now();
    let duration = arg.time.into();
    if let Some(desc) = arg.desc {
        println!("{}", desc);
    }
    println!(
        "Timer started {} to {}, {}",
        humantime::format_rfc3339_seconds(start_system_now),
        humantime::format_rfc3339_seconds((start_system_now + duration).into()),
        humantime::format_duration(duration),
    );

    let stdout = io::stdout();
    if stdout.is_terminal() {
        let mut progress_bar = TermProgress::with_stdout(stdout);
        progress_bar.show_progress(start_time, duration)?;
    } else {
        thread::sleep(duration);
    }

    Ok(())
}
