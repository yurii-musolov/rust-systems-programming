use serde::Deserialize;
use serde_json;

// load the regex-fules.json file to provide configs
const JSON: &str = include_str!("../rules.json");


#[derive(Deserialize, Debug)]
struct ComplianceRule {
    path_regex: String,
    file_permissions: u32,
    required_files: Vec<String>,
    non_existent_files: Vec<String>,
}

impl ComplianceRule {
    fn new(path_regex: String, file_permissions: u32, required_files: Vec<String>, non_existent_files: Vec<String>) -> Self {
        Self {
            path_regex,
            file_permissions,
            required_files,
            non_existent_files,
        }
    }
}

// Load the rules from a configuration file (JSON)
fn load_rules() -> Vec<ComplianceRule> {
    let loaded_json: Vec<ComplianceRule> = serde_json::from_str(JSON).unwrap();

    loaded_json.iter().map(|rule|ComplianceRule::new(
        rule.path_regex.clone(),
        rule.file_permissions,
        rule.required_files.clone(),
        rule.non_existent_files.clone(),
    )).collect::<Vec<_>>()
}

fn main() {
    let rules = load_rules(); 
    println!("{:#?}", rules);
}