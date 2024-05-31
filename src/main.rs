use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str;
use opencv::videoio::{self, VideoCapture};

mod camera;

const ADDR : &str = "192.168.1.1:8080";
const PORT : &str = "8080";

fn main() -> std::io::Result<()> {
    {
        let mut cam_num = 0;
        let mut cam_qual = 100;
        let mut cam1 = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
        let mut cam2 = videoio::VideoCapture::new(1, videoio::CAP_ANY).unwrap();

        let socket = UdpSocket::bind(ADDR)?;

        loop
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

            if *header == 6{
                
            }

            let _ = send_camera(&socket, &mut cam1, &mut cam2, cam_num, cam_qual);
        }
    }
}

fn send_camera(socket : &UdpSocket, cam1 : &mut VideoCapture, cam2 : &mut VideoCapture, cam_num : i32, cam_qual : i32) -> std::io::Result<()> {
    {
        let cam_buf;

        if cam_num==0{
            cam_buf = camera::get_camera_buf(cam1, cam_qual);
        }
        else{
            cam_buf = camera::get_camera_buf(cam2, cam_qual);
        }
        socket.send_to(&cam_buf, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), PORT.parse::<u16>().unwrap()))?;
    }
    Ok(())
}