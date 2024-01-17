use lopdf::Document;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn search_pdf(file_path: &str, query: &str) -> lopdf::Result<bool> {
    let pdf_doc = match Document::load(file_path) {
        Ok(doc) => doc,
        Err(err) => return Err(err),
    };

    let pages = pdf_doc.get_pages();
    let found = Arc::new(AtomicBool::new(false));

    pages.par_iter().for_each(|(i, _)| {
        if !found.load(Ordering::Relaxed) {
            let page_number = (*i + 1) as u32;
            let text = pdf_doc.extract_text(&[page_number]);
            if text.is_ok() && text.unwrap().contains(query) {
                found.store(true, Ordering::Relaxed);
            }
        }
    });

    Ok(found.load(Ordering::Relaxed))
}
