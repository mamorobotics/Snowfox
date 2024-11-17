use opencv::{prelude::*, videoio::*, core::*, imgcodecs::*};
use opencv::videoio::{self, VideoCapture};

pub struct Camera {
    camera : VideoCapture
}

impl Camera {
    pub fn new(id : i32, width : f64, height : f64, fps : f64) -> Self{
        let mut camera : VideoCapture = VideoCapture::new(id, videoio::CAP_ANY).unwrap();
        let _ = camera.set(CAP_PROP_FRAME_WIDTH, width);
        let _ = camera.set(CAP_PROP_FRAME_HEIGHT, height);
        //let _ = camera.set(CAP_PROP_FPS, fps);
        return Self {camera};
    }

    pub fn get_camera_buf(&mut self, cam_qual : i32) -> (Vec<u8>, String){
        let mut frame = Mat::default();
        
        let mut res = self.camera.read(&mut frame).unwrap();
        while !res{
            res = self.camera.read(&mut frame).unwrap();
        }
        
        let mut buf = opencv::core::Vector::new();
        let mut params = Vector::new();
        params.push(IMWRITE_JPEG_QUALITY);
        params.push(cam_qual);
        imencode(".jpg", &frame, &mut buf, &params).unwrap();

        //let mut buf = Mat::data_bytes(&frame).unwrap().to_vec();

        //let mut step = frame.step1(0).unwrap() as usize;
        //let mut rows = frame.rows() as usize;
        //let mut bytes = step  * rows;
        let mut bytes = buf.to_vec().len();

        println!("Bytes: {}", bytes);

        (buf.to_vec(), bytes.to_string())
    }
}
