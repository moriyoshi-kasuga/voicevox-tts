use std::{io::Write, time::Instant};
use vvcore::*;

fn main() {
    let time = Instant::now();

    let dir = std::ffi::CString::new("./voicevox_core/open_jtalk_dic_utf_8-1.11").unwrap();
    let vvc =
        VoicevoxCore::new_from_options(AccelerationMode::Auto, 0, true, dir.as_c_str()).unwrap();

    let end = time.elapsed();
    println!(
        "{}.{:03}秒経過しました。",
        end.as_secs(),
        end.subsec_millis()
    );

    println!("loaded vvc");

    let time = Instant::now();

    let text: &str = "こんにちは";
    let speaker: u32 = 1;
    let wav = vvc.tts_simple(text, speaker).unwrap();

    let end = time.elapsed();
    println!(
        "{}.{:03}秒経過しました。",
        end.as_secs(),
        end.subsec_millis()
    );

    let mut file = std::fs::File::create("audio.wav").unwrap();
    file.write_all(wav.as_slice()).unwrap();
}
