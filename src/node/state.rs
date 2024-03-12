use std::collections::BTreeSet;

use crate::{log::LogEntry, util::{LogId, NodeId, TermId}};

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
  votes_responded: u32,
  votes_granted: u32,
}

impl CandidateNode {
  pub fn new() -> Self {
    CandidateNode {
      votes_granted: 0,
      votes_responded: 0,
    }
  }
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

  pub fn leader(&self) -> Option<&NodeId> {
    match &self.state {
      NodeState::LeaderNode(_) => Some(&self.node_id),
      NodeState::FollowerNode(follower_state) => follower_state.leader.as_ref(),
      NodeState::CandidateNode(_) => None,
    }
  }

  fn restart(&mut self) {
    self.current_term += 1;
    self.voted_for = None;
    match &self.state {
      NodeState::LeaderNode(_) => todo!(),
      NodeState::FollowerNode(_) => todo!(),
      NodeState::CandidateNode(_) => todo!(),
    }
  }

  fn timeout(&mut self) {
    let a = match &self.state {
      NodeState::LeaderNode(_) => None,
      NodeState::FollowerNode(_) | NodeState::CandidateNode(_) => {
        self.current_term += 1;
        self.voted_for = Some(self.node_id.clone());
        let new_state = CandidateNode::new();
        Some(())
      }
    };
  }
}