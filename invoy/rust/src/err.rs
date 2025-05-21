use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildInvoiceError {
    #[error("could not retrieve child ghostscript process' \"stdout\"")]
    GsRetrieveStdout,

    #[error("input directory name is not valid UTF-8")]
    InputNameNotUtf8,

    #[error("input directory name is malformed")]
    InputNameMalformed,

    #[error("unable to run ghostscript command: {0}")]
    GsCommandRun(std::io::Error),

    #[error("unable to extract name of invoice directory from provided path")]
    ParseInvoiceDirName,
}
