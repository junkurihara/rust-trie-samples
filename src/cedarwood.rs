use crate::constants::*;
use crate::utils::*;
use cedarwood::Cedar;
use regex::Regex;

pub struct CW {
  prefix_cedar: Cedar,
  suffix_cedar: Cedar,
  prefix_dict: Vec<String>,
  suffix_dict: Vec<String>,
}

impl CW {
  pub fn new(vec_domain_str: Vec<String>) -> Self {
    let start_with_star = Regex::new(r"^\*\..+").unwrap();
    let end_with_star = Regex::new(r".+\.\*$").unwrap();
    // TODO: currently either one of prefix or suffix match with '*' is supported
    let re = Regex::new(&format!("{}{}{}", r"^", REGEXP_DOMAIN_OR_PREFIX, r"$")).unwrap();
    let dict: Vec<String> = vec_domain_str
      .iter()
      .map(|d| if start_with_star.is_match(d) { &d[2..] } else { d })
      .filter(|x| re.is_match(x) || (x.split('.').count() == 1))
      .map(|y| y.to_ascii_lowercase())
      .collect();
    let prefix_dict: Vec<String> = dict
      .iter()
      .filter(|d| end_with_star.is_match(d))
      .map(|d| d[..d.len() - 2].to_string())
      .collect();
    let suffix_dict: Vec<String> = dict
      .iter()
      .filter(|d| !end_with_star.is_match(d))
      .map(|d| reverse_string(d))
      .collect();

    let prefix_kv: Vec<(&str, i32)> = prefix_dict
      .iter()
      .map(AsRef::as_ref)
      .enumerate()
      .map(|(k, s)| (s, k as i32))
      .collect();
    let mut prefix_cedar = Cedar::new();
    prefix_cedar.build(&prefix_kv);

    let suffix_kv: Vec<(&str, i32)> = suffix_dict
      .iter()
      .map(AsRef::as_ref)
      .enumerate()
      .map(|(k, s)| (s, k as i32))
      .collect();
    let mut suffix_cedar = Cedar::new();
    suffix_cedar.build(&suffix_kv);

    CW {
      prefix_cedar,
      suffix_cedar,
      prefix_dict,
      suffix_dict,
    }
  }

  pub fn find_suffix_match(&self, query_domain: &str) -> bool {
    let rev_nn = reverse_string(query_domain);
    let matched_items = self
      .suffix_cedar
      .common_prefix_iter(&rev_nn)
      .map(|(x, _)| self.suffix_dict[x as usize].clone());

    let mut matched_as_domain = matched_items.filter(|found| {
      if found.len() == rev_nn.len() {
        true
      } else if let Some(nth) = rev_nn.chars().nth(found.chars().count()) {
        nth.to_string() == "."
      } else {
        false
      }
    });
    matched_as_domain.next().is_some()
    // if let Some(d) = matched_as_domain.next() {
    //   // println!("[with cw] domain suffix or exact domain found!: {} ({})", query_domain, d);
    //   true
    // } else {
    //   false
    // }
  }

  pub fn find_prefix_match(&self, query_domain: &str) -> bool {
    let matched_items = self
      .prefix_cedar
      .common_prefix_iter(query_domain)
      .map(|(x, _)| self.prefix_dict[x as usize].clone());

    let mut matched_as_domain = matched_items.filter(|found| {
      if let Some(nth) = query_domain.chars().nth(found.chars().count()) {
        nth.to_string() == "."
      } else {
        false
      }
    });
    matched_as_domain.next().is_some()
    // if let Some(d) = matched_as_domain.next() {
    //   // println!("[with cw] domain prefix found!: {} ({})", query_domain, d);
    //   true
    // } else {
    //   false
    // }
  }
}
