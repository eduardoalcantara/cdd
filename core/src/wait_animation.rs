//! Pac-Man wait animation on stderr while search runs.
//!
//! Track pattern: `c · · · · ð · · · · · ð ...` (fruit every 5 cells)
//! Eat cycle: c → C+ð → @ → c (advance, fruit becomes ·)

use crossterm::terminal;
use std::io::{stderr, IsTerminal, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

const FRUIT: char = '\u{00F0}'; // ð
const SEED: char = '\u{00B7}'; // ·
const PAC_CLOSED: char = 'c';
const PAC_OPEN: char = 'C';
const PAC_EATING: char = '@';

const FRUIT_SPACING: usize = 5;
const FRAME_MS: u64 = 220;

pub struct WaitAnimation {
    stop: Arc<AtomicBool>,
    width: usize,
    handle: Option<JoinHandle<()>>,
}

impl WaitAnimation {
    pub fn start() -> Self {
        if !stderr().is_terminal() {
            return Self {
                stop: Arc::new(AtomicBool::new(true)),
                width: 0,
                handle: None,
            };
        }

        let width = terminal_width();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_for_thread = Arc::clone(&stop);
        let handle = thread::spawn(move || animation_loop(stop_for_thread, width));

        Self {
            stop,
            width,
            handle: Some(handle),
        }
    }
}

impl Drop for WaitAnimation {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        if self.width > 0 {
            let _ = write!(stderr(), "\r{}\r", " ".repeat(self.width));
            let _ = stderr().flush();
        }
    }
}

fn terminal_width() -> usize {
    terminal::size()
        .map(|(columns, _)| columns as usize)
        .unwrap_or(80)
        .clamp(20, 200)
}

fn is_fruit_cell(index: usize) -> bool {
    index >= FRUIT_SPACING && (index - FRUIT_SPACING) % FRUIT_SPACING == 0
}

fn init_track(width: usize) -> Vec<char> {
    (0..width)
        .map(|index| {
            if is_fruit_cell(index) {
                FRUIT
            } else {
                SEED
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EatPhase {
    Approach,
    Open,
    Bite,
    Swallow,
}

fn render_line(track: &[char], width: usize, position: usize, eat_phase: Option<EatPhase>) -> String {
    (0..width)
        .map(|index| {
            match eat_phase {
                None => {
                    if index == position {
                        PAC_CLOSED
                    } else {
                        track[index]
                    }
                }
                Some(EatPhase::Approach) => {
                    if index == position {
                        PAC_CLOSED
                    } else {
                        track[index]
                    }
                }
                Some(EatPhase::Open) => {
                    if index == position {
                        PAC_OPEN
                    } else if index == position + 1 {
                        FRUIT
                    } else {
                        track[index]
                    }
                }
                Some(EatPhase::Bite) => {
                    if index == position + 1 {
                        PAC_EATING
                    } else {
                        track[index]
                    }
                }
                Some(EatPhase::Swallow) => {
                    if index == position + 1 {
                        PAC_CLOSED
                    } else {
                        track[index]
                    }
                }
            }
        })
        .collect()
}

fn animation_loop(stop: Arc<AtomicBool>, width: usize) {
    let mut track = init_track(width);
    let mut position = 0usize;
    let mut eat_phase: Option<EatPhase> = None;

    while !stop.load(Ordering::Relaxed) {
        let line = render_line(&track, width, position, eat_phase);
        let _ = write!(stderr(), "\r{line}");
        let _ = stderr().flush();

        match eat_phase {
            None => {
                if position + 1 >= width {
                    position = 0;
                    track = init_track(width);
                } else if is_fruit_cell(position + 1) && track[position + 1] == FRUIT {
                    eat_phase = Some(EatPhase::Approach);
                } else {
                    position += 1;
                }
            }
            Some(EatPhase::Approach) => {
                eat_phase = Some(EatPhase::Open);
            }
            Some(EatPhase::Open) => {
                eat_phase = Some(EatPhase::Bite);
            }
            Some(EatPhase::Bite) => {
                track[position + 1] = SEED;
                eat_phase = Some(EatPhase::Swallow);
            }
            Some(EatPhase::Swallow) => {
                position += 1;
                eat_phase = None;
            }
        }

        thread::sleep(Duration::from_millis(FRAME_MS));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fruit_cells_follow_spacing_pattern() {
        assert!(!is_fruit_cell(0));
        assert!(!is_fruit_cell(4));
        assert!(is_fruit_cell(5));
        assert!(is_fruit_cell(10));
        assert!(is_fruit_cell(15));
    }

    #[test]
    fn init_track_places_fruits_and_seeds() {
        let track = init_track(12);
        assert_eq!(track[0..5], [SEED, SEED, SEED, SEED, SEED]);
        assert_eq!(track[5], FRUIT);
        assert_eq!(track[6..10], [SEED, SEED, SEED, SEED]);
        assert_eq!(track[10], FRUIT);
        assert_eq!(track[11], SEED);
    }

    #[test]
    fn open_phase_shows_capital_c_and_fruit_ahead() {
        let track = init_track(20);
        let line = render_line(&track, 20, 4, Some(EatPhase::Open));
        assert_eq!(line.chars().nth(4), Some(PAC_OPEN));
        assert_eq!(line.chars().nth(5), Some(FRUIT));
    }

    #[test]
    fn bite_phase_shows_at_on_fruit_cell() {
        let track = init_track(20);
        let line = render_line(&track, 20, 4, Some(EatPhase::Bite));
        assert_eq!(line.chars().nth(5), Some(PAC_EATING));
    }

    #[test]
    fn terminal_width_has_a_sensible_minimum() {
        assert!(terminal_width() >= 20);
    }
}
