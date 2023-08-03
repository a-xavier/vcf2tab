# vcf2tab
Transforms a vcf to a table - Expanding the INFO field

### Instructions  

[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and rust needs to be installed first.  

```git clone https://github.com/a-xavier/vcf2tab```  
```cd vcf2tab```  
```cargo build --release```  

### Usage
Only one argument needed  
```-i VCF_FILE``` the vcf file to transform  

The resulting binary will be located in ```./vcf2tab/target/release/vcf2tab```  
You might need to flag the file as executable with ```chmod +x ./vcf2tab```  

