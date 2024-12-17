use opencv::{prelude::*, videoio::*, core::*, imgcodecs::*};
use opencv::videoio::{self, VideoCapture};

pub struct Camera {
    camera : VideoCapture,
    id : i32,
    width : f64,
    height : f64
}

impl Camera {
    pub fn get_index(&mut self) -> i32{
	    return self.id;
    }
    pub fn new(id : i32, width : f64, height : f64) -> Self{
        let mut camera : VideoCapture = VideoCapture::new(id, videoio::CAP_ANY).unwrap();
        let _ = camera.set(CAP_PROP_FRAME_HEIGHT, height);
        let _ = camera.set(CAP_PROP_FRAME_WIDTH, width);

        return Self{camera, id, width, height};
    }

    pub fn get_single_camera_buf(&mut self, cam_qual : i32) -> (Vec<u8>, String){
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

        let bytes = buf.to_vec().len();

        (buf.to_vec(), bytes.to_string())
    }
    pub fn new_index(&mut self, id : i32) -> Self{
	    let mut camera : VideoCapture = VideoCapture::new(id, videoio::CAP_ANY).unwrap();

        let _ = camera.set(CAP_PROP_FRAME_WIDTH, self.width);
        let _ = camera.set(CAP_PROP_FRAME_HEIGHT, self.height);

        let width = self.width;
        let height = self.height;
        
        return Self{camera, id, width, height};
    }
}
