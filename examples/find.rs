use echonet_lite as el;
use el::{prop, props, Edt, Properties, Property};
use std::io;
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

const EL_MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 23, 0);

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3610")?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(&EL_MULTICAST_ADDR, &[0, 0, 0, 0].into())?;

    let packet = el::ElPacketBuilder::new()
        .transaction_id(1)
        .seoj([0x05u8, 0xFFu8, 0x01u8])
        .deoj([0x0Eu8, 0xF0u8, 0x01u8])
        .esv(el::ServiceCode::Get)
        .props(props!([0x80, []]))
        .build();
    let bytes = packet.serialize().expect("fail to serialize");

    socket.send_to(&bytes, (EL_MULTICAST_ADDR, 3610))?;
    loop {
        let mut buffer = [0u8; 1024];
        match socket.recv_from(&mut buffer) {
            Err(_) => break,
            Ok((_, src_addr)) => {
                if let Ok((_, response)) = el::ElPacket::from_bytes(&buffer) {
                    if response.is_response_for(&packet) {
                        println!("got response from {}", src_addr);
                        println!("{}", response);
                    }
                }
            }
        }
    }

    Ok(())
}
