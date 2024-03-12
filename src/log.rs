use bytes::Bytes;
use crate::util::{LogId, TermId};

pub struct LogEntry {
  term: TermId,
  data: Bytes
}

pub struct Log {
  log: Vec<LogEntry>,
  commit_index: LogId,
}