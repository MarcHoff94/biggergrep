use lopdf::Document;


pub fn search_pdf(file_path: &str, query: &str) -> lopdf::Result<bool> {
    let pdf_doc = match Document::load(file_path) {
        Ok(doc) => doc,
        Err(err) => return Err(err),
    };
    let pages = pdf_doc.get_pages();

    for (i, _) in pages.iter().enumerate() {
        let page_number = (i + 1) as u32;
        let text = pdf_doc.extract_text(&[page_number]);
        if text.unwrap().contains(query) {
           return Ok(true)
        }
    };

    Ok(false)
}