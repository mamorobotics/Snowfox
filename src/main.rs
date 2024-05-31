use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str;

mod camera;

const ADDR : &str = "192.168.1.1:8080";
const PORT : &str = "8080";

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind(ADDR)?;

        let mut i=0;
        while i <1 00
        {
            let mut init_buf = [0; 32];
            let (_amt, _src) = socket.recv_from(&mut init_buf)?;

            let msg = str::from_utf8(&init_buf).unwrap();

            let size = &msg[..msg.find("!").unwrap()].parse::<i32>().unwrap();
            let header = &msg[msg.rfind("!").unwrap()..].parse::<i32>().unwrap();

            let mut data_buf = Vec::new();

            let mut total_size = 0;
            while total_size < size-1
            {
                let mut temp_buf = Vec::new();
                temp_buf.resize(if (size - total_size) > 65500{65500} else {(size-total_size) as usize}, 0);
                let (_amt, _src) = socket.recv_from(&mut temp_buf)?;
                total_size += if (size - total_size) > 65500{65500} else {size-total_size};
                data_buf.append(&mut temp_buf);
            }

            let _ = send_camera(&socket);

            i+=1;
        }
    }
    Ok(())
}

fn send_camera(socket : &UdpSocket) -> std::io::Result<()> {
    {
        let cam_buf = [0;0];
        socket.send_to(&cam_buf, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), PORT.parse::<u16>().unwrap()))?;
    }
    Ok(())
}