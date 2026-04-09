//! Fleet protocol bridge — Cocapn A2A/A2UI integration

use crate::AgentState;

/// Fleet message types
#[derive(Debug, Clone)]
pub enum FleetMessage {
    Heartbeat { agent_id: String, state: AgentState },
    TaskRequest { from: String, task_type: String, payload: String },
    TaskResponse { to: String, result: String, confidence: f64 },
    Discovery { agent_id: String, capabilities: Vec<String> },
    Status { agent_id: String, metrics: FleetMetrics },
}

#[derive(Debug, Clone, Default)]
pub struct FleetMetrics {
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub avg_confidence: f64,
    pub uptime_s: u64,
    pub gpu_utilization_pct: f64,
}

/// Fleet protocol handler
pub struct FleetBridge {
    pub agent_id: String,
    pub connected_agents: Vec<String>,
    pub message_log: Vec<FleetMessage>,
    pub metrics: FleetMetrics,
}

impl FleetBridge {
    pub fn new(agent_id: &str) -> Self {
        FleetBridge {
            agent_id: agent_id.to_string(),
            connected_agents: Vec::new(),
            message_log: Vec::new(),
            metrics: FleetMetrics::default(),
        }
    }

    /// Send a message to the fleet
    pub fn send(&mut self, msg: FleetMessage) {
        match &msg {
            FleetMessage::Discovery { agent_id, .. } => {
                if !self.connected_agents.contains(agent_id) {
                    self.connected_agents.push(agent_id.clone());
                }
            }
            FleetMessage::TaskResponse { result, confidence, .. } => {
                if *confidence > 0.5 {
                    self.metrics.tasks_completed += 1;
                } else {
                    self.metrics.tasks_failed += 1;
                }
            }
            _ => {}
        }
        self.message_log.push(msg);
    }

    /// Generate heartbeat message
    pub fn heartbeat(&self, state: &AgentState) -> FleetMessage {
        FleetMessage::Heartbeat {
            agent_id: self.agent_id.clone(),
            state: state.clone(),
        }
    }

    /// Get fleet status summary
    pub fn status(&self) -> String {
        format!("Agent {}: {} peers, {}/{} tasks ok, conf {:.2}%",
            self.agent_id, self.connected_agents.len(),
            self.metrics.tasks_completed,
            self.metrics.tasks_completed + self.metrics.tasks_failed,
            self.metrics.avg_confidence * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fleet_bridge() {
        let mut bridge = FleetBridge::new("test-agent");
        bridge.send(FleetMessage::Discovery {
            agent_id: "peer-1".into(), capabilities: vec!["inference".into()],
        });
        bridge.send(FleetMessage::TaskResponse {
            to: "peer-1".into(), result: "ok".into(), confidence: 0.9,
        });
        assert_eq!(bridge.connected_agents.len(), 1);
        assert_eq!(bridge.metrics.tasks_completed, 1);
        assert!(bridge.status().contains("test-agent"));
    }
}
