use std::{
    io::{self, Error, Stdout, Write},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    style::Stylize,
    terminal, QueueableCommand,
};

pub(crate) struct TermProgress {
    stdout: Stdout,
}

impl TermProgress {
    pub(crate) fn with_stdout(stdout: Stdout) -> Self {
        TermProgress { stdout }
    }

    pub(crate) fn show_progress(
        &mut self,
        start_time: Instant,
        duration: Duration,
    ) -> io::Result<()> {
        self.stdout.queue(cursor::Hide)?;

        loop {
            if let Ok(true) = event::poll(Duration::from_secs(0)) {
                if let Ok(e) = event::read() {
                    match e {
                        Event::Key(key) if key.code == KeyCode::Char('c') => {
                            return Err(Error::other("user cancelled"));
                        }
                        _ => (),
                    }
                }
            }

            let elapsed = start_time.elapsed();
            let elapsed = if elapsed > duration {
                duration
            } else {
                elapsed
            };

            let (_, y) = cursor::position()?;
            let (w, _) = terminal::size()?;

            self.stdout
                .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
                .queue(cursor::MoveTo(0, y))?
                .queue(cursor::Hide)?;

            let percents = format!(
                "{:>5.1}%",
                100.0 * elapsed.as_secs() as f64 / duration.as_secs() as f64
            );
            let bar_width = w - percents.len() as u16 - 2;
            let progress = elapsed.as_millis() * bar_width as u128 / duration.as_millis();

            write!(
                self.stdout,
                " {}{} {}",
                "\u{2587}".repeat(progress as usize).blue(),
                "\u{2587}"
                    .repeat(bar_width as usize - progress as usize)
                    .dark_grey(),
                percents
            )?;

            self.stdout.flush()?;

            if elapsed >= duration {
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }
}

impl Drop for TermProgress {
    fn drop(&mut self) {
        let _ = self.stdout.queue(cursor::Show);
        let _ = self.stdout.flush();
    }
}
