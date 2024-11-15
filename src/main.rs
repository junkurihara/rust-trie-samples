mod cedarwood;
mod constants;
mod hashset;
mod qptrie;
mod utils;
use std::{
  fs,
  sync::{Arc, RwLock},
};

const DOMAIN_LIST_PATH: &str = "./.private/blocklist.txt";

#[tokio::main]
async fn main() {
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
    let cw = cedarwood::CW::new(vec_domain_str.clone());
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

  {
    // Instantiating the cedarwood takes time so we use the clone of the base instance
    let cw_base = RwLock::new(Arc::new(cedarwood::CW::new(vec_domain_str.clone())));

    let cnt = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut handle_vec: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    let start = std::time::Instant::now();
    for key in candidate_keys.iter() {
      let cw = cw_base.read().unwrap().clone();
      let key = key.clone();
      let cnt_clone = cnt.clone();
      let handle = tokio::spawn(async move {
        if cw.find_suffix_match(&key) {
          cnt_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
          return;
        };
        if cw.find_prefix_match(&key) {
          cnt_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        };
      });
      handle_vec.push(handle);
    }

    // wait for all tasks to finish
    let _ = futures::future::join_all(handle_vec).await;

    let end = start.elapsed();
    println!("CW Spawn Thread: {:6}ms", end.subsec_micros());
    println!(
      "CW Spawn Thread: blocked {:6} domains",
      cnt.load(std::sync::atomic::Ordering::Relaxed)
    );
  }

  {
    let size = 128usize;
    // Instantiating the cedarwood takes time so we use the clone of the base instance
    let cw_base = cedarwood::CW::new(vec_domain_str.clone());
    let nested_list: Vec<Arc<RwLock<cedarwood::CW>>> = (0..size)
      .map(|i| {
        if (i + 1) % 16 == 0 {
          println!("Instantiating CW: {}/{}", i + 1, size)
        };
        Arc::new(RwLock::new(cw_base.clone()))
      })
      .collect();

    let cnt = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut handle_vec: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    let start = std::time::Instant::now();
    for (i, key) in candidate_keys.iter().enumerate() {
      let cw = nested_list[i % size].clone();
      let key = key.clone();
      let cnt_clone = cnt.clone();
      let handle = tokio::spawn(async move {
        let cw = cw.read().unwrap();
        if cw.find_suffix_match(&key) {
          cnt_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
          return;
        };
        if cw.find_prefix_match(&key) {
          cnt_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        };
      });
      handle_vec.push(handle);
    }

    // wait for all tasks to finish
    let _ = futures::future::join_all(handle_vec).await;

    let end = start.elapsed();
    println!("CW Spawn Thread {size}: {:6}ms", end.subsec_micros());
    println!(
      "CW Spawn Thread {size}: blocked {:6} domains",
      cnt.load(std::sync::atomic::Ordering::Relaxed)
    );
  }
}
