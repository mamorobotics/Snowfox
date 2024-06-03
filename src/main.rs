use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str;
use opencv::videoio::{self, VideoCapture, *};

mod camera;

const ADDR : &str = "192.168.1.1:8080";
const PORT : &str = "8080";

const WIDTH : f64 = 1280.0;
const HEIGHT : f64 = 720.0;
const FPS : f64 = 20.0;

fn main() -> std::io::Result<()> {
    {
        let mut cam_num = 0;
        let mut cam_qual = 100;

        let mut cam1 = camera::Camera(0, WIDTH, HEIGHT, FPS);
        let mut cam2 = camera::Camera(1, WIDTH, HEIGHT, FPS);

        let socket = UdpSocket::bind(ADDR)?;
        socket.set_nonblocking(true).unwrap();

        socket.send_to(&[0,1,1,0], SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), PORT.parse::<u16>().unwrap()))?;

        loop
        {
            let mut init_buf = [0; 32];
            let (_amt, _src) = socket.recv_from(&mut init_buf)?;

            let msg = str::from_utf8(&init_buf).unwrap();

            let size = &msg[..msg.rfind("!").unwrap()].parse::<i32>().unwrap(); 
            let headers = &msg[msg.rfind("!").unwrap()..];

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

            for head in (*headers).split("?"){
                if head == "6"{
                    let data = str::from_utf8(&data_buf).unwrap();
                    for msg_temp_all in data.split("?"){
                        let msg_temp : Vec<_> = msg_temp_all.split("!").collect();
                        let msg_temp_type = msg_temp.clone().into_iter().nth(0);
                        let msg_temp_val = msg_temp.clone().into_iter().nth(1).expect("value").parse::<i32>().unwrap();
                        if msg_temp_type == Some("cam") 
                        {
                            cam_num = msg_temp_val;
                        }else if msg_temp_type == Some("qual") 
                        {
                            cam_qual = msg_temp_val;
                        }
                    }
                }
            }

            let _ = send_camera(&socket, &mut cam1, &mut cam2, cam_num, cam_qual);
        }
    }
}

fn send_camera(socket : &UdpSocket, cam1 : &mut camera::Camera, cam2 : &mut camera::Camera, cam_num : i32, cam_qual : i32) -> std::io::Result<()> {
    {
        let cam_buf;

        if cam_num==0{
            cam_buf = cam1.getCameraBuf(cam_qual);
        }
        else{
            cam_buf = cam2.getCameraBuf(cam_qual);
        }
        socket.send_to(&cam_buf, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), PORT.parse::<u16>().unwrap()))?;
    }
    Ok(())
}