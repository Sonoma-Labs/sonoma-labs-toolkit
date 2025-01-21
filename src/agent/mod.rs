pub mod base;
pub mod trading;
pub mod analysis;
pub mod state;
pub mod capabilities;

pub use base::Agent;
pub use trading::TradingAgent;
pub use analysis::AnalysisAgent;
pub use state::AgentState;
pub use capabilities::AgentCapabilities;

pub trait AgentBehavior {
    fn process_data(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn update_state(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}