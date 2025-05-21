use std::io::BufRead;
use std::io::BufReader;
use std::process::*;

use crate::builder_util::*;
use crate::err::*;

const GS_PDF_CREATION_FLAGS: &[&'static str] = &[
    "-sDEVICE=pdfwrite",
    "-sPAPERSIZE=letter",
    "-dFIXEDMEDIA",
    "-dPDFFitPage",
    "-dCompatibilityLevel=1.4",
    "-o",
];

pub struct Builder {}

// gs -o invoice.pdf -sDEVICE=pdfwrite -sPAPERSIZE=letter -dFIXEDMEDIA -dPDFFitPage -dCompatibilityLevel=1.4 summary.pdf confirmation.pdf bol.pdf

impl InvoiceBuilder for Builder {
    fn build(
        self: Self,
        input_paths: impl IntoIterator<Item: Into<String>>,
        output_path: impl Into<String>,
    ) -> Result<(), BuildInvoiceError> {
        let output_path: String = output_path.into();

        let input_paths = input_paths
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<String>>();

        let res = Command::new("gs")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(GS_PDF_CREATION_FLAGS)
            .arg(output_path)
            .args(input_paths)
            .spawn();

        let handle = match res {
            Ok(v) => v,
            Err(err) => return Err(BuildInvoiceError::GsCommandRun(err)),
        };

        let stdout = handle.stdout.ok_or(BuildInvoiceError::GsRetrieveStdout)?;
        let reader = BufReader::new(stdout);
        let lines = reader.lines();

        for line in lines {
            println!("{}", line.unwrap_or("".to_owned()));
        }

        Ok(())
    }
}
