use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Merges two JSON values recursively.
/// We merge here with arrays being added together and objects being
/// added together with shared keys being overwritten by the new value.
fn merge_json_values(a: &mut Value, b: Value) {
    match (a, b) {
        // concat arrays
        (Value::Array(ref mut a_array), Value::Array(b_array)) => {
            // Concatenate arrays
            a_array.extend(b_array);
        },
        // merge recursively
        (Value::Object(ref mut a_obj), Value::Object(b_obj)) => {
            // Merge objects recursively
            for (k, b_value) in b_obj {
                if let Some(a_value) = a_obj.get_mut(&k) {
                    merge_json_values(a_value, b_value);
                } else {
                    a_obj.insert(k, b_value);
                }
            }
        },
        // For other types, replace `a` with `b`
        (a_ref, b_value) => {
            *a_ref = b_value;
        }
    }
}

/// Merges multiple JSON files specified by their file paths.
fn merge_json_files<P: AsRef<Path>>(paths: &[P]) -> serde_json::Result<Value> {
    let mut result_value: Option<Value> = None;

    for path in paths {
        let file = File::open(path.as_ref()).unwrap();
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader)?;

        if let Some(ref mut res) = result_value {
            merge_json_values(res, value);
        } else {
            result_value = Some(value);
        }
    }

    Ok(result_value.unwrap_or(Value::Null))
}

fn main() -> serde_json::Result<()> {
    // we take in many files and dump it into one output file
    // No support for simdjson yet - maybe we dont need this much perf
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file1> <input_file2> ... <output_file>", args[0]);
        std::process::exit(1);
    }
    
    let json_files = &args[1..args.len() - 1];
    let output_file = &args[args.len() - 1];

    let merged_json = merge_json_files(&json_files)?;
    let output = File::create(output_file)?;
    serde_json::to_writer_pretty(output, &merged_json)?;
    Ok(())
}