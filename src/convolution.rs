use std::collections::VecDeque;

pub fn convolve(
  input_sample: i16,
  impulse_response: &[i16],
  history_buffer: &mut VecDeque<i128>
  ) -> i16 {
  let inp = input_sample as i128;
  for (i, kernal_sample) in impulse_response.iter().enumerate() {
    match history_buffer.get_mut(i) {
      Some(history_sample) => *history_sample += inp * (*kernal_sample as i128),
      None => return 0
    }
  }

  match history_buffer.pop_front() {
    Some(output) => {
      history_buffer.push_back(0);
      return output as i16;
    },
    None => return 0
  }
}

pub fn convolve_signals(
  input_signal: &[i16],
  impulse_response: &[i16],
  ) -> Vec<i128> {
  let mut output_buffer: Vec<i128> = Vec::new();

  // queue to keep track of convolutions so far
  // starts off with a bunch of 0s
  let out_len = impulse_response.len() + input_signal.len();
  for _ in 0..out_len {
    output_buffer.push(0);
  }

  println!("total {}:", input_signal.len());
  //convolve each sample with IR
  for (i, input_sample) in input_signal.iter().enumerate() {
    let inp = *input_sample as i128;
    for (j, kernal_sample) in impulse_response.iter().enumerate() {
      output_buffer[i+j] += inp * (*kernal_sample as i128);
    }
    print!(".");
  }

  output_buffer
  }
