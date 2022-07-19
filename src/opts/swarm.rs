use crate::models::SwarmSpec;
use containers_api::{impl_field, impl_opts_builder, impl_str_field, impl_vec_field};

impl_opts_builder!(json => SwarmJoin);

impl SwarmJoinOptsBuilder {
    impl_str_field!(
        /// Listen address used for inter-manager communication if the node gets promoted to manager,
        /// as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP).
        listen_addr => "ListenAddr"
    );

    impl_str_field!(
        // Externally reachable address advertised to other nodes. This can either be an address/port
        // combination in the form 192.168.1.1:4567, or an interface followed by a port number, like eth0:4567.
        // If the port number is omitted, the port number from the listen address is used. If AdvertiseAddr is
        // not specified, it will be automatically detected when possible.
        advertise_addr => "AdvertiseAddr"
    );

    impl_str_field!(
        /// Address or interface to use for data path traffic.
        data_path_addr => "DataPathAddr"
    );

    impl_vec_field!(
        /// Addresses of manager nodes already participating in the swarm.
        remote_addrs => "RemoteAddrs"
    );

    impl_str_field!(
        /// Secret token for joining this swarm.
        join_token => "JoinToken"
    );
}

impl_opts_builder!(json => SwarmInit);

impl SwarmInitOptsBuilder {
    impl_str_field!(
        // Listen address used for inter-manager communication if the node gets promoted to manager,
        // as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP).
        listen_addr => "ListenAddr"
    );

    impl_str_field!(
        // Externally reachable address advertised to other nodes. This can either be an address/port
        // combination in the form 192.168.1.1:4567, or an interface followed by a port number, like eth0:4567.
        // If the port number is omitted, the port number from the listen address is used. If AdvertiseAddr is
        // not specified, it will be automatically detected when possible.
        advertise_addr => "AdvertiseAddr"
    );

    impl_str_field!(
        /// Address or interface to use for data path traffic.
        data_path_addr => "DataPathAddr"
    );

    impl_field!(
        // Specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151.
        // If no port is set or is set to 0, default port 4789 will be used.
        data_path_port: u32 => "DataPathPort"
    );

    impl_vec_field!(
        // Default Address Pool specifies default subnet pools for global scope networks.
        default_addr_pool => "DefaultAddrPool"
    );

    impl_field!(
        /// Force creation of a new swarm.
        force_new_cluster: bool => "ForceNewCluster"
    );

    impl_field!(
        // SubnetSize specifies the subnet size of the networks created from the default subnet pool.
        subnet_size: u32 => "SubnetSize"
    );

    impl_field!(
        /// User modifiable swarm configuration.
        spec: SwarmSpec => "Spec"
    );
}
