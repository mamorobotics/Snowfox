use opencv::{prelude::*, videoio::*};
use opencv::videoio::{self, VideoCapture, *};

pub struct Camera {
    camera : videoio::VideoCapture,
    width : f64, height : f64, fps : f64
}

pub impl Camera {
    pub fn new(&mut self, id : i8, width : f64, height : f64, fps : f64) {
        let mut cam = videoio::VideoCapture::new(id, videoio::CAP_ANY).unwrap();
        let _ = cam.set(CAP_PROP_FRAME_WIDTH, WIDTH);
        let _ = cam.set(CAP_PROP_FRAME_HEIGHT, HEIGHT);
        let _ = cam.set(CAP_PROP_FPS, FPS);
        let _ = cam.set(CAP_PROP_FOURCC, f64::from(VideoWriter::fourcc('M', 'J', 'P', 'G').unwrap()));
        Self {cam, width, height, fps};
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
