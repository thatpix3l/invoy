use crate::err;
use std::iter::IntoIterator;

pub trait InvoiceBuilder {
    fn build(
        self: Self,
        input_paths: impl IntoIterator<Item: Into<String>>,
        output_path: impl Into<String>,
    ) -> Result<(), err::BuildInvoiceError>;
}
