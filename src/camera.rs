use opencv::{prelude::*, videoio::*};

pub fn get_camera_buf(cam : &mut VideoCapture, cam_qual : i32) -> Vec<u8>{        
        let mut frame = Mat::default();
        cam.read(&mut frame);

        let buf = Mat::data_bytes(&frame).unwrap();

        return buf.to_vec();
}