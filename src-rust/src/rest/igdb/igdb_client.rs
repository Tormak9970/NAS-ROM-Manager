fn remove_special_chars(query: &str) -> String {
  return query.replace("\u{2122}", "")        // Remove trademark symbol
    .replace("\u{00a9}", "")                  // Remove copywrite symbol
    .replace("\u{00ae}", "")                  // Remove registered symbol
    .replace("\u{2120}", "")                  // Remove service mark symbol
    .trim()                                               
    .to_string();
}