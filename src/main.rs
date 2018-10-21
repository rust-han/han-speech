#![cfg(any(target_os = "macos", target_os = "ios"))]

extern crate rand;
extern crate coreaudio;

use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat};
use coreaudio::audio_unit::render_callback::{self, data};

use std::iter::{ Iterator, IntoIterator };
use std::f64::consts::PI;

pub const SAMPLE_HZ: f64 = 44_100.0;


#[derive(Debug)]
pub struct WhiteNoise;

impl WhiteNoise {
    pub fn new() -> WhiteNoise {
        WhiteNoise { }
    }
}

impl Iterator for WhiteNoise {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        Some( rand::random::<f64>() * 0.15 )
    }
}

#[derive(Debug)]
pub struct SineWave {
    val: f64,
}

impl SineWave {
    pub fn new(val: f64) -> SineWave {
        SineWave { val: val }
    }
}

impl Iterator for SineWave {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        // 男性均值 200
        // 女性均值 400
        self.val += 200.0 / SAMPLE_HZ;

        let phase = (self.val * PI * 2.0).sin() * 0.15;
        
        Some(phase)
    }
}

fn play<S: 'static + Iterator<Item = f64>>(mut samples: S) {
    let mut audio_unit = AudioUnit::new(IOType::DefaultOutput).unwrap();
    let stream_format = audio_unit.output_stream_format().unwrap();
    assert!(SampleFormat::F32 == stream_format.sample_format);

    type Args = render_callback::Args<data::NonInterleaved<f32>>;
    audio_unit.set_render_callback(move |args| {
        let Args { num_frames, mut data, .. } = args;
        for i in 0..num_frames {
            let sample = samples.next().unwrap();
            for channel in data.channels_mut() {
                channel[i] = sample as f32;
            }
        }
        Ok(())
    }).unwrap();

    audio_unit.start().expect("播放失败！");
    std::thread::sleep(std::time::Duration::from_millis(2000));
    audio_unit.stop().expect("停止播放失败！");
}

fn main() {
    println!("play white noise ...");
    play(WhiteNoise::new());

    println!("play sine wave ...");
    play(SineWave::new(0.0));


}