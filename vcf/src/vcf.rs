use std::io;
use std::io::BufRead;
use crate::Header;
use crate::HeaderValue::Flat;
use crate::validate_fileformat::is_valid_file_format;
use crate::parse;

pub struct VCF {
    pub file_format: String,
}

#[derive(Debug)]
pub enum VCFError {
    ParseError,
    IoError(io::Error),
}

impl From<io::Error> for VCFError {
    fn from(error: io::Error) -> Self {
        VCFError::IoError(error)
    }
}

impl From<parse::ParseError> for VCFError {
    fn from(error: parse::ParseError) -> Self {
        VCFError::ParseError
    }
}

/// Create a VCF object from a file.
///
/// Given a valid file, one can obtain an object continaing the VCF data.
///
/// For example, we can check the version of a vcf file as follows.
/// 
/// ```
/// use vcf::vcf::parse_vcf;
/// let vcf_source = br#"##fileformat=VCFv4.4
/// ###fileDate=20090805
/// ###source=myImputationProgramV3.1
/// ###reference=file:///seq/references/1000GenomesPilot-NCBI36.fasta
/// ###contig=<ID=20,length=62435964,assembly=B36,md5=f126cdf8a6e0c7f379d618ff66beb2da,species="Homo sapiens",taxonomy=x>
/// ###phasing=partial
/// ###INFO=<ID=NS,Number=1,Type=Integer,Description="Number of Samples With Data">
/// ###INFO=<ID=DP,Number=1,Type=Integer,Description="Total Depth">
/// ###INFO=<ID=AF,Number=A,Type=Float,Description="Allele Frequency">
/// ###INFO=<ID=AA,Number=1,Type=String,Description="Ancestral Allele">
/// ###INFO=<ID=DB,Number=0,Type=Flag,Description="dbSNP membership, build 129">
/// ###INFO=<ID=H2,Number=0,Type=Flag,Description="HapMap2 membership">
/// ###FILTER=<ID=q10,Description="Quality below 10">
/// ###FILTER=<ID=s50,Description="Less than 50% of samples have data">
/// ###FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
/// ###FORMAT=<ID=GQ,Number=1,Type=Integer,Description="Genotype Quality">
/// ###FORMAT=<ID=DP,Number=1,Type=Integer,Description="Read Depth">
/// ###FORMAT=<ID=HQ,Number=2,Type=Integer,Description="Haplotype Quality">
/// ##CHROM POS ID REF ALT QUAL FILTER INFO FORMAT NA00001 NA00002 NA00003
/// 20 14370 rs6054257 G A 29 PASS NS=3;DP=14;AF=0.5;DB;H2 GT:GQ:DP:HQ 0|0:48:1:51,51 1|0:48:8:51,51 1/1:43:5:.,.
/// 20 17330 . T A 3 q10 NS=3;DP=11;AF=0.017 GT:GQ:DP:HQ 0|0:49:3:58,50 0|1:3:5:65,3 0/0:41:3
/// 20 1110696 rs6040355 A G,T 67 PASS NS=2;DP=10;AF=0.333,0.667;AA=T;DB GT:GQ:DP:HQ 1|2:21:6:23,27 2|1:2:0:18,2 2/2:35:4
/// 20 1230237 . T . 47 PASS NS=3;DP=13;AA=T GT:GQ:DP:HQ 0|0:54:7:56,60 0|0:48:4:51,51 0/0:61:2
/// 20 1234567 microsat1 GTC G,GTCT 50 PASS NS=3;DP=9;AA=G GT:GQ:DP 0/1:35:4 0/2:17:2 1/1:40:3
/// "#;
///# use vcf::vcf::VCFError;
/// let vcf = parse_vcf(&vcf_source[..])?;
/// assert_eq!(vcf.file_format, "VCFv4.4");
///# Ok::<(), VCFError>(())
/// ```
///
/// On the other hand, if the file is invalid, for example because the file format attribute is
/// missing, then the function will return a `VCFError` instance, highlighting the error.
///
/// ```
/// use vcf::vcf::parse_vcf;
/// let vcf_source = br#"##fileDate=20090805
/// ###source=myImputationProgramV3.1
/// ###reference=file:///seq/references/1000GenomesPilot-NCBI36.fasta
/// ###contig=<ID=20,length=62435964,assembly=B36,md5=f126cdf8a6e0c7f379d618ff66beb2da,species="Homo sapiens",taxonomy=x>
/// ###phasing=partial
/// ###INFO=<ID=NS,Number=1,Type=Integer,Description="Number of Samples With Data">
/// ###INFO=<ID=DP,Number=1,Type=Integer,Description="Total Depth">
/// ###INFO=<ID=AF,Number=A,Type=Float,Description="Allele Frequency">
/// ###INFO=<ID=AA,Number=1,Type=String,Description="Ancestral Allele">
/// ###INFO=<ID=DB,Number=0,Type=Flag,Description="dbSNP membership, build 129">
/// ###INFO=<ID=H2,Number=0,Type=Flag,Description="HapMap2 membership">
/// ###FILTER=<ID=q10,Description="Quality below 10">
/// ###FILTER=<ID=s50,Description="Less than 50% of samples have data">
/// ###FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
/// ###FORMAT=<ID=GQ,Number=1,Type=Integer,Description="Genotype Quality">
/// ###FORMAT=<ID=DP,Number=1,Type=Integer,Description="Read Depth">
/// ###FORMAT=<ID=HQ,Number=2,Type=Integer,Description="Haplotype Quality">
/// ##CHROM POS ID REF ALT QUAL FILTER INFO FORMAT NA00001 NA00002 NA00003
/// 20 14370 rs6054257 G A 29 PASS NS=3;DP=14;AF=0.5;DB;H2 GT:GQ:DP:HQ 0|0:48:1:51,51 1|0:48:8:51,51 1/1:43:5:.,.
/// 20 17330 . T A 3 q10 NS=3;DP=11;AF=0.017 GT:GQ:DP:HQ 0|0:49:3:58,50 0|1:3:5:65,3 0/0:41:3
/// 20 1110696 rs6040355 A G,T 67 PASS NS=2;DP=10;AF=0.333,0.667;AA=T;DB GT:GQ:DP:HQ 1|2:21:6:23,27 2|1:2:0:18,2 2/2:35:4
/// 20 1230237 . T . 47 PASS NS=3;DP=13;AA=T GT:GQ:DP:HQ 0|0:54:7:56,60 0|0:48:4:51,51 0/0:61:2
/// 20 1234567 microsat1 GTC G,GTCT 50 PASS NS=3;DP=9;AA=G GT:GQ:DP 0/1:35:4 0/2:17:2 1/1:40:3
/// "#;
/// use vcf::vcf::VCFError;
/// match parse_vcf(&vcf_source[..]) {
///     Err(VCFError) => assert!(true),
///     _ => assert!(false),
/// };
/// ```
///
/// Similarly, we can obtain the format information for a file via the `format` attribute.
///
/// ```
/// use vcf::vcf::parse_vcf;
/// let vcf_source = br#"##fileformat=VCFv4.4
/// ###fileDate=20090805
/// ###source=myImputationProgramV3.1
/// ###reference=file:///seq/references/1000GenomesPilot-NCBI36.fasta
/// ###contig=<ID=20,length=62435964,assembly=B36,md5=f126cdf8a6e0c7f379d618ff66beb2da,species="Homo sapiens",taxonomy=x>
/// ###phasing=partial
/// ###INFO=<ID=NS,Number=1,Type=Integer,Description="Number of Samples With Data">
/// ###INFO=<ID=DP,Number=1,Type=Integer,Description="Total Depth">
/// ###INFO=<ID=AF,Number=A,Type=Float,Description="Allele Frequency">
/// ###INFO=<ID=AA,Number=1,Type=String,Description="Ancestral Allele">
/// ###INFO=<ID=DB,Number=0,Type=Flag,Description="dbSNP membership, build 129">
/// ###INFO=<ID=H2,Number=0,Type=Flag,Description="HapMap2 membership">
/// ###FILTER=<ID=q10,Description="Quality below 10">
/// ###FILTER=<ID=s50,Description="Less than 50% of samples have data">
/// ###FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
/// ###FORMAT=<ID=GQ,Number=1,Type=Integer,Description="Genotype Quality">
/// ###FORMAT=<ID=DP,Number=1,Type=Integer,Description="Read Depth">
/// ###FORMAT=<ID=HQ,Number=2,Type=Integer,Description="Haplotype Quality">
/// ##CHROM POS ID REF ALT QUAL FILTER INFO FORMAT NA00001 NA00002 NA00003
/// 20 14370 rs6054257 G A 29 PASS NS=3;DP=14;AF=0.5;DB;H2 GT:GQ:DP:HQ 0|0:48:1:51,51 1|0:48:8:51,51 1/1:43:5:.,.
/// 20 17330 . T A 3 q10 NS=3;DP=11;AF=0.017 GT:GQ:DP:HQ 0|0:49:3:58,50 0|1:3:5:65,3 0/0:41:3
/// 20 1110696 rs6040355 A G,T 67 PASS NS=2;DP=10;AF=0.333,0.667;AA=T;DB GT:GQ:DP:HQ 1|2:21:6:23,27 2|1:2:0:18,2 2/2:35:4
/// 20 1230237 . T . 47 PASS NS=3;DP=13;AA=T GT:GQ:DP:HQ 0|0:54:7:56,60 0|0:48:4:51,51 0/0:61:2
/// 20 1234567 microsat1 GTC G,GTCT 50 PASS NS=3;DP=9;AA=G GT:GQ:DP 0/1:35:4 0/2:17:2 1/1:40:3
/// "#;
///# use vcf::vcf::VCFError;
/// let vcf = parse_vcf(&vcf_source[..])?;
/// let hq_description = vcf.format
///     .iter()
///     .find(|item| match item.get("ID") {Some("HQ") => true, _ => false})
///     .and_then(|item| item.get("Description"))
///     .unwrap();
/// assert_eq!(hq_description, "Haplotype Quality");
///# Ok::<(), VCFError>(())
/// ```
pub fn parse_vcf(source: impl BufRead) ->  Result<VCF, VCFError> {
    let mut lines = source.lines();
    let first_line = lines.next().ok_or(VCFError::ParseError)??;
    let parsed = Header::parse(&first_line)?;
    if !is_valid_file_format(&parsed) {
        return Err(VCFError::ParseError)
    }
    let file_format = match parsed.value {
        Flat(s) => s.to_string(),
        _ => panic!(),
    };
    let formats: Result<Vec<_>, VCFError> = lines
        .map(
            |result| match result {
                Ok(ref line) => Header::parse(line).map_err(VCFError::from),
                Err(e) => Err(VCFError::IoError(e)),
            }
        )
        .collect();
    Ok(VCF {file_format: file_format.to_string()})
}
