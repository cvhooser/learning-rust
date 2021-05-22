use std::{env, fs, io, collections::HashMap};

fn main() -> std::io::Result<()> {
    let mut inputs: Vec<String> = Vec::new();
    let mut input = String::new();
    if let Ok(_n) = io::stdin().read_line(&mut input) {
      input.truncate(input.len() - 1);
      // input.pop();
      inputs.push(input);
    }

    input = String::new();
    if let Ok(_n) = io::stdin().read_line(&mut input) {
      // input.truncate(input.len() - 1);
      // input.pop()
      inputs.push(input);
    }

    let matching_pairs = sock_merchant(
      inputs[0].parse::<i32>().unwrap(), 
      inputs[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
    );

    let filepath = env::var("OUTPUT_PATH").unwrap();
    fs::File::create(&filepath)?;
    fs::write(&filepath, matching_pairs.to_string())?;

    Ok(())
}

fn sock_merchant(_: i32, ar: Vec<i32>) -> i32 {
  // println!("{:#?}", &ar);
  let mut pairs = 0;

  let mut socks = HashMap::new();

  for num in ar {
    // println!("{}", num);
    let count = socks.entry(num).or_insert(0);
    *count += 1;
  }
  
  for (&k, &v) in &socks {
    pairs += (f64::from(v) / 2.0).floor() as i32;
    // println!("k={}, v={}, pairs={}", k, v, pairs);
  }

  pairs
}
