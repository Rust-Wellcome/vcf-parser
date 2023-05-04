///Create a VCF object from a file.
///
///```
///use std::fs::File;
///use vcf::vcf::parse_vcf;
///let vcf_file = File::open("example_files/minimal_valid_example.vcf").unwrap();
///let vcf = parse_vcf(vcf_file);
///```
pub fn parse_vcf(vcf_file: std::fs::File) {}
