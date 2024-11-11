mod cedarwood;
mod constants;
mod hashset;
mod qptrie;
mod utils;
use std::fs;

const DOMAIN_LIST_PATH: &str = "./.private/blocklist.txt";

fn main() {
  let vec_domain_str: Vec<String> = if let Ok(content) = fs::read_to_string(DOMAIN_LIST_PATH) {
    content.split('\n').filter(|c| !c.is_empty()).map(|d| d.to_string()).collect()
  } else {
    panic!("Failed to read domain list")
  };
  let Ok(candidate_keys) = fs::read_to_string("./.private/domains.txt").map(|c| {
    c.split('\n')
      .filter(|c| !c.is_empty())
      .map(|d| {
        let mut d = d.to_string().to_ascii_lowercase();
        let _ = d.pop(); // remove trailing dot
        d
      })
      .collect::<Vec<String>>()
  }) else {
    panic!("Failed to read candidate keys")
  };

  // HashSet
  {
    let mut cnt = 0;
    let hashset = hashset::HS::new(vec_domain_str.clone());

    let start = std::time::Instant::now();
    for key in candidate_keys.iter() {
      if hashset.find_suffix_match(key) {
        cnt += 1;
        continue;
      };
      if hashset.find_prefix_match(key) {
        cnt += 1;
      };
    }
    let end = start.elapsed();
    println!("HS: {:6}ms", end.subsec_micros());
    println!("HS: blocked {:6} domains", cnt);
  }

  {
    let mut cnt = 0;
    let qp = qptrie::QP::new(vec_domain_str.clone());

    let start = std::time::Instant::now();
    for key in candidate_keys.iter() {
      if qp.find_suffix_match(key) {
        cnt += 1;
        continue;
      };
      if qp.find_prefix_match(key) {
        cnt += 1;
      };
    }
    let end = start.elapsed();
    println!("QP: {:6}ms", end.subsec_micros());
    println!("QP: blocked {:6} domains", cnt);
  }

  {
    let mut cnt = 0;
    let qp = qptrie::QP::new(vec_domain_str.clone());

    let start = std::time::Instant::now();
    for key in candidate_keys.iter() {
      if qp.smart_suffix_match(key) {
        cnt += 1;
        continue;
      };
      if qp.smart_prefix_match(key) {
        cnt += 1;
      };
    }
    let end = start.elapsed();
    println!("QPS: {:6}ms", end.subsec_micros());
    println!("QPS: blocked {:6} domains", cnt);
  }

  {
    let mut cnt = 0;
    let cw = cedarwood::CW::new(vec_domain_str);
    let start = std::time::Instant::now();
    for key in candidate_keys.iter() {
      if cw.find_suffix_match(key) {
        cnt += 1;
        continue;
      };
      if cw.find_prefix_match(key) {
        cnt += 1;
      };
    }
    let end = start.elapsed();
    println!("CW: {:6}ms", end.subsec_micros());
    println!("CW: blocked {:6} domains", cnt);
  }
}
