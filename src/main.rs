use std::{
    thread,
    time::{Duration, Instant, SystemTime},
};

use clap::Parser;

/// Simple timer in terminal
#[derive(Parser, Debug)]
struct Cli {
    /// Set time
    #[arg(short = 'T', long)]
    time: humantime::Duration,

    /// Description for timer
    desc: Option<String>,
}

fn main() {
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

    while start_time.elapsed() <= duration {
        let remains = duration - start_time.elapsed();

        if remains > Duration::from_secs(1) {
            let content = format!(
                "{} / {}",
                humantime::format_duration(Duration::from_secs(remains.as_secs())),
                humantime::format_duration(duration)
            );

            println!("{}", content);
        }

        let half_second = Duration::from_millis(500);
        if half_second > remains {
            thread::sleep(remains);
        } else {
            thread::sleep(half_second);
        }
    }
}
