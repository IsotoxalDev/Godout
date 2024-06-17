use godot::{
    prelude::*,
    classes::{SpriteFrames, file_access::ModeFlags, Image, ImageTexture},
    engine::{GFile, image::Format},
};
use std::io::ErrorKind;

#[derive(GodotClass)]
#[class(base=SpriteFrames, init)]
struct GodoutSpriteFramesGIF {
    base: Base<SpriteFrames>,
}


#[godot_api]
impl GodoutSpriteFramesGIF {
    
    #[func]
    fn load(&mut self, path: GString) -> u8 {
        if &String::from(&path)[(&path.len()-4)..] != ".gif" {return 9}
        let file = match GFile::open(path, ModeFlags::READ) {
            Ok(t) => t,
            Err(e) => {
                return match e.kind() {
                    ErrorKind::NotFound => 7,
                    ErrorKind::PermissionDenied => 10,
                    _ => 1
                }
            }
        };
        let mut decoder = gif::DecodeOptions::new();
        decoder.set_color_output(gif::ColorOutput::RGBA);
        let mut decoder = decoder.read_info(file).unwrap();
        let mut normal_delay: Option<f32> = None;
        let mut image: Option<Gd<Image>> = None;
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            if let None = normal_delay {normal_delay = Some(frame.delay as f32)};
            if let Some(img) = image.clone() {
                let mut i = img.clone();
                match frame.dispose {
                    gif::DisposalMethod::Keep => {
                            let img_size = Vector2i::new(frame.width as i32, frame.height as i32);
                            i.blit_rect(
                                Image::create_from_data(
                                        frame.width as i32,
                                        frame.height as i32,
                                        false, Format::RGBA8,
                                        frame.buffer.clone().into_owned()[..].into()
                                    ).unwrap(),
                                Rect2i::new(Vector2i::ZERO, img_size),
                                Vector2i::new(frame.left as i32, frame.top as i32)
                            );
                        },
                    _ => {
                        i.set_data(
                            frame.width as i32,
                            frame.height as i32,
                            false, Format::RGBA8,
                            frame.buffer.clone().into_owned()[..].into()
                        );
                    }
                }
                image = Some(i)
            }
            else {
                image = Image::create_from_data(
                    frame.width as i32,
                    frame.height as i32,
                    false, Format::RGBA8,
                    frame.buffer.clone().into_owned()[..].into()
                )
            }
                
            self.base_mut().add_frame_ex(
                "default".into(),
                if let Some(texture) = ImageTexture::create_from_image(image.clone().unwrap()) {texture.upcast()}
                else {return 16}
            )
                .duration(frame.delay as f32/normal_delay.unwrap()).done();
            self.base_mut().set_animation_speed("default".into(), 100./frame.delay as f64);
        }
        0
    }

    #[func]
    fn save(&self, path: GString) -> u8 {
        if &String::from(&path)[(&path.len()-4)..] != ".gif" {return 9}
        let file = match GFile::open(path, ModeFlags::WRITE) {
            Ok(t) => t,
            Err(e) => {
                return match e.kind() {
                    ErrorKind::PermissionDenied => 10,
                    _ => 1
                }
            }
        };
        0
    }
}

// Entry point
struct GodoutAnimationGIFExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodoutAnimationGIFExtension {}
