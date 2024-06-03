use opencv::{prelude::*, videoio::*};
use opencv::videoio::{self, VideoCapture};

pub struct Camera {
    camera : VideoCapture
}

impl Camera {
    pub fn new(id : i32, width : f64, height : f64, fps : f64) -> Self{
        let mut camera : VideoCapture = VideoCapture::new(id, videoio::CAP_ANY).unwrap();
        let _ = camera.set(CAP_PROP_FRAME_WIDTH, width);
        let _ = camera.set(CAP_PROP_FRAME_HEIGHT, height);
        let _ = camera.set(CAP_PROP_FPS, fps);
        let _ = camera.set(CAP_PROP_FOURCC, f64::from(VideoWriter::fourcc('M', 'J', 'P', 'G').unwrap()));
        return Self {camera};
    }

    pub fn get_camera_buf(&mut self, cam_qual : i32) -> Vec<u8>{      
        let mut frame = Mat::default();
        
        let mut res = self.camera.read(&mut frame).unwrap();
        while !res{
            res = self.camera.read(&mut frame).unwrap();
        }

        let buf = Mat::data_bytes(&frame).unwrap();

        return buf.to_vec();
    }
}
