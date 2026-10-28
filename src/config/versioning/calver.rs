use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct ConfigCalVer {
    pub format: CalVerFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Year4,
    Year2,
    Year2Padded,
    Month,
    MonthPadded,
    Day,
    DayPadded,
    Major,
    Minor,
    Micro,
}

impl Token {
    fn parse(text: &str) -> Result<Self, String> {
        match text {
            "YYYY" => Ok(Token::Year4),
            "YY" => Ok(Token::Year2),
            "0Y" => Ok(Token::Year2Padded),
            "M" | "MM" => Ok(Token::Month),
            "0M" => Ok(Token::MonthPadded),
            "D" | "DD" => Ok(Token::Day),
            "0D" => Ok(Token::DayPadded),
            "MAJOR" => Ok(Token::Major),
            "MINOR" => Ok(Token::Minor),
            "MICRO" => Ok(Token::Micro),
            "PATCH" => Err("'PATCH' is not a CalVer token, use 'MICRO'".to_string()),
            "W" | "0W" | "WW" => Err(format!("Week token '{text}' is not supported yet")),
            _ => Err(format!("unknown token: {text}")),
        }
    }

    fn is_year(self) -> bool {
        matches!(self, Token::Year4 | Token::Year2 | Token::Year2Padded)
    }

    fn family(self) -> Family {
        match self {
            Token::Year4 | Token::Year2 | Token::Year2Padded => Family::Year,
            Token::Month | Token::MonthPadded => Family::Month,
            Token::Day | Token::DayPadded => Family::Day,
            Token::Major => Family::Major,
            Token::Minor => Family::Minor,
            Token::Micro => Family::Micro,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Family {
    Year,
    Month,
    Day,
    Major,
    Minor,
    Micro,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(try_from = "String")]
pub struct CalVerFormat {
    tokens: Vec<Token>,
}

impl TryFrom<String> for CalVerFormat {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("format cannot be empty".to_string());
        }

        let mut tokens: Vec<Token> = Vec::new();
        for part in value.split(".") {
            let token = Token::parse(part)?;
            if tokens.iter().any(|t| t.family() == token.family()) {
                return Err(format!(
                    "token '{part}' conflicts with another {:?} token",
                    token.family()
                ));
            }
            tokens.push(token);
        }

        if !tokens.iter().any(|token| token.is_year()) {
            return Err("format must contain a year token (YYYY, YY or 0Y)".to_string());
        }

        Ok(Self { tokens })
    }
}

impl Default for CalVerFormat {
    fn default() -> Self {
        Self {
            tokens: vec![Token::Year4, Token::Month, Token::Micro],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::versioning::calver::CalVerFormat;

    use super::{ConfigCalVer, Token};

    #[test]
    fn default_config_calver() {
        let config = ConfigCalVer::default();
        assert_eq!(config.format, parse("YYYY.M.MICRO").unwrap());
    }

    #[test]
    fn token_aliases_map_to_the_same_token() {
        assert_eq!(Token::parse("M").unwrap(), Token::parse("MM").unwrap());
        assert_eq!(Token::parse("D").unwrap(), Token::parse("DD").unwrap());
    }

    #[test]
    fn token_patch_suggests_to_use_micro() {
        let err = Token::parse("PATCH").unwrap_err();
        assert_eq!(
            err,
            "'PATCH' is not a CalVer token, use 'MICRO'".to_string()
        );
    }

    #[test]
    fn token_rejects_weeks_and_unknown() {
        for input in ["W", "0W", "WW", "FOO", "yyyy", ""] {
            assert!(
                Token::parse(input).is_err(),
                "input: {input}, should be rejected"
            );
        }
    }

    fn parse(input: &str) -> Result<CalVerFormat, String> {
        CalVerFormat::try_from(input.to_string())
    }

    #[test]
    fn parses_known_formats() {
        for input in [
            "YYYY.M.MICRO",
            "YY.0M",
            "YYYY.0M.0D",
            "YY.MINOR.MICRO",
            "YYYY.MINOR",
        ] {
            assert!(parse(input).is_ok(), "'{input}' should be accepted");
        }
    }

    #[test]
    fn keeps_tokens_in_order() {
        let format = parse("YYYY.0M").unwrap();
        assert_eq!(format.tokens, vec![Token::Year4, Token::MonthPadded]);
    }

    #[test]
    fn deprecated_alias_give_the_same_format() {
        assert_eq!(parse("YY.MM.MICRO").unwrap(), parse("YY.M.MICRO").unwrap());
    }

    #[test]
    fn rejects_two_tokens_of_the_same_family() {
        for input in ["YYYY.YY", "YY.0Y", "YYYY.M.0M", "YYYY.D.0D", "YYYY.M.MM"] {
            assert!(parse(input).is_err(), "'{input}' should be rejected");
        }
    }

    #[test]
    fn rejects_invalid_formats() {
        for input in [
            "",
            "YYYY.FOO",
            "YYYY.PATCH",
            "M.MICRO",
            "YYYY.M.M",
            "YYYY.M.MM",
            "YYYY-M",
            "YYYY..M",
        ] {
            assert!(parse(input).is_err(), "'{input}' should be rejected");
        }
    }
}
