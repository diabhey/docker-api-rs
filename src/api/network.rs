//! Create and manage user-defined networks that containers can be attached to.

use crate::{
    conn::Payload,
    models,
    opts::{
        ContainerConnectionOpts, ContainerDisconnectionOpts, NetworkCreateOpts, NetworkListOpts,
        NetworkPruneOpts,
    },
    Result,
};

impl_api_ty!(Network => id);

impl Network {
    impl_api_ep! { net: Network, resp
        Inspect -> &format!("/networks/{}", net.id), models::Network
        Delete -> &format!("/networks/{}", net.id), ()
    }

    api_doc! { Network => Connect
    /// Connect a container to a network.
    |
    pub async fn connect(&self, opts: &ContainerConnectionOpts) -> Result<()> {
        self.docker
            .post(
                &format!("/networks/{}/connect", self.id),
                Payload::Json(opts.serialize()?),
            )
            .await.map(|_| ())
    }}

    api_doc! { Network => Disconnect
    /// Disconnect a container from a network.
    |
    pub async fn disconnect(&self, opts: &ContainerDisconnectionOpts) -> Result<()> {
        self.docker
            .post(
                &format!("/networks/{}/disconnect", &self.id),
                Payload::Json(opts.serialize()?),
            )
            .await
            .map(|_| ())
    }}
}

impl Networks {
    impl_api_ep! { __: Network, resp
        List -> "/networks", models::Network
        Prune -> "/networks/prune", models::NetworkPruneResponse
    }

    api_doc! { Network => Create
    /// Create a new network.
    |
    pub async fn create(&self, opts: &NetworkCreateOpts) -> Result<Network> {
        // #TODO: handle missing id and return warnings (?)
        self.docker
            .post_json("/networks/create", Payload::Json(opts.serialize()?))
            .await
            .map(|resp: models::NetworkCreateResponse| {
                Network::new(self.docker.clone(), resp.id.unwrap_or_default())
            })
    }}
}
