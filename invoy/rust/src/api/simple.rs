use crate::{err, pdf_components_util};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

pub fn pick_invoice_dir() -> Option<String> {
    rfd::FileDialog::new()
        .pick_folder()
        .map(|p| p.to_str().map(|s| s.to_owned()))
        .flatten()
}

pub fn build_invoice(input_dir: Option<String>) -> Option<String> {
    
    let input_dir = match input_dir {
        Some(v) => v,
        None => return Some(err::BuildInvoiceError::GsRetrieveStdout.to_string())
    };

    crate::pdf_components_util::build_invoice(pdf_components_util::BuildInvoiceArguments{
        input_dir: input_dir.into()
    }).err().map(|v| v.to_string())
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
