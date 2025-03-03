use napi::bindgen_prelude::*;
use napi_derive::napi;
use strsim::{jaro_winkler, levenshtein};
use deunicode::deunicode;



fn normalize(s: &str) -> String {
  deunicode(s).to_lowercase()
}

#[napi(object)]
#[derive(Clone)]
pub struct BestMatch {
  pub string: String,
  pub score: f64,
}

#[napi(object)]
pub struct BestMatchResult {
  pub best_match: BestMatch,
  pub all_matches: Vec<BestMatch>,
  pub has_tie: bool,
}

#[napi]
pub enum SimilarityAlgorithm {
  JaroWinkler,
  LevenshteinSimilarity,
}

#[napi(object)]
pub struct FindBestMatchOptions {
  pub algorithm: Option<SimilarityAlgorithm>,
  pub threshold: Option<f64>,
}

#[napi]
pub fn string_similarity(
  s1: String,
  s2: String,
  algorithm: Option<SimilarityAlgorithm>,
) -> f64 {
  // Normalize inputs for better Unicode support.
  let s1 = normalize(&s1);
  let s2 = normalize(&s2);
  let algorithm = algorithm.unwrap_or(SimilarityAlgorithm::JaroWinkler);
  match algorithm {
    SimilarityAlgorithm::JaroWinkler => jaro_winkler(&s1, &s2),
    SimilarityAlgorithm::LevenshteinSimilarity => {
      let distance = levenshtein(&s1, &s2);
      let max_len = std::cmp::max(s1.len(), s2.len()) as f64;
      if max_len == 0.0 {
        1.0
      } else {
        1.0 - (distance as f64 / max_len)
      }
    }
  }
}

#[napi]
pub fn find_best_match(
  target: String,
  candidates: Vec<String>,
  options: Option<FindBestMatchOptions>,
) -> Result<BestMatchResult> {
  if candidates.is_empty() {
    return Err(Error::from_reason("Candidates list cannot be empty."));
  }

  // Normalize the target string.
  let target_norm = normalize(&target);

  let options = options.unwrap_or(FindBestMatchOptions {
    algorithm: None,
    threshold: None,
  });
  let algorithm = options.algorithm.unwrap_or(SimilarityAlgorithm::JaroWinkler);
  let threshold = options.threshold.unwrap_or(0.0);

  // Compute scores (using normalized strings)
  let mut scored: Vec<(String, f64)> = candidates
    .into_iter()
    .map(|cand| {
      let cand_norm = normalize(&cand);
      let score = match algorithm {
        SimilarityAlgorithm::JaroWinkler => jaro_winkler(&target_norm, &cand_norm),
        SimilarityAlgorithm::LevenshteinSimilarity => {
          let distance = levenshtein(&target_norm, &cand_norm);
          let max_len = std::cmp::max(target_norm.len(), cand_norm.len()) as f64;
          if max_len == 0.0 {
            1.0
          } else {
            1.0 - (distance as f64 / max_len)
          }
        }
      };
      (cand, score)
    })
    .filter(|(_, score)| *score >= threshold)
    .collect();

  if scored.is_empty() {
    return Err(Error::from_reason("No candidates meet the threshold."));
  }

  // Sort descending by score
  scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
  let top_score = scored[0].1;

  // Only keep candidates that exactly match the top score.
  let best_matches: Vec<BestMatch> = scored
    .into_iter()
    .filter(|(_, score)| (score - top_score).abs() < std::f64::EPSILON)
    .map(|(s, score)| BestMatch { string: s, score })
    .collect();

  let has_tie = best_matches.len() > 1;

  Ok(BestMatchResult {
    best_match: best_matches[0].clone(),
    all_matches: best_matches,
    has_tie,
  })
}

#[napi]
pub fn levenshtein_distance(s1: String, s2: String) -> i32 {
  levenshtein(&s1, &s2).try_into().unwrap()
}
