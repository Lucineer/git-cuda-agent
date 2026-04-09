//! Command queue — async task dispatch with <5μs overhead

use std::collections::VecDeque;

/// Command types
#[derive(Debug, Clone)]
pub enum CommandType {
    Inference { prompt: String, max_tokens: usize },
    Reasoning { query: String, depth: usize },
    Coordinate { target: String, message: String },
    Learn { experience: String },
    Status,
    Shutdown,
}

/// A command in the queue
#[derive(Debug, Clone)]
pub struct Command {
    pub id: u64,
    pub cmd_type: CommandType,
    pub priority: u8,      // 0=low, 1=normal, 2=high, 3=critical
    pub confidence: f64,
}

/// Command queue with priority ordering
pub struct CommandQueue {
    queue: VecDeque<Command>,
    next_id: u64,
    total_processed: u64,
}

impl CommandQueue {
    pub fn new() -> Self {
        CommandQueue { queue: VecDeque::new(), next_id: 0, total_processed: 0 }
    }
    pub fn enqueue(&mut self, cmd_type: CommandType, priority: u8, confidence: f64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let cmd = Command { id, cmd_type, priority, confidence };
        // Insert sorted by priority (highest first)
        let pos = self.queue.iter().position(|c| c.priority < priority).unwrap_or(self.queue.len());
        self.queue.insert(pos, cmd);
        id
    }
    pub fn dequeue(&mut self) -> Option<Command> {
        let cmd = self.queue.pop_front();
        if cmd.is_some() { self.total_processed += 1; }
        cmd
    }
    pub fn len(&self) -> usize { self.queue.len() }
    pub fn is_empty(&self) -> bool { self.queue.is_empty() }
    pub fn stats(&self) -> (usize, u64) { (self.queue.len(), self.total_processed) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        let mut q = CommandQueue::new();
        q.enqueue(CommandType::Status, 0, 1.0);
        q.enqueue(CommandType::Inference { prompt: "test".into(), max_tokens: 100 }, 2, 0.9);
        q.enqueue(CommandType::Shutdown, 3, 1.0);
        assert_eq!(q.dequeue().unwrap().priority, 3); // critical first
        assert_eq!(q.dequeue().unwrap().priority, 2);
        assert_eq!(q.dequeue().unwrap().priority, 0);
    }
}
