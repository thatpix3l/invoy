

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::pdf_components_util::{build, build_recursive, BuildInvoiceArguments, INVOICE_DIR_NAME_RE};

use flutter_rust_bridge::frb;

#[frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

pub fn pick_invoice_dir() -> Option<String> {
    rfd::FileDialog::new()
        .pick_folder()
        .map(|p| p.to_str().map(|s| s.to_owned()))
        .flatten()
}

pub fn build_invoice(input_dir: String) -> Option<String> {
    
    build_recursive(BuildInvoiceArguments{input_dir: input_dir.into()})
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .first()
        .map(|s| s.into())

}

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
