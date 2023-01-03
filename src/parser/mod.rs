use std::{collections::HashMap, path::Path};

use anyhow::Result;
use bio::{io::fasta::Reader, seq_analysis::gc};

mod tests;

#[derive(Debug, PartialEq)]
struct FastaRecord {
    id: String,
    sequence: String,
    nucleotide_abundance: HashMap<char, usize>,
    gc_percentage: f32

}

impl FastaRecord {
    pub fn parse(path: String) -> Result<Self> {
        let reader = Reader::from_file(&path).expect(format!("Couldn't create a reader from file {}", path).as_str());

        let record = reader
            .records()
            .next()
            .expect(format!("File {} is empty", &path).as_str())?;
        let id = record.id().to_string();
        let seq = String::from_utf8(record.seq().to_vec())?;

        let mut nucleotide_abundance: HashMap<char, usize> = HashMap::from([
            ('A', 0),
            ('T', 0),
            ('G', 0),
            ('C', 0)
        ]);

        for nucl in seq.chars(){
            let value = nucleotide_abundance.get(&nucl).unwrap();
            nucleotide_abundance.entry(nucl).and_modify(|val| *val += 1);
        }

        let gc = gc::gc_content(seq.as_bytes());
        
        Ok(Self {
            id: id,
            sequence: seq,
            nucleotide_abundance: nucleotide_abundance,
            gc_percentage: gc
        })
    }
}
