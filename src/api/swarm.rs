//! Control and manage clusters of engines also known as Swarm

use crate::{
    conn::Payload,
    models,
    opts::{SwarmInitOpts, SwarmJoinOpts},
    Docker, Result,
};

api_doc! { Swarm
|
pub struct Swarm {
    docker: Docker,
}
}

impl Swarm {
    pub fn new(docker: Docker) -> Self {
        Self { docker }
    }

    impl_api_ep! {_swarm: Swarm, resp
        Inspect -> "/swarm", models::Swarm
    }

    api_doc! { Swarm => Unlockkey
    /// Get the unlock key.
    |
    pub async fn get_unlock_key(&self) -> Result<models::UnlockKeyResponse> {
        self.docker.get_json("/swarm/unlockkey").await
    }}

    api_doc! { Swarm => Unlock
    /// Unlock a locked manager.
    |
    pub async fn unlock_manager(&self, key: &models::SwarmUnlockRequest) -> Result<()> {
        self.docker
            .post("/swarm/unlock", Payload::Json(serde_json::to_string(key)?))
            .await
            .map(|_| ())
    }}

    api_doc! { Swarm => Init
    /// Initialize a new swarm.
    |
    pub async fn initialize(&self, opts: &SwarmInitOpts) -> Result<()> {
        self.docker
            .post("/swarm/init", Payload::Json(opts.serialize()?))
            .await
            .map(|_| ())
    }}

    api_doc! { Swarm => Join
    /// Join an existing swarm.
    |
    pub async fn join(&self, opts: &SwarmJoinOpts) -> Result<()> {
        self.docker
            .post("/swarm/join", Payload::Json(opts.serialize()?))
            .await
            .map(|_| ())
    }}

    api_doc! { Swarm => Leave
    /// Leave the current swarm.
    |
    pub async fn leave(&self) -> Result<()> {
        self.docker
            .post("/swarm/leave?force=false", Payload::empty())
            .await
            .map(|_| ())
    }}

    api_doc! { Swarm => Leave
    /// Leave the current swarm forcefully, even if this is the last manager or that it will break the cluster.
    |
    pub async fn force_leave(&self) -> Result<()> {
        self.docker
            .post("/swarm/leave?force=true", Payload::empty())
            .await
            .map(|_| ())
    }}
}
