use opencv::{prelude::*, videoio::*};

pub fn get_camera_buf(cam : &mut VideoCapture, cam_qual : i32) -> Vec<u8>{      
        let mut frame = Mat::default();
        
        let mut res = cam.read(&mut frame).unwrap();
        while !res{
            res = cam.read(&mut frame).unwrap();
        }

        let buf = Mat::data_bytes(&frame).unwrap();

        return buf.to_vec();
}