use core::fmt::Debug;
use core::fmt::Display;
use core::time::Duration;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use voxell_timer::time;

#[derive(Debug)]
pub struct ThreadDetails {
    pub tx: mpsc::Sender<ThreadMessage>,
    #[forbid(unused)]
    pub handle: Option<JoinHandle<()>>,
}

pub enum ThreadMessage {
    Message(String),
    Close,
}

#[macro_export]
macro_rules! sentinel_println {
    (Sentinel: $sent:expr, println!($($args:tt)*)) => {{
        let tx_clone = $sent.print_thread.tx.clone();
        let msg = format!($($args)*);
        tx_clone.send($crate::solver::ThreadMessage::Message(msg)).unwrap();

    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PuzzleDetails {
    puzzle: usize,
    part: usize,
}

#[derive(Debug)]
pub struct SolverSentinel {
    pub total_time: Duration,
    pub print_thread: ThreadDetails,
    pub solved_puzzles: Vec<PuzzleDetails>,
}

impl SolverSentinel {
    #[inline]
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            while let Ok(thread_msg) = rx.recv() {
                match thread_msg {
                    ThreadMessage::Message(msg) => println!("{}", msg),
                    ThreadMessage::Close => break,
                };
            }
        });
        let handle = Some(handle);
        let det = ThreadDetails { tx, handle };

        Self {
            total_time: Duration::from_secs(0),
            print_thread: det,
            solved_puzzles: Vec::new(),
        }
    }

    #[inline]
    pub fn solve<Func, Ret>(&mut self, puzzle: usize, part: usize, input: &'static str, f: Func)
    where
        Ret: Debug + Display + Send + 'static,
        Func: FnOnce(&str, &mut Self) -> Ret,
    {
        let (res, dur) = time!(f(input, self));
        sentinel_println!(
            Sentinel: self,
            println!(
                "Solved puzzle {} part {}: {}\n\t\t\t^ this took {:?}\n____________________________________--------",
                puzzle, part, res, dur
            )
        );
        self.total_time += dur;
        self.solved_puzzles.push(PuzzleDetails { puzzle, part });
    }

    #[inline]
    pub fn finalize(mut self) {
        sentinel_println!(
            Sentinel: self,
            println!(
                "Solved {} puzzles.\n^^^^^^ all puzzles took a total of {:?}",
                self.solved_puzzles.len(),
                self.total_time
            )
        );

        self.print_thread.tx.send(ThreadMessage::Close).unwrap();
        self.print_thread.handle.take().unwrap().join().unwrap();
    }
}
