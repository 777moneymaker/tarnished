mod tests;

use std::io::Read;

use anyhow::Result;
use bio::io::fasta::Reader;
use tui::text::Text;

trait NucleotideCounter {
    fn count(&self, item: char) -> u64;
}

impl NucleotideCounter for [u8] {
    fn count(&self, item: char) -> u64 {
        let mut sum: u64 = 0;
        for ch in self.bytes() {
            if ch.expect("Couldn't read nucleotide as byte") == item as u8 {
                sum += 1;
            }
        }
        sum
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastaRecord {
    pub id: String,
    pub nucleotide_counts: [(&'static str, u64); 4],
}

impl FastaRecord {
    pub fn parse(path: String) -> Result<Self> {
        let reader = Reader::from_file(&path)
            .unwrap_or_else(|_| panic!("Couldn't create a reader from file {}", path));

        let record = reader
            .records()
            .next()
            .unwrap_or_else(|| panic!("File {} is empty", &path))?;

        record
            .check()
            .unwrap_or_else(|_| panic!("File {} contains invalid fasta format", path));
        let id: String = record.id().to_string();
        let seq = record.seq();
        let counts = [
            ("A", seq.count('A')),
            ("T", seq.count('T')),
            ("G", seq.count('G')),
            ("C", seq.count('C')),
        ];

        Ok(Self {
            id,
            nucleotide_counts: counts,
        })
    }
}
impl From<FastaRecord> for Text<'_> {
    fn from(record: FastaRecord) -> Self {
        Text::from(record.id)
    }
}

impl From<&FastaRecord> for Text<'_> {
    fn from(record: &FastaRecord) -> Self {
        Text::from(record.id.clone())
    }
}
