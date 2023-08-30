use matchbox_signaling::SignalingServer;
use std::net::Ipv4Addr;
use tracing::info;

pub async fn signaling_server() -> Result<(), matchbox_signaling::Error> {
    let server = SignalingServer::full_mesh_builder((Ipv4Addr::UNSPECIFIED, 3536))
        .on_connection_request(|connection| {
            info!("Connecting: {connection:?}");
            Ok(true) // Allow all connections
        })
        .on_id_assignment(|(socket, id)| info!("{socket} received {id}"))
        .on_peer_connected(|id| info!("Joined: {id}"))
        .on_peer_disconnected(|id| info!("Left: {id}"))
        .cors()
        .trace()
        .build();
    server.serve().await
}
