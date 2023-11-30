use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant, SystemTime},
};

use clap::Parser;
use crossterm::{cursor, terminal, QueueableCommand};

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

    let mut stdout = io::stdout();

    loop {
        let elapsed = start_time.elapsed();
        let elapsed = if elapsed > duration { duration } else { elapsed };

        let (_, y) = cursor::position()?;
        let (w, _) = terminal::size()?;

        stdout
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
            .queue(cursor::MoveTo(0, y))?
            .queue(cursor::Hide)?;

        let time_span = format!(
            "{}/{}",
            humantime::format_duration(Duration::from_secs(elapsed.as_secs())),
            humantime::format_duration(duration)
        );
        let bar_width = w - time_span.len() as u16 - 4;
        let progress = elapsed.as_millis() * bar_width as u128 / duration.as_millis();

        write!(
            stdout,
            "[{}{}][{}]",
            "#".repeat(progress as usize),
            "-".repeat(bar_width as usize - progress as usize),
            time_span
        )?;

        stdout.flush()?;

        if elapsed == duration {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }

    stdout.queue(cursor::Show)?;

    Ok(())
}
