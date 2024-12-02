use std::{
    fmt::{Debug, Display},
    sync::{mpsc, LazyLock, Mutex},
    thread::JoinHandle,
    time::Duration,
};

use voxell_timer::time;

pub struct PrintThreadDetails {
    pub tx: mpsc::Sender<ThreadMessage>,
    #[forbid(unused)]
    pub handle: Option<JoinHandle<()>>,
}

impl Drop for SolverSentinel {
    fn drop(&mut self) {
        PRINT_THREAD.lock().unwrap().tx.send(ThreadMessage::Close).unwrap();
        PRINT_THREAD.lock().unwrap().handle.take().unwrap().join().unwrap();
    }
}

pub enum ThreadMessage {
    Message(String),
    Close,
}

pub static PRINT_THREAD: LazyLock<Mutex<PrintThreadDetails>> = LazyLock::new(|| {
    let (tx, rx) = mpsc::channel();
    let handle = std::thread::spawn(move || {
        while let Ok(thread_msg) = rx.recv() {
            match thread_msg {
                ThreadMessage::Message(msg) => println!("{}", msg),
                ThreadMessage::Close => break,
            };
        }
    });
    let handle = Some(handle);
    Mutex::new(PrintThreadDetails { tx, handle })
});

#[macro_export]
macro_rules! threaded_println {
    ($($args:tt)*) => {{
        let tx_clone = $crate::solver::PRINT_THREAD.lock().unwrap().tx.clone();
        let msg = format!($($args)*);
        tx_clone.send($crate::solver::ThreadMessage::Message(msg)).unwrap();
    }};
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SolverSentinel {
    total_time: Duration,
}

impl SolverSentinel {
    #[inline]
    pub const fn new() -> Self {
        Self {
            total_time: Duration::from_secs(0),
        }
    }

    #[inline]
    pub fn solve<Func, Ret>(&mut self, puzzle: usize, part: usize, input: &'static str, f: Func)
    where
        Ret: Debug + Display + Send + 'static,
        Func: FnOnce(&str) -> Ret,
    {
        let (res, dur) = time!(f(input));
        self.total_time += dur;
        threaded_println!("Solved puzzle {} part {}: {}\n\t\t\t^ this took {:?}", puzzle, part, res, dur);
    }
}
