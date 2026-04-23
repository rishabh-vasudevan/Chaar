use std::collections::HashMap;

type Feature = String;
type Value = String;

const FEATURE_PAT: &str = "--";

// TODO: This can be clubbed with other utils later on, does not have to be an entire file of its own
pub fn parse_args() -> HashMap<Feature, Option<Value>> {
    let all_args = std::env::args().collect::<Vec<_>>();
    let len_elements = all_args.len();
    let mut arg_map: HashMap<Feature, Option<Value>> = HashMap::new();

    for (idx, arg_val) in all_args.iter().enumerate().skip(1) {
        if arg_val.starts_with(FEATURE_PAT) {
            if idx + 1 < len_elements && !all_args[idx + 1].starts_with(FEATURE_PAT) {
                arg_map.insert(arg_val[2..].to_string(), Some(all_args[idx + 1].clone()));
            } else {
                arg_map.insert(arg_val[2..].to_string(), None);
            }
        }
    }
    arg_map
}
