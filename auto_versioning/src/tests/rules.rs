use crate::analyzer::determine_file_impact;
use crate::models::PatternWeights;
use std::collections::HashMap;

#[test]
fn test_pattern_detection() {
    let mut weights = HashMap::new();
    weights.insert("pub trait".to_string(), "major".to_string());
    weights.insert("pub fn".to_string(), "minor".to_string());
    
    let test_cases = vec![
        ("pub trait MyTrait {}", "major"),
        ("pub fn my_func() {}", "minor"),
        ("// Just a comment", "patch"),
    ];

    for (content, expected) in test_cases {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(&temp_file, content).unwrap();
        
        let impact = determine_file_impact(
            temp_file.path(),
            &PatternWeights { weights: weights.clone() }
        );
        
        assert_eq!(impact.unwrap(), expected);
    }
}
