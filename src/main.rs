mod functions;
use std::fs::File;
use tabfile::Tabfile;
use std::path::Path;
use std::io::Write;
use clap::Parser;
use std::time::Instant;

fn main() {
    // Start time
    let now = Instant::now();

    // Constants - ish
    let mut debug: bool = false;
    let mut info_fields: Vec<String> = Vec::new();
    let mut header: Vec<String> = Vec::new();
    let default_columns: [&str; 9] = ["#CHROM", "POS", "ID", "REF", "ALT", "QUAL", "FILTER", "INFO", "FORMAT"];
    let default_columns_len: usize = default_columns.len();
    let mut _number_of_samples: usize = 1;
    let output_separator: &str = "\t";
    let mut _counter: i64 = 1;
    // EXPAND THOSE FIELDS IN INFO USING PIPE |
    let mut info_field_to_expend : Vec<String> = Vec::new();
    
    // Parse arguments
    // INPUT and OUTPUT
    let cli = functions::Cli::parse();

    // PARSE ARGUMENTS
    let filepath = cli.input.as_deref().unwrap();
    let output_path: String = filepath.to_owned().into_os_string().into_string().unwrap().replace(".vcf", ".txt");
    let mut output_file: File = File::create(output_path).expect("Could Not Create Output File");
    if cli.verbose == true {debug = true} else {debug = false};

    let tabfile: Tabfile = functions::read_tab_file(&Path::new(filepath));


    // TEST HEADER
    if let Ok(lines) = functions::read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with('#'){
                    // Get all items in the info fields
                    let info_field = functions::parse_header(&ip);
                    if !info_field.is_empty(){
                        if debug{println!("DEBUG INFO FIELDS: {}", info_field);}
                        info_fields.push(info_field.clone());
                        if ip.contains("|") {
                            info_field_to_expend.push(info_field);
                            let tmp_string = functions::extract_string_between_substrings(&ip, "Description=\"", "\">");
                            for item in functions::string_to_vector_by_delimiter(tmp_string, "|"){
                                if debug{println!("DEBUG INFO FIELDS: {}", &item);}
                                info_fields.push(item);
                            }
                        }
                    }
                    
                    // get header as a vector 
                    if ip.starts_with("#CHROM"){
                        header = functions::string_to_vector_by_delimiter(ip, "\t");
                        _number_of_samples = header.len() - default_columns_len;
                        println!("{} samples founds", _number_of_samples);
                        // remove info field
                        header.remove(7);
                        // insert info fields
                        header.splice(7..7, info_fields.clone());
                        if debug{println!("DEBUG:\nHeader is \n{:?}", header)}
                        // BREAK AFTER FINDING HEADER
                        break
                    }
                }
            }
        }
    }

    // Sanity check for headers
    // Write to file

    if header.is_empty(){panic!("Could not find header row : #CHROM POS	ID	REF	ALT	QUAL    FILTER\nAre you are using a vcf file?")}
    if info_fields.is_empty(){panic!("Could not find any INFO field to expand or VCF header is malformed or contains quotes")}

    if debug{
        println!("Fields with pipes to expand:");
        functions::print_vec(&info_field_to_expend);
    }

    // IF ALL GOOD WRITE HEADER
    for item in &header{
        // DO not write the one that need to be expended
        if info_field_to_expend.contains(item) == false{
        output_file.write_all(item.as_bytes()).unwrap();
        output_file.write_all(output_separator.as_bytes()).unwrap();
        }
    }

    output_file.write_all("\n".as_bytes()).unwrap();

    // PROCESS LINES OF TAB FILES
    println!("Processing full file");
    let full_information: Vec<Vec<String>> = tabfile.into_iter().map(|x|functions::process_vcf_line(x.unwrap().fields(), &info_fields, _number_of_samples, &info_field_to_expend)).collect();
    println!("Writing {} records.", full_information.len());
    //CONTIUNUE TODO
    let lines: Vec<String> = full_information
        .iter()
        .map(|vec_strings| vec_strings.join("\t"))
        .collect();
    let content = lines.join("\n");
    let _ = output_file.write_all(content.as_bytes());


    // END TIME
    let elapsed_time = now.elapsed();
    println!("Done in {}s", elapsed_time.as_secs())
}
