use pdf::file::File as PdfFile;
use pdf::object::{Object, ObjectStream};
use pdf::parser::parse_stream as Parser;
use std::fs::File;
use std::io::BufReader;

fn load_pdf(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let pdf_file = PdfFile::from_reader(reader)?;

    let mut parser = Parser::new(&pdf_file);

    while let Some(object) = parser.next_object()? {
        match object {
            ObjectStream::Page(page) => {
                println!("Page: {:?}", page);
            }
            _ => {}
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = load_pdf("example.pdf") {
        eprintln!("Error loading PDF: {}", e);
    }
}
