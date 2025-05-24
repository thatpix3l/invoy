
use rayon::iter::ParallelIterator;
// Regex-related imports.
use regex::Regex;
use regex_macro::regex;
use walkdir::{DirEntry, WalkDir};
use std::sync::LazyLock;

// Path handling-related imports.
use std::path::{Path, PathBuf};

use crate::builder_util::*;

use crate::err::*;

type Re<'a> = &'a LazyLock<Regex>;

static INVOICE_NAME_RE: Re = regex!(
    r"(?x)
Invoice\ \#
(?<year>[0-9]{2}) # invoice year
(?<month>[0-9]{2}) # invoice month
(?<day>[0-9]{2}) # invoice day
(?<load_number>[0-9]+) # invoice load number
\ -\ Tradeport\ -\ Star\ Transport\.pdf
"
);

pub static INVOICE_DIR_NAME_RE: Re = regex!(
    r"(?x)
invoice\ 
(?<invoice_id> # invoice id
    (?<year>[0-9]{2}) # invoice year
    (?<month>[0-9]{2}) # invoice month
    (?<day>[0-9]{2}) # invoice day
    (?<load_number>[0-9]+) # invoice load number
)
"
);

const INVOICE_DOCUMENT_ORDER: &[&'static str] = &["summary.pdf", "confirmation.pdf", "bol.pdf"];

pub struct BuildInvoiceArguments {
    pub input_dir: PathBuf,
}

fn entry_path(entry: &DirEntry) -> Result<PathBuf, String> {
    let file_name = match entry.file_name().to_owned().into_string() {
        Ok(v) => v,
        Err(_) => return Err("entry's file name cannot be converted into a \"String\"".into())
    };
    
    match INVOICE_DIR_NAME_RE.is_match(&file_name) {
        true => Ok(entry.path().to_owned()),
        false => Err("entry's file name does not match the regex".into())
    }

}

pub fn build_recursive(args: impl Into<BuildInvoiceArguments>) -> impl ParallelIterator<Item = BuildInvoiceError> {
    
    let args = args.into();
    
    use rayon::prelude::*;

    WalkDir::new(args.input_dir.clone()).into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|e| entry_path(&e))
        .flatten()
        .map(|p| build(BuildInvoiceArguments{input_dir: p})
            .err()
        )
        .flatten()
}

pub fn build(args: impl Into<BuildInvoiceArguments>) -> Result<(), BuildInvoiceError> {
    let args = args.into();

    let input_dir_name = args
        .input_dir
        .file_name()
        .map(|s| s.to_str())
        .flatten()
        .ok_or(BuildInvoiceError::ParseInvoiceDirName)?;

    // Extract ID of invoice, based on input directory name.
    let invoice_id = INVOICE_DIR_NAME_RE
        .captures(input_dir_name)
        .ok_or(BuildInvoiceError::InputNameNotUtf8)?
        .name("invoice_id")
        .ok_or(BuildInvoiceError::InputNameMalformed)?
        .as_str();

    let invoice_name = format!("Invoice # {} - Tradeport - Star Transport.pdf", invoice_id);

    let mut output_dir_path = args.input_dir.clone();
    output_dir_path.push(invoice_name);
    let output_dir_path = output_dir_path.into_os_string().into_string().unwrap();

    let paths = INVOICE_DOCUMENT_ORDER
        .iter()
        .map(|s| *s)
        .map(|s| {
            let mut partial_doc_path = args.input_dir.clone();
            partial_doc_path.push(s);
            partial_doc_path
        })
        .map(|s| s.into_os_string().into_string())
        .flatten()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let builder = crate::gs::Builder {};
    builder.build(paths, output_dir_path)
}
