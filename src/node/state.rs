use std::collections::BTreeSet;

use crate::{message::LogEntry, util::{LogId, NodeId, TermId}};

pub enum NodeState {
  LeaderNode(LeaderNode),
  FollowerNode(FollowerNode),
  CandidateNode(CandidateNode),
}

struct LeaderNode {
  next_index: LogId,
  match_index: LogId
}

struct FollowerNode {
  leader: Option<NodeId>,
}

struct CandidateNode {
  votes_collected: u32,
}

// String -> rpc message 

struct RaftState {
  node_id: NodeId,
  state: NodeState,
  peers: BTreeSet<NodeId>,

  current_term: TermId,
  voted_for: Option<NodeId>,
  log: Vec<LogEntry>,

  commit_index: TermId,
  last_applied: TermId,
}

impl RaftState {
  pub fn new(
    node_id: NodeId,
    mut peers: BTreeSet<NodeId>,
    log: Vec<LogEntry>,
  ) -> Self {
    peers.remove(&node_id);
    Self {
      node_id,
      peers,
      state: NodeState::FollowerNode(FollowerNode {
        leader: None,
      }),
      current_term: 0,
      voted_for: None,
      log,
      commit_index: 0,
      last_applied: 0
    }
  }
}