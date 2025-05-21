use directories::BaseDirs;

// Commandline-related imports.
use clap::{Parser, Subcommand};
use iced::futures::FutureExt;

use iced::theme::palette;
use iced::widget::Image;
// Regex-related imports.
use regex::Regex;
use regex_macro::regex;
use std::sync::LazyLock;

// Path handling-related imports.
use std::path::{Path, PathBuf};

mod builder_util;
use builder_util::*;

mod err;
use err::*;

mod gs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandRoot {
    #[command(subcommand)]
    command: RootSubcommands,
}

#[derive(Subcommand)]
enum RootSubcommands {
    Build(CommandBuild),
}

#[derive(Parser)]
struct CommandBuild {
    #[arg(short = 'i', long = "input-dir", default_value = "./", help = "lmao")]
    input_dir: PathBuf,
}

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

static INVOICE_DIR_NAME_RE: Re = regex!(
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

struct BuildInvoiceArguments {
    input_dir: PathBuf,
}

fn build_invoice(args: impl Into<BuildInvoiceArguments>) -> Result<(), BuildInvoiceError> {
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

    let builder = gs::Builder {};
    builder.build(paths, output_dir_path)
}

#[derive(Debug, Clone)]
enum Message {
    BuildInvoice,
    BeginPickInvoiceDir,
    EndPickInvoiceDir(String),
    CancelledPickInvoiceDir,
}

#[derive(Clone)]
struct Icons {
    folder_symbolic: PathBuf,
}

fn img(name: impl Into<String>) -> PathBuf {
    lookup(&name.into()).with_theme("Adwaita").find().unwrap()
}

impl Default for Icons {
    fn default() -> Self {
        Self {
            folder_symbolic: img("folder-symbolic"),
        }
    }
}

#[derive(Clone)]
struct State {
    invoice_dir: PathBuf,
    base_dirs: BaseDirs,
    icons: Icons,
}

impl Default for State {
    fn default() -> Self {
        Self {
            invoice_dir: "".into(),
            base_dirs: directories::BaseDirs::new()
                .expect("could not retrieve home directory path"),
            icons: Icons::default(),
        }
    }
}

impl Into<BuildInvoiceArguments> for State {
    fn into(self) -> BuildInvoiceArguments {
        BuildInvoiceArguments {
            input_dir: self.invoice_dir,
        }
    }
}

use iced::{Color, Task, Theme};

async fn pick_invoice_dir(default_dir: PathBuf) -> Message {
    rfd::AsyncFileDialog::new()
        .set_directory(default_dir)
        .pick_folder()
        .map(|fh| {
            fh.map(|fh| {
                fh.path()
                    .to_str()
                    .expect("could not convert invoice directory \"&Path\" into \"&str\"")
                    .to_owned()
            })
            .map(|s| Message::EndPickInvoiceDir(s))
        })
        .map(|maybe_msg| maybe_msg.unwrap_or(Message::CancelledPickInvoiceDir))
        .await
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::BeginPickInvoiceDir => {
            Task::future(pick_invoice_dir(state.base_dirs.home_dir().into()))
        }
        Message::EndPickInvoiceDir(invoice_dir_path) => {
            state.invoice_dir = invoice_dir_path.into();
            Task::none()
        }
        Message::BuildInvoice => {
            if let Err(err) = build_invoice(state.clone()) {
                println!("{}", err);
            }
            Task::none()
        }
        Message::CancelledPickInvoiceDir => {
            println!("user cancelled picking an invoice directory");
            Task::none()
        }
    }
}

use freedesktop_icons::lookup;
use iced::Element;

fn view(state: &State) -> Element<Message> {
    use iced::widget::{button, column, image, row, svg, text, text_input};

    let mut icon_style = svg::Style::default();
    icon_style.color = Some(Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    });

    let folder_symbolic_icon: svg::Svg<'_, Theme> = svg(state.icons.folder_symbolic.clone())
        .content_fit(iced::ContentFit::Fill)
        .width(32)
        .height(32)
        .style(move |_, _| icon_style);

    column![
        row![
            button(folder_symbolic_icon).on_press(Message::BeginPickInvoiceDir),
            button("Build Invoice").on_press(Message::BuildInvoice),
        ]
        .spacing(4),
        row![text(
            state.invoice_dir.to_str().unwrap_or("INVALID PATH NAME")
        )]
    ]
    .into()
}

fn main() -> iced::Result {
    iced::application("Invoy", update, view)
        .theme(|_| iced::Theme::Dark)
        .run()
}
