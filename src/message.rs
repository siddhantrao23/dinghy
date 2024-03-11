use bytes::Bytes;
use crate::util::{LogId, NodeId, TermId};

enum RpcMessage {
  AppendEntriesRequest(AppendEntriesRequest), // messages and heartbeat
  AppendEntriesResponse(AppendEntriesResponse),
  VoteRequest(VoteRequest),
  VoteResponse(VoteResponse),
}

struct RaftMessage {
  term: TermId,
  msg: RpcMessage
}

pub struct LogEntry {
  term: TermId,
  data: Bytes
}

struct AppendEntriesRequest {
  leader_id: NodeId,
  prev_log_index: LogId,
  prev_log_term: TermId,
  leader_commit: LogId,
  entries: Vec<LogEntry>,
}

struct AppendEntriesResponse {
  success: bool,
}

struct VoteRequest {
  candidate_id: NodeId,
  last_log_index: LogId,
  last_log_term: TermId,
}

struct VoteResponse {
  term: TermId,
  vote_success: bool,
}