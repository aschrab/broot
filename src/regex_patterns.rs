//! a filename filtering pattern using a regular expression

use core::result;
use regex;
use std::fmt;

use crate::errors::RegexError;
use crate::patterns;

#[derive(Debug, Clone)]
pub struct RegexPattern {
    rex: regex::Regex,
    flags: String, // kept only because we may need to build the pattern using to_string()
}

impl fmt::Display for RegexPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.rex, self.flags)
    }
}

impl RegexPattern {
    pub fn from(pat: &str, flags: &str) -> result::Result<RegexPattern, RegexError> {
        let mut builder = regex::RegexBuilder::new(pat);
        for c in flags.chars() {
            match c {
                'i' => {
                    builder.case_insensitive(true);
                }
                'U' => {
                    builder.swap_greed(true);
                }
                _ => {
                    return Err(RegexError::UnknownFlag { bad: c });
                }
            }
        }
        Ok(RegexPattern {
            rex: builder.build()?,
            flags: flags.to_string(),
        })
    }
    // return a match if the pattern can be found in the candidate string
    pub fn find(&self, candidate: &str) -> Option<patterns::Match> {
        // note that there's no significative cost related to using
        //  find over is_match
        match self.rex.find(candidate) {
            Some(rm) => {
                let mut pos = Vec::with_capacity(rm.end() - rm.start());
                for i in rm.start()..rm.end() {
                    pos.push(i);
                }
                Some(patterns::Match { score: 1, pos })
            }
            None => None,
        }
    }
    // return the number of results we should find before starting to
    //  sort them (unless time is runing out).
    // In the case of regexes, there's no need to find more results, as
    //  their score is always 1
    pub fn optimal_result_number(&self, targeted_size: usize) -> usize {
        targeted_size
    }
}
