use pdf::file::File;

mod test;

fn main() {

    let file_path = "example.pdf";

    match File::open(file_path) {

        Ok(pdf_file) => {
            println!("Successfully opened the PDF file.");
        },
        Err(e) => {
            eprintln!("Failed to open the PDF file: {}", e);
        }

    }
}