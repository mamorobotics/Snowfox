use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::{str, thread, sync::*};
use pad::PadStr;

use atomic::{AtomicI32, Ordering};

mod camera;

const ADDR : &str = "192.168.1.2:8080";
const PORT : &u16 = &8080;

const WIDTH : f64 = 1280.0;
const HEIGHT : f64 = 720.0;
const FPS : f64 = 20.0;

fn main() -> std::io::Result<()> {
    {
        let cam_num = AtomicI32::new(1);
        let cam_qual = AtomicI32::new(100);

        let mut cam1 = camera::Camera::new(0, WIDTH, HEIGHT, FPS);
        let mut cam2 = camera::Camera::new(0, WIDTH, HEIGHT, FPS);

        let socket = Arc::new(UdpSocket::bind(ADDR)?);

        send_handshake(&socket);

        let sock = Arc::clone(&socket);

        thread::scope(|s| {
            s.spawn(|| {
                loop{
                    send_camera(&sock, &mut cam1, &mut cam2, cam_num.load(Ordering::Relaxed), cam_qual.load(Ordering::Relaxed)).unwrap();
                    //send_camera(&mut cam1, &mut cam2, 0, cam_qual.load(Ordering::Relaxed)).unwrap();
                }
            });
            s.spawn(||{
                loop
                {
                    let mut init_buf = [0; 32];
                    let (_amt, _src) = socket.recv_from(&mut init_buf).unwrap();

                    let msg = str::from_utf8(&init_buf).unwrap();

                    let size = &msg[..msg.rfind("!").unwrap()].parse::<i32>().unwrap(); 
                    let headers = &msg[msg.rfind("!").unwrap()..];

                    let mut data_buf = Vec::new();

                    let mut total_size = 0;
                    while total_size < size-1
                    {
                        let mut temp_buf = Vec::new();
                        temp_buf.resize(if (size - total_size) > 65500{65500} else {(size-total_size) as usize}, 0);
                        let (_amt, _src) = socket.recv_from(&mut temp_buf).unwrap();
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
                                    cam_num.store(msg_temp_val, Ordering::Relaxed);
                                }else if msg_temp_type == Some("qual") 
                                {
                                    cam_qual.store(msg_temp_val, Ordering::Relaxed);
                                }
                            }
                        }
                    }
                }
            });
        });
    }Ok(())
}

fn send_handshake(socket : &UdpSocket)
{
    socket.send_to("0110".as_bytes(), SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT)).unwrap();
}

fn send_camera(socket : &UdpSocket, cam1 : &mut camera::Camera, cam2 : &mut camera::Camera, cam_num : i32, cam_qual : i32) -> std::io::Result<()> {
//fn send_camera(cam1 : &mut camera::Camera, cam2 : &mut camera::Camera, cam_num : i32, cam_qual : i32) -> std::io::Result<()> {
    {
        let mut cam_buf1: Vec<u8>;
        let mut cam_buf2: Vec<u8>;

        let mut bytes1: String;
        let mut bytes2: String;

        (cam_buf1, bytes1) = cam1.get_camera_buf(cam_qual);
        (cam_buf2, bytes2) = cam2.get_camera_buf(cam_qual);

        let packet_size = 65500;

        println!("Send");

        if(cam_num == 1){
            let pre_msg = (bytes1 + "!" + "4").pad_to_width(32);
            let pre_msg_bytes = pre_msg.as_bytes();
            socket.send_to(&pre_msg_bytes, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
    
            println!("{}", pre_msg);
    
            if cam_buf1.len() > packet_size {
                while cam_buf1.len() > packet_size {
                    let temp: Vec<u8> = cam_buf1[..packet_size].to_vec();
                    cam_buf1 = cam_buf1[(packet_size+1)..].to_vec();
    
                    socket.send_to(&temp, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
                }
            }
            if cam_buf1.len() != 0 {
                socket.send_to(&cam_buf1, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
            }
        }else if(cam_num == 2){
            let pre_msg = (bytes2 + "!" + "4").pad_to_width(32);
            let pre_msg_bytes = pre_msg.as_bytes();
            socket.send_to(&pre_msg_bytes, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
    
            println!("{}", pre_msg);
    
            if cam_buf2.len() > packet_size {
                while cam_buf2.len() > packet_size {
                    let temp: Vec<u8> = cam_buf2[..packet_size].to_vec();
                    cam_buf2 = cam_buf2[(packet_size+1)..].to_vec();
    
                    socket.send_to(&temp, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
                }
            }
            if cam_buf2.len() != 0 {
                socket.send_to(&cam_buf2, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
            }
        }
    }
    Ok(())
}