use mockall::automock;
use sawtooth_alica_payload::payloads::TransactionPayload;

#[automock]
pub trait TransactionPayloadFilter {
    fn filter(&self, payloads: &mut Vec<TransactionPayload>);
}

pub struct AgentIdFilter {
    required_id: String
}

impl AgentIdFilter {
    pub fn new(id: &str) -> Self {
        AgentIdFilter {
            required_id: id.to_string()
        }
    }
}

impl TransactionPayloadFilter for AgentIdFilter {
    fn filter(&self, payloads: &mut Vec<TransactionPayload>) {
        payloads.retain(|payload| &payload.agent_id == &self.required_id)
    }
}

pub struct MessageTypeFilter {
    required_message_type: String
}

impl MessageTypeFilter {
    pub fn new(message_type: &str) -> Self {
        MessageTypeFilter {
            required_message_type: message_type.to_string()
        }
    }
}

impl TransactionPayloadFilter for MessageTypeFilter {
    fn filter(&self, payloads: &mut Vec<TransactionPayload>) {
        payloads.retain(|payload| &payload.message_type == &self.required_message_type)
    }
}

#[cfg(test)]
mod test {
    use sawtooth_alica_payload::payloads::TransactionPayload;
    use crate::filter::{AgentIdFilter, TransactionPayloadFilter, MessageTypeFilter};

    #[test]
    fn it_filters_all_payloads_with_the_wrong_agent_id_out() {
        let id_to_filter = "agent1";
        let mut payloads = vec![
            TransactionPayload::new(id_to_filter, "type", "message".as_bytes(), 69182798179),
            TransactionPayload::new("agent2", "type", "message".as_bytes(), 69182798179),
            TransactionPayload::new("agent3", "type", "message".as_bytes(), 69182798179),
        ];

        AgentIdFilter::new(id_to_filter).filter(&mut payloads);

        assert_eq!(payloads.len(), 1);
    }

    #[test]
    fn it_filters_all_payloads_with_the_wrong_message_type() {
        let message_type_to_filter = "type1";
        let mut payloads = vec![
            TransactionPayload::new("agent1", message_type_to_filter, "message".as_bytes(), 69182798179),
            TransactionPayload::new("agent2", message_type_to_filter, "message".as_bytes(), 69182798179),
            TransactionPayload::new("agent3", "other_type", "message".as_bytes(), 69182798179),
        ];

        MessageTypeFilter::new(message_type_to_filter).filter(&mut payloads);

        assert_eq!(payloads.len(), 2);
    }
}