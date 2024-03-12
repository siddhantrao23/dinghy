use crate::{log::LogEntry, util::{LogId, NodeId, TermId}};

enum RpcMessage {
  AppendEntriesRequest(AppendEntriesRequest), // messages and heartbeat
  AppendEntriesResponse(AppendEntriesResponse),
  RequestVoteRequest(RequestVoteRequest),
  RequestVoteResponse(RequestVoteResponse),
}

struct RaftMessage {
  term: TermId,
  msg: RpcMessage
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

struct RequestVoteRequest {
  candidate_id: NodeId,
  last_log_index: LogId,
  last_log_term: TermId,
}

struct RequestVoteResponse {
  term: TermId,
  vote_success: bool,
}