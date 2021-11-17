pub fn convolve_signals(
  input_signal: &[i16],
  impulse_response: &[i16],
) -> Vec<i16> {
  let mut output_buffer: Vec<i16> = Vec::new();

  // queue to keep track of convolutions so far
  // starts off with a bunch of 0s
  let out_len = impulse_response.len() + input_signal.len();
  for _ in 0..out_len {
    output_buffer.push(0);
  }

  println!("total: {}", input_signal.len());
  //convolve each sample with IR
  for (i, input_sample) in input_signal.iter().enumerate() {
    let s = input_sample / 64;
    for (j, kernal_sample) in impulse_response.iter().enumerate() {
      output_buffer[i+j] = output_buffer[i+j].saturating_add(s.saturating_mul(*kernal_sample / 1024));
    }
    if i > 0 && 128 % i == 0 {
      print!(".");
    }
  }
  output_buffer
}
