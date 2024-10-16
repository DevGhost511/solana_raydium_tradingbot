use crate::types::actions::{Amount, SolanaTransferActionPayload};
use crate::types::events::BotEvent;

#[derive(Debug, Clone)]
pub enum SolanaStrategyEvent {
    Original(BotEvent),
    ForAgent(AgentEvent),
}
impl From<BotEvent> for SolanaStrategyEvent {
    fn from(event: BotEvent) -> Self {
        SolanaStrategyEvent::Original(event)
    }
}

#[derive(Debug, Clone)]
pub enum AgentEvent {
    Buy(Amount),
    Sell(Amount),
    Transfer(Vec<SolanaTransferActionPayload>),
    Collect,
    Deactivate,
}
