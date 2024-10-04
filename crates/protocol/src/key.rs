pub trait AgentSigner<RES> {
    fn sign_connect_req(&self) -> Vec<u8>;
    fn validate_connect_res(&self, resp: &[u8]) -> anyhow::Result<RES>;
}

pub trait ClusterValidator<REQ>: Send + Sync + Clone + 'static {
    fn validate_connect_req(&self, req: &[u8]) -> anyhow::Result<REQ>;
    fn generate_agent_id(&self, req: &REQ) -> anyhow::Result<u64>;
    fn generate_domain(&self, req: &REQ) -> anyhow::Result<String>;
    fn sign_response_res(&self, m: &REQ, err: Option<String>) -> Vec<u8>;
}
