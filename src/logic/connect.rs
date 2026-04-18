use anyhow::{Ok, Result};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use qight::RelayClient;
use std::net::{IpAddr, SocketAddr};

async fn set_up_connection() -> Result<RelayClient> {
    let mdns = ServiceDaemon::new()?;
    let service_type = "_qight._udp.local.";
    let receiver = mdns.browse(service_type)?;

    let addr = loop {
        if let std::result::Result::Ok(ServiceEvent::ServiceResolved(info)) =
            receiver.recv_async().await
        {
            if let Some(addr) = info.get_addresses_v4().iter().next() {
                break SocketAddr::new(IpAddr::V4(*addr), info.get_port());
            }
        }
    };

    let client = RelayClient::connect(addr).await?;

    println!("Connected!");

    Ok(client)
}
