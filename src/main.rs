// Commandline-related imports.
use clap::{Parser, Subcommand};
use thiserror::Error;

// Regex-related imports.
use std::{collections::BTreeMap, sync::LazyLock};
use regex_macro::regex;
use regex::Regex;

// Path handling-related imports.
use std::path::{Path, PathBuf};

// PDF manipulation-related imports.
use lopdf::{Bookmark, Document, Object, ObjectId};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandRoot {
    #[command(subcommand)]
    command: RootSubcommands,
}

#[derive(Subcommand)]
enum RootSubcommands {
    Build(CommandBuild)
}

#[derive(Parser)]
struct CommandBuild {
    #[arg(short = 'i', long = "input-dir", default_value = "./", help = "lmao")]
    input_dir: String
}

type Re<'a> = &'a LazyLock<Regex>;

static INVOICE_NAME_RE: Re = regex!(r"(?x)
Invoice\ \#
(?<year>[0-9]{2}) # invoice year
(?<month>[0-9]{2}) # invoice month
(?<day>[0-9]{2}) # invoice day
(?<load_number>[0-9]+) # invoice load number
\ -\ Tradeport\ -\ Star\ Transport\.pdf
");

static INVOICE_DIR_NAME_RE: Re = regex!(r"(?x)
invoice\ 
(?<invoice_id> # invoice id
    (?<year>[0-9]{2}) # invoice year
    (?<month>[0-9]{2}) # invoice month
    (?<day>[0-9]{2}) # invoice day
    (?<load_number>[0-9]+) # invoice load number
)
");

#[derive(Error, Debug)]
enum BuildInvoiceError {
    #[error("input directory path should not end with \"..\"")]
    InputPathDisallowedSuffix,

    #[error("input directory name is not valid UTF-8")]
    InputNameNotUtf8,

    #[error("input directory name is malformed")]
    InputNameMalformed,

    #[error("expected a MediaBox in PDF page")]
    MediaBoxMissing,

    #[error("unable to build invoice")]
    Generic,
}

// Combines multiple PDFs into one
pub fn merge_and_save(documents: Vec<Document>, path: PathBuf) -> std::io::Result<()> {

    // Define a starting max_id (will be used as start index for object_ids)
    let mut max_id = 1;
    let mut pagenum = 1;

    // Collect all Documents Objects grouped by a map
    let mut documents_pages: BTreeMap<(u32, u16), lopdf::Object> = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    for mut doc in documents {
        let mut first = false;
        doc.renumber_objects_with(max_id);

        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc.get_pages()
                .into_values()
                .map(|object_id| {
                    if !first {
                        let bookmark = Bookmark::new(
                            format!("Page_{}", pagenum),
                            [0.0, 0.0, 1.0],
                            0,
                            object_id,
                        );
                        document.add_bookmark(bookmark, None);
                        first = true;
                        pagenum += 1;
                    }

                    (object_id, doc.get_object(object_id).unwrap().to_owned())
                })
                .collect::<BTreeMap<ObjectId, Object>>(),
        );
        documents_objects.extend(doc.objects);
    }

    // Catalog and Pages are mandatory
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.iter() {

        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects
        // All other objects should be collected and inserted into the main Document
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages"
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            b"Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            b"Page" => {}     // Ignored, processed later and separately
            b"Outlines" => {} // Ignored, not supported yet
            b"Outline" => {}  // Ignored, not supported yet
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    // If no "Pages" object found abort
    if pages_object.is_none() {
        println!("Pages root not found.");

        return Ok(());
    }

    // Iterate over all "Page" objects and collect into the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            document
                .objects
                .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found abort
    if catalog_object.is_none() {
        println!("Catalog root not found.");

        return Ok(());
    }

    let catalog_object = catalog_object.unwrap();
    let pages_object = pages_object.unwrap();

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = pages_object.1.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                .into_keys()
                .map(Object::Reference)
                .collect::<Vec<_>>(),
        );

        document
            .objects
            .insert(pages_object.0, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.1.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", pages_object.0);
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        document
            .objects
            .insert(catalog_object.0, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_object.0);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    document.max_id = document.objects.len() as u32;

    // Reorder all new Document objects
    document.renumber_objects();

    //Set any Bookmarks to the First child if they are not set to a page
    document.adjust_zero_pages();

    //Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = document.build_outline() {
        if let Ok(Object::Dictionary(dict)) = document.get_object_mut(catalog_object.0)
        {
            dict.set("Outlines", Object::Reference(n));
        }
    }

    document.compress();

    // Save the merged PDF
    // Store file in current working directory.
    // Note: Line is excluded when running tests
    if false {
        document.save(path.clone()).unwrap();
    }

    document.save(path).unwrap();

    Ok(())
}

fn build_invoice(command_build: &CommandBuild) -> Result<(), BuildInvoiceError> {
    
    let input_dir_path = Path::new(command_build.input_dir.as_str());
    
    // Extract name of input directory path.
    let input_dir_name = input_dir_path
        .file_name()
        .ok_or(BuildInvoiceError::InputPathDisallowedSuffix)?
        .to_str()
        .unwrap();
    
    // Extract ID of invoice, based on input directory name.
    let invoice_id = INVOICE_DIR_NAME_RE
        .captures(input_dir_name)
        .ok_or(BuildInvoiceError::InputNameNotUtf8)?
        .name("invoice_id")
        .ok_or(BuildInvoiceError::InputNameMalformed)?
        .as_str();
    
    let invoice_name = format!("Invoice # {} - Tradeport - Star Transport.pdf", invoice_id);
    
    let mut output_dir_path = PathBuf::from(command_build.input_dir.clone());
    output_dir_path.push(invoice_name);
    
    let documents = vec![
        "summary.pdf",
        "confirmation.pdf",
        "bol.pdf",
    ]
    .into_iter()
    .map(
        |s| {
            let mut path = input_dir_path.to_owned();
            path.push(s);
            path
        }
    )
    .map(
        |p| lopdf::Document::load(p)
    )
    .flatten()
    .collect::<Vec<_>>();
    
    merge_and_save(documents, output_dir_path).or(Err(BuildInvoiceError::Generic))

}

fn main() {
    
    // let maybe_invoice_captures = INVOICE_NAME_RE.captures("Invoice # 2504070176221 - Tradeport - Star Transport.pdf");
    // 
    // let matched_word = maybe_invoice_captures
    // .expect("could not capture groups from supposed invoice filename; is it formatted correctly?")
    // .name("year")
    // .expect("year not found")
    // .as_str();
    // 
    // println!("{}", matched_word);
    // 
    // return;

    let cli = CommandRoot::parse();
    match &cli.command {
        RootSubcommands::Build(command_build) => 
            if let Err(e) = build_invoice(command_build) {
                println!("{}", e)
            }
        ,
    }
}
