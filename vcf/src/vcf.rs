///Create a VCF object from a file.
///
///```
///use std::fs::File;
///use vcf::vcf::parse_vcf;
///let vcf_file = File::open("example_files/example_1_point_1_from_spec.vcf").unwrap();
///let vcf = parse_vcf(vcf_file);
///```
pub fn parse_vcf(vcf_file: std::fs::File) {}
