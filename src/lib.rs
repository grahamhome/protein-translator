use std::collections::HashMap;
use std::str;

mod tests;

pub struct CodonsInfo<'a>(HashMap<&'a str, &'a str>);

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.0.get(codon).map(|p| *p)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(|chunk| self.name_for(str::from_utf8(chunk).unwrap()))
            .take_while(|candidate| candidate.is_none() || candidate.unwrap() != "stop codon")
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo(pairs.into_iter().collect())
}
