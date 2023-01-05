mod tests;

use anyhow::Result;
use bio::io::fasta::Reader;
use tui::text::Text;

trait NucleotideCounter {
    fn count(&self, item: &str) -> u64;
}

impl NucleotideCounter for String {
    fn count(&self, item: &str) -> u64 {
        self.matches(item).count() as u64
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FastaRecord {
    pub id: String,
    pub nucleotide_counts: [(&'static str, u64); 4],
}

impl FastaRecord {
    pub fn parse(path: String) -> Result<Self> {
        let reader = Reader::from_file(&path)
            .expect(format!("Couldn't create a reader from file {}", path).as_str());

        let record = reader
            .records()
            .next()
            .expect(format!("File {} is empty", &path).as_str())?;
        let id = record.id().to_string();
        let seq = String::from_utf8(record.seq().to_vec())?;

        let counts = [
            ("A", seq.count("A")),
            ("T", seq.count("T")),
            ("G", seq.count("G")),
            ("C", seq.count("C")),
        ];

        Ok(Self {
            id: id,
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
