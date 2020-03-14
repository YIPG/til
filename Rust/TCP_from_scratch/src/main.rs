use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap_mac::Iface::new("tun0", tun_tap_mac::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    let nbytes = nic.recv(&mut buf[..]).expect("failed at nit recv");
    eprintln!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    Ok(())
}
