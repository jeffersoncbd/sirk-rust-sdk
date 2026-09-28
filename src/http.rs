mod health;
mod new;
mod read;
mod run_agent;

pub(super) struct HttpTransport {
    pub(super) agent: ureq::Agent,
    pub(super) endpoint: String,
}
