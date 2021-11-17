use std::collections::VecDeque;
use std::env;

mod convolution;
use convolution::convolve_signals;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = &args[1];
    let ir_file = &args[2];
    let output_file = &args[3];

    let input_signal = load_wav(input_file);
    let impulse_response = load_wav(ir_file);
    let out = &convolve_signals(&input_signal, &impulse_response);
    write_wav(out, output_file);
}

fn write_wav(buffer: &[f32], file_name: &str) {
  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 32,
    sample_format: hound::SampleFormat::Float
  };
  let mut writer = hound::WavWriter::create(file_name, spec).unwrap();

  for sample in buffer {
    writer.write_sample(*sample).unwrap();
  }
}

pub fn load_wav(file_name: &str) -> Vec<f32> {
  println!("loading: {}", file_name);
  let mut reader = hound::WavReader::open(file_name).unwrap();
  let samples: Vec<f32> = reader.samples().map(|s| s.unwrap()).collect();
  samples
}
