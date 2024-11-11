use crate::constants::*;
use regex::Regex;
use rustc_hash::FxHashSet as HashSet;

pub struct HS(HashSet<String>);

impl HS {
  pub fn new(vec_domain_str: Vec<String>) -> Self {
    let start_with_star = Regex::new(r"^\*\..+").unwrap();
    let re = Regex::new(&format!("{}{}{}", r"^", REGEXP_DOMAIN_OR_PREFIX, r"$")).unwrap(); // TODO: TODO:
    let hs: HashSet<String> = vec_domain_str
      .iter()
      .map(|d| if start_with_star.is_match(d) { &d[2..] } else { d })
      .filter(|x| re.is_match(x) || (x.split('.').count() == 1))
      .map(|y| y.to_string())
      .collect();
    HS(hs)
  }

  pub fn find_suffix_match(&self, query_domain: &str) -> bool {
    let nn_part: Vec<&str> = query_domain.split('.').collect();

    let parts_num = nn_part.len();
    if parts_num > 0 {
      for i in 0..parts_num {
        // println!("check suffix or exact {}", nn_part[i..parts_num].join("."));
        if self.0.contains(&nn_part[i..parts_num].join(".")) {
          // println!("domain suffix or exact domain found!: {}", query_domain);
          return true;
        }
      }
    }
    false
  }

  pub fn find_prefix_match(&self, query_domain: &str) -> bool {
    let nn_part: Vec<&str> = query_domain.split('.').collect();

    let parts_num = nn_part.len();
    if parts_num > 1 {
      for i in 1..parts_num {
        // println!("prefix {}.*", nn_part[0..parts_num - i].join("."));
        if self.0.contains(&format!("{}.*", nn_part[0..parts_num - i].join("."))) {
          // println!("domain prefix found!: {}", query_domain);
          return true;
        }
      }
    }
    false
  }
}
