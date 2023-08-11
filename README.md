# vcf2tab  
[![Rust](https://github.com/a-xavier/vcf2tab/actions/workflows/rust.yml/badge.svg)](https://github.com/a-xavier/vcf2tab/actions/workflows/rust.yml)  

Transforms a vcf to a table - Expanding the INFO field

### Instructions  

[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and rust needs to be installed first.  

```git clone https://github.com/a-xavier/vcf2tab```  
```cd vcf2tab```  
```cargo build --release```  
The resulting binary will be located in ```./vcf2tab/target/release/vcf2tab```  
You might need to flag the file as executable with ```chmod +x ./vcf2tab```  

### Usage
Only one argument needed  
```-i VCF_FILE.vcf``` the vcf file to transform  

The resulting tab-delimited file will be called ```VCF_FILE.txt``` and will be located in the same directory as the input file.

