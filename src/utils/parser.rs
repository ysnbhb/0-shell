use shell_words:: {split , ParseError};

pub fn parst_input(s: String) -> Result<Vec<String>, ParseError> {
   split(&s)
}
