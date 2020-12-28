pub fn variance(_input: &[f64]) -> f64 {
  let input_size = _input.len() as f64;
  let mean = _input.iter().fold(0.0, |acc, v| acc + v) / input_size;

  return _input.iter().fold(0.0, |acc, v| {
    let value = v - mean;
    return acc + value * value;
  }) / input_size;
}

pub fn std_dev(_input: &[f64]) -> f64 {
  return variance(_input).sqrt();
}
