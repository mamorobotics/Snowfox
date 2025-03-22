use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::{str, thread, sync::*};
use pad::PadStr;

use atomic::{AtomicI32, Ordering};

mod camera;

const ADDR : &str = "192.168.1.2:8080";
const PORT : &u16 = &8080;

const MULT : f64 = 80.0;
const WIDTH : f64 = 16.0*MULT;
const HEIGHT : f64 = 9.0*MULT;

fn main() -> std::io::Result<()> {
    {
        let cam_num = AtomicI32::new(1);
        let cam_qual = AtomicI32::new(80);

        let mut cam1 = camera::Camera::new(2, WIDTH, HEIGHT);
        let mut cam2 = camera::Camera::new(0, WIDTH, HEIGHT);

        let socket = Arc::new(UdpSocket::bind(ADDR)?);

        send_handshake(&socket);

        let sock = Arc::clone(&socket);

        //Camera sender
        thread::scope(|s| {
            s.spawn(|| {
                loop{
                    // println!("{}", camera::Camera::get_index(&mut cam));
                    // println!("{}", cam_num.load(Ordering::Relaxed));
                    // println!("");

                    if cam_num.load(Ordering::Relaxed) ==  1{
                        send_camera(&sock, &mut cam1, cam_qual.load(Ordering::Relaxed)).unwrap();
                    }else if cam_num.load(Ordering::Relaxed) == 0{
                        send_camera(&sock, &mut cam2, cam_qual.load(Ordering::Relaxed)).unwrap();
                    }
                }
            });
            //Cam num and quality reciver
            s.spawn(||{
                loop
                {
                    let mut init_buf = [0; 32];
                    let (_amt, _src) = socket.recv_from(&mut init_buf).unwrap();

                    let msg = str::from_utf8(&init_buf).unwrap();
                    println!("{}", msg);

                    let size = &msg[..msg.rfind("!").unwrap()].parse::<i32>().unwrap();
                    let headers = &msg[msg.rfind("!").unwrap()+1..];

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
                        println!("{}", head);
                        if head == "6"{
                            let data = str::from_utf8(&data_buf).unwrap();
                            println!("{}", data);
                            for msg_temp_all in data.split("?"){
                                let msg_temp : Vec<_> = msg_temp_all.split("!").collect();
                                let msg_temp_type = msg_temp.clone().into_iter().nth(0);
                                let msg_temp_val = msg_temp.clone().into_iter().nth(1).expect("value").parse::<i32>().unwrap();
                                if msg_temp_type == Some("cam")
                                {
                                    cam_num.store(msg_temp_val, Ordering::Relaxed);

                                                    println!("{}", msg_temp_val);
                                }else if msg_temp_type == Some("qual")
                                {
                                    cam_qual.store(msg_temp_val, Ordering::Relaxed);
                                    println!("{}", msg_temp_val);
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
    println!("sent handshake");
}

fn send_camera(socket : &UdpSocket, cam : &mut camera::Camera, cam_qual : i32) -> std::io::Result<()> {
    {
        let mut cam_buf: Vec<u8>;

        let bytes: String;

        let packet_size = 1472;

        (cam_buf, bytes) = cam.get_single_camera_buf(cam_qual);
        let pre_msg = (bytes + "!" + "4").pad_to_width(32);
        let pre_msg_bytes = pre_msg.as_bytes();
        socket.send_to(&pre_msg_bytes, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;

        if cam_buf.len() > packet_size {
            while cam_buf.len() > packet_size {
                let temp: Vec<u8> = cam_buf[..packet_size].to_vec();
                cam_buf = cam_buf[(packet_size)..].to_vec();

                socket.send_to(&temp, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
            }
        }
        if cam_buf.len() != 0 {
            socket.send_to(&cam_buf, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
        }
    }
    Ok(())
}

fn send_camera_both(socket : &UdpSocket, cam1 : &mut camera::Camera, cam2 : &mut camera::Camera, cam_qual : i32) -> std::io::Result<()> {
    {
        let mut cam_buf: Vec<u8>;

        let bytes: String;

        let packet_size = 65500;

            (cam_buf, bytes) = cam1.get_single_camera_buf(cam_qual);
        let pre_msg = (bytes + "!" + "4").pad_to_width(32);
        let pre_msg_bytes = pre_msg.as_bytes();
        socket.send_to(&pre_msg_bytes, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;

        if cam_buf.len() > packet_size {
            while cam_buf.len() > packet_size {
                let temp: Vec<u8> = cam_buf[..packet_size].to_vec();
                cam_buf = cam_buf[(packet_size+1)..].to_vec();

                socket.send_to(&temp, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
            }
        }
        if cam_buf.len() != 0 {
            socket.send_to(&cam_buf, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), *PORT))?;
        }
    }
    Ok(())
}
