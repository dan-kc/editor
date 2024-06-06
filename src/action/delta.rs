use chrono::{DateTime, Local};

#[allow(dead_code)]
enum BufferDelta {
    Insert {
        position: usize,
        text: Box<str>,
    },
    Delete {
        position: usize,
        length: usize,
    },
    Replace {
        position: usize,
        length: usize,
        text: Box<str>,
    },
}

enum AppDelta {
    Grapple,
}

enum Delta {
    Buffer(BufferDelta),
    App(AppDelta),
}

struct Action {
    timestamp: DateTime<Local>,
    deltas: Vec<Delta>,
}

impl Action {
    fn new(deltas: Vec<Delta>) -> Self {
        return Self {
            timestamp: Local::now(),
            deltas,
        };
    }
}
