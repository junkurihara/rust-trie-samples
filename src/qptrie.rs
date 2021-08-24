use crate::constants::*;
use crate::utils::*;
use qp_trie::Trie;
use regex::Regex;

pub struct QP {
  suffix_qp: Trie<qp_trie::wrapper::BString, ()>,
  prefix_qp: Trie<qp_trie::wrapper::BString, ()>,
}

impl QP {
  pub fn new(vec_domain_str: Vec<String>) -> Self {
    let start_with_star = Regex::new(r"^\*\..+").unwrap();
    let end_with_star = Regex::new(r".+\.\*$").unwrap();
    let re = Regex::new(&format!("{}{}{}", r"^", REGEXP_DOMAIN_OR_PREFIX, r"$")).unwrap(); // TODO: TODO:
    let vec_domain_str_cleaned: Vec<String> = vec_domain_str
      .iter()
      .map(|d| {
        if start_with_star.is_match(d) {
          &d[2..]
        } else {
          d
        }
      })
      .filter(|x| re.is_match(x) || (x.split('.').collect::<Vec<&str>>().len() == 1))
      .map(|y| y.to_string())
      .collect();

    // QP Trie for shortest pattern match
    let mut prefix_qp: Trie<qp_trie::wrapper::BString, ()> = Trie::new();
    let mut suffix_qp: Trie<qp_trie::wrapper::BString, ()> = Trie::new();
    for domain in vec_domain_str_cleaned.iter() {
      if end_with_star.is_match(domain) {
        prefix_qp.insert_str(domain, ());
      } else {
        suffix_qp.insert_str(&reverse_string(domain), ());
      }
    }

    QP {
      prefix_qp,
      suffix_qp,
    }
  }

  pub fn smart_suffix_match(&self, query_domain: &str) -> bool {
    let rev_nn = reverse_string(query_domain);
    let rev_nn_part: Vec<&str> = rev_nn.split(".").collect();

    // 先にマッチする部分だけ取り出してしまう
    let lcs = self
      .suffix_qp
      .longest_common_prefix(rev_nn.as_bytes())
      .as_str();
    let mut lcs_part: Vec<&str> = vec![];
    for (i, s) in lcs.split(".").into_iter().enumerate() {
      if s != rev_nn_part[i] {
        break;
      }
      lcs_part.push(s);
    }

    let parts_num = lcs_part.len();
    if parts_num > 0 {
      for i in 0..parts_num {
        // println!("suffix or exact {}", lcs_part[0..parts_num - i].join("."));
        if self
          .suffix_qp
          .contains_key_str(&lcs_part[0..parts_num - i].join("."))
        {
          println!(
            "[with lcs] domain suffix or exact domain found!: {}",
            query_domain
          );
          return true;
        }
      }
    }

    false
  }

  pub fn smart_prefix_match(&self, query_domain: &str) -> bool {
    let nn_part: Vec<&str> = query_domain.split(".").collect();

    // 先にマッチする部分だけ取り出してしまう
    let lcp = self
      .prefix_qp
      .longest_common_prefix(query_domain.as_bytes())
      .as_str();
    let mut lcp_part: Vec<&str> = vec![];
    for (i, p) in lcp.split(".").into_iter().enumerate() {
      if p != nn_part[i] {
        break;
      }
      lcp_part.push(p);
    }

    let parts_num = lcp_part.len();
    if parts_num > 0 {
      for i in 0..parts_num {
        if self
          .prefix_qp
          .contains_key_str(&format!("{}.*", lcp_part[0..parts_num - i].join(".")))
        {
          println!("[with lcs] domain prefix found!: {}", query_domain);
          return true;
        }
      }
    }

    false
  }

  pub fn find_suffix_match(&self, query_domain: &str) -> bool {
    // longest common suffixを引くとsuffixより長いやつの存在しかわからないので、
    // 結局HashSetと同じような実装にならざるを得ない。
    // APIの問題なので、Crateを拡張すればよいが。
    let rev_nn = reverse_string(query_domain);
    let rev_nn_part: Vec<&str> = rev_nn.split(".").collect();

    let parts_num = rev_nn_part.len();
    if parts_num > 0 {
      for i in 0..parts_num {
        // println!(
        //   "suffix or exact {}",
        //   rev_nn_part[0..parts_num - i].join(".")
        // );
        if self
          .suffix_qp
          .contains_key_str(&rev_nn_part[0..parts_num - i].join("."))
        {
          println!("domain suffix or exact domain found!: {}", query_domain);
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
        if self
          .prefix_qp
          .contains_key_str(&format!("{}.*", nn_part[0..parts_num - i].join(".")))
        {
          println!("domain prefix found!: {}", query_domain);
          return true;
        }
      }
    }
    false
  }

  // pub fn find_suffix_match(&self, query_domain: &str) -> bool {
  //   // let rev_qn = reverse_string(&query_domain);

  //   let nn_part: Vec<&str> = rev_qn.split('.').collect();
  //   let parts_num = nn_part.len();
  //   // let mut domain_to_match = common_domain_suffix[0].to_string();
  //   for idx in 0..parts_num {
  //     // let domain_to_match =
  //     // str_vec_to_domain(&common_domain_suffix[0..domain_parts_num - idx].to_vec());
  //     let domain_to_match = nn_part[0..idx + 1].join(".");
  //     println!("checking {}", domain_to_match);
  //     if self.suffix_qp.contains_key_str(&domain_to_match) {
  //       println!("domain suffix or exact domain found!: {}", query_domain);
  //       break;
  //     }

  //     // check longest common suffix with qptrie
  //     // let longest_common_suffix = self
  //     //   .suffix_qp
  //     //   .longest_common_prefix(rev_qn.as_bytes())
  //     //   .as_str();
  //     // let qp_end = qp_start.elapsed();

  //     // info!(
  //     //   "[Block] QP_LCP: {:6}ms経過しました。",
  //     //   qp_end.subsec_nanos() / 1_000
  //     // );
  //     // let qp_start = std::time::Instant::now(); // TODO:

  //     // // retrieve the exact domain part of the matched suffix
  //     // let vec_lcs_split: Vec<&str> = longest_common_suffix.split(".").collect();
  //     // let vec_rqn_split: Vec<&str> = rev_qn.split(".").collect();
  //     // let mut common_domain_suffix = vec![];
  //     // for i in 0..vec_lcs_split.len() {
  //     //   if vec_lcs_split[i] == vec_rqn_split[i] {
  //     //     common_domain_suffix.push(vec_lcs_split[i])
  //     //   }
  //     // }
  //     // let common_domain_suffix: Vec<&str> = vec_lcs_split
  //     //   .zip(vec_rqn_split)
  //     //   .filter(|(x, y)| x == y)
  //     //   .map(|(x, _)| x)
  //     //   .collect();

  //     // domain_to_match = format!(
  //     //   "{}.{}",
  //     //   domain_to_match.to_string(),
  //     //   common_domain_suffix[idx]
  //     // );
  //     // } else if self
  //     //   .suffix_qp
  //     //   .contains_key_str(&format!("{}.*", domain_to_match))
  //     // {
  //     //   info!("[Block] matched domain suffix rule: {}", domain_to_match);
  //     //   break;
  //     // }
  //   }
  //   false
  // }
}
