
#[cfg(test)]
mod tests {    
    use std::{path::PathBuf, collections::HashMap};

    use crate::parser::FastaRecord;


    #[test]
    fn test_parsing() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/parser/test_files/AB626963.fna".to_string());
        let file = file.as_path().to_str().unwrap();
        let parsed_result = FastaRecord::parse(file.to_string());
        let parsed_record = parsed_result.unwrap();
        
        assert_eq!(
            parsed_record,
            FastaRecord {
                id: "AB626963.1".to_string(),
                sequence: "TAGATACATAAATTTTGTATTTGATGAATATTTAATAGGTTAGATAAGTTTGAAAAGT".to_string(),
                nucleotide_abundance: HashMap::from([('A', 23), ('T', 24), ('G', 10), ('C', 1)]),
                gc_percentage: 0.18965517
            }
        )
    }
}

