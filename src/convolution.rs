use std::collections::VecDeque;

pub fn convolve(
  input_sample: f32,
  impulse_response: &[f32],
  history_buffer: &mut VecDeque<f32>
) -> f32 {
  for (i, kernal_sample) in impulse_response.iter().enumerate() {
    match history_buffer.get_mut(i) {
      Some(history_sample) => *history_sample += input_sample * (kernal_sample/2.0),
      None => return 0.0
      }
  }

  match history_buffer.pop_front() {
    Some(output) => {
      history_buffer.push_back(0.0);
      return output;
    },
    None => return 0.0
  }
}

pub fn convolve_signals(
  input_signal: &[f32],
  impulse_response: &[f32],
) -> Vec<f32> {
  let mut output_buffer: Vec<f32> = Vec::new();

  // queue to keep track of convolutions so far
  // starts off with a bunch of 0s
  let mut history_buffer: VecDeque<f32> = VecDeque::new();
  let history_len = impulse_response.len() + input_signal.len();
  for _ in 0..history_len {
    history_buffer.push_back(0.0);
  }

  println!("total {}:", input_signal.len());
  //convolve each sample with IR
  for sample in input_signal {
    output_buffer.push(convolve(
      *sample,
      impulse_response,
      &mut history_buffer
    ));
    print!(".");
  }

  output_buffer
}
