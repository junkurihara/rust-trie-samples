use crate::utils::*;
use qp_trie::Trie;

pub struct QP {
  suffix_qp: Trie<qp_trie::wrapper::BString, ()>,
}

impl QP {
  pub fn new(vec_domain_str: Vec<String>) -> Self {
    // QP Trie for suffix shortest match
    let mut suffix_qp: Trie<qp_trie::wrapper::BString, ()> = Trie::new();
    for domain in vec_domain_str.into_iter().enumerate() {
      suffix_qp.insert_str(&reverse_string(&domain.1), ());
    }

    QP { suffix_qp }
  }

  pub fn find_suffix_match(&self, query_domain: &str) -> bool {
    let rev_qn = reverse_string(&query_domain);

    // longest common suffixを引くとsuffixより長いやつの存在しかわからないので、
    // 結局HashSetと同じような実装にならざるを得ない。
    // APIの問題なので、Crateを拡張すればよいっちゃよい。

    let nn_part: Vec<&str> = rev_qn.split('.').collect();
    let parts_num = nn_part.len();
    // let mut domain_to_match = common_domain_suffix[0].to_string();
    for idx in 0..parts_num {
      // let domain_to_match =
      // str_vec_to_domain(&common_domain_suffix[0..domain_parts_num - idx].to_vec());
      let domain_to_match = nn_part[0..idx + 1].join(".");
      println!("checking {}", domain_to_match);
      if self.suffix_qp.contains_key_str(&domain_to_match) {
        println!("domain suffix or exact domain found!: {}", query_domain);
        break;
      }

      // check longest common suffix with qptrie
      // let longest_common_suffix = self
      //   .suffix_qp
      //   .longest_common_prefix(rev_qn.as_bytes())
      //   .as_str();
      // let qp_end = qp_start.elapsed();

      // info!(
      //   "[Block] QP_LCP: {:6}ms経過しました。",
      //   qp_end.subsec_nanos() / 1_000
      // );
      // let qp_start = std::time::Instant::now(); // TODO:

      // // retrieve the exact domain part of the matched suffix
      // let vec_lcs_split: Vec<&str> = longest_common_suffix.split(".").collect();
      // let vec_rqn_split: Vec<&str> = rev_qn.split(".").collect();
      // let mut common_domain_suffix = vec![];
      // for i in 0..vec_lcs_split.len() {
      //   if vec_lcs_split[i] == vec_rqn_split[i] {
      //     common_domain_suffix.push(vec_lcs_split[i])
      //   }
      // }
      // let common_domain_suffix: Vec<&str> = vec_lcs_split
      //   .zip(vec_rqn_split)
      //   .filter(|(x, y)| x == y)
      //   .map(|(x, _)| x)
      //   .collect();

      // domain_to_match = format!(
      //   "{}.{}",
      //   domain_to_match.to_string(),
      //   common_domain_suffix[idx]
      // );
      // } else if self
      //   .suffix_qp
      //   .contains_key_str(&format!("{}.*", domain_to_match))
      // {
      //   info!("[Block] matched domain suffix rule: {}", domain_to_match);
      //   break;
      // }
    }
    false
  }
}
