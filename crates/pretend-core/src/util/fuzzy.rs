pub fn score(query: &str, candidate: &str) -> i32 {
  let query = query.to_lowercase();
  let candidate = candidate.to_lowercase();
  if query.is_empty() {
    return 0;
  }

  let mut score = 0;
  for (index, query_char) in query.chars().enumerate() {
    if let Some(candidate_char) = candidate.chars().nth(index) {
      if candidate_char == query_char {
        score += 1;
      }
    }
  }

  if candidate.starts_with(&query) {
    score += 10;
  }
  score
}
