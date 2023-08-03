use std::path::Path;
use tabfile::Tabfile;
use std::fs::File;
use std::io::{self, BufRead};
use clap::Parser;
use std::path::PathBuf;



/// PARSER TO USE
#[derive(Parser)]
#[command(author = "AX", version = "0.0.1", about = "vcf2tab", long_about = "Transforms a vcf into a table. Tries to expand all possible fields from INFO", arg_required_else_help = true)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, required = true, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,
}


/// Wrap a function to open a Tabfile from a path
pub fn read_tab_file(filepath: &Path) -> Tabfile{
    let tabfile: Tabfile =  Tabfile::open(filepath)
    .unwrap()
    .separator('\t')
    .comment_character('#')
    ;// if you want to ignore lines starting with #;
   return tabfile;
}

/// Return a string that is sandwiched between two substrings
pub fn extract_string_between_substrings(input_string: &str, start_substring: &str, end_substring: &str) -> String {
    if let Some(start_index) = input_string.find(start_substring) {
        let rest_of_string = &input_string[start_index + start_substring.len()..];
        // If end string is found
        if let Some(end_index) = rest_of_string.find(end_substring) {
            // Use get() to extract the substring between the two substrings
            return rest_of_string.get(0..end_index).unwrap().to_owned();
        } else { // return all the rest if we don't find the end string
            return rest_of_string.to_owned();
        }
    }

    // If either of the substrings is not found, return empty string (should never happen)
    return String::new()
}

/// Parse header in order to get the info fields
pub fn parse_header(line: &str) -> String{
    // if the INFO field is found in header 
    let mut info_field = String::new();
    if line.find("##INFO=<ID=") != None {
        info_field = extract_string_between_substrings(line, "##INFO=<ID=" ,",");
    }
    return info_field
}

/// Convenience function to transform a Vec<&str> to a Vec<String>
pub fn transform_vec_str_to_vec_string(vec_str: Vec<&str>) -> Vec<String> {
    let vec_string: Vec<String> = vec_str.into_iter().map(|x| x.to_owned()).collect();
    return vec_string;
}

/// Split the vcf line by expending the info field
pub fn process_vcf_line(line_vec: Vec<&str>, info_fields: &Vec<String>, _number_of_samples: usize, info_field_to_expend: &Vec<String>) -> Vec<String>{
        // First 7 columns are kept
        let mut new_line: Vec<String> = transform_vec_str_to_vec_string(line_vec[0..7].to_vec());

        // Expand info field
        let info_string = line_vec[7];
        // iterate over all fields from INFO
        for field in info_fields.into_iter(){
            // println!("{}", formated_info_field);
            if info_field_to_expend.contains(&field){
            // If the field is TO BE extended
            let tmp_str = extract_string_between_substrings(info_string,&format!("{}=", field), ";");
            let tmp_vec = string_to_vector_by_delimiter(tmp_str, "|");
                for item in tmp_vec{
                    new_line.push(item);
                }   
            } else {
            // If the field is to NOT be extended
            new_line.push(extract_string_between_substrings(info_string,&format!("{}=", field), ";"));
            }
        }
        // Get genotype ;
        new_line.extend(transform_vec_str_to_vec_string(line_vec[8..].to_vec()));

        return new_line;
}

/// Verbatim from example https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Input a string an return a vector of stringd
pub fn string_to_vector_by_delimiter(input_str: String, delimiter: &str) -> Vec<String>{
    let return_value: Vec<String> = input_str.split(delimiter).map(String::from).collect();
    return return_value
}

/// Convenience function to print vectors

pub fn print_vec(vec_to_print: &Vec<String>){
    for item in vec_to_print{
        println!("{}", item);
    }
}
