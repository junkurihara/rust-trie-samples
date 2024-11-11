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
      .map(|d| d.to_string())
      .collect::<Vec<String>>()
  }) else {
    panic!("Failed to read candidate keys")
  };

  // HashSet
  {
    let hashset = hashset::HS::new(vec_domain_str.clone());

    let start = std::time::Instant::now();
    for key in candidate_keys.iter() {
      hashset.find_suffix_match(key);
      hashset.find_prefix_match(key);
    }
    let end = start.elapsed();
    println!("HS: {:6}ns", end.subsec_nanos());
  }

  {
    let qp = qptrie::QP::new(vec_domain_str.clone());

    let start = std::time::Instant::now();
    for key in candidate_keys.iter() {
      qp.find_suffix_match(key);
      qp.find_prefix_match(key);
    }
    let end = start.elapsed();
    println!("QP: {:6}ns", end.subsec_nanos());
  }

  {
    let qp = qptrie::QP::new(vec_domain_str.clone());

    let start = std::time::Instant::now();
    for key in candidate_keys.iter() {
      qp.smart_suffix_match(key);
      qp.smart_prefix_match(key);
    }
    let end = start.elapsed();
    println!("QPS: {:6}ns", end.subsec_nanos());
  }

  {
    let cw = cedarwood::CW::new(vec_domain_str);
    let start = std::time::Instant::now();
    for key in candidate_keys.iter() {
      cw.find_suffix_match(key);
      cw.find_prefix_match(key);
    }
    let end = start.elapsed();
    println!("CW: {:6}ns", end.subsec_nanos());
  }
}
