use lopdf::Document as Document;
use std::fs::File as File;

fn main(){
    let file_path = "test.pdf";
    let open = File::open(file_path);
    let file = File::open("test.pdf");
    let doc = Document::load(file)?;

    println!("Number of pages: {}", doc.get_num_pages());
    println!("Metadata: {:?}", doc.get_info_dict());

    if let Some(page) = doc.get_page(0) {
        println!("Page size: {:?}", page.get_media_box());
    }
}