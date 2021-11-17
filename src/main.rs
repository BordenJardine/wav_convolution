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

fn write_wav(buffer: &[i16], file_name: &str) {
  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int
  };
  let mut writer = hound::WavWriter::create(file_name, spec).unwrap();

  for sample in buffer {
    writer.write_sample(*sample).unwrap();
  }
}

pub fn load_wav(file_name: &str) -> Vec<i16> {
  println!("loading: {}", file_name);
  let mut reader = hound::WavReader::open(file_name).unwrap();
  let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
  samples
}
