use regex::Regex;

lazy_static! {
    static ref INTS_RE: Regex = Regex::new(r"-?\d+").unwrap();
}

pub fn ints(input: &str) -> Vec<i64> {
    INTS_RE
        .find_iter(input)
        .map(|s| s.as_str().parse::<i64>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let output = ints("");
        let expected: Vec<i64> = vec![];
        assert_eq!(expected, output);
    }

    #[test]
    fn test_ints() {
        let output = ints("jag heter ape 321 och det är 19 fåglar här ååh kaffe är 123 gott");
        let expected: Vec<i64> = vec![321, 19, 123];
        assert_eq!(expected, output);
    }
}
