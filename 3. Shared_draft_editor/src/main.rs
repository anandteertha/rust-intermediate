use std::{cell::RefCell, rc::Rc};

use crate::{
    document::Document,
    editor::{BodyEditor, Reviewer, TitleEditor},
};

mod document;
mod editor;
fn main() {
    let title = String::from("My document title");
    let body = String::from("\nmy document body");
    let version = 0;
    let document_obj: Rc<RefCell<Document>> =
        Rc::new(RefCell::new(Document::new(title, body, version)));
    let title_editor = TitleEditor::new(Rc::clone(&document_obj));
    title_editor.update_title();
    let body_editor = BodyEditor::new(Rc::clone(&document_obj));
    body_editor.append_paragraph();
    let reviewer = Reviewer::new(Rc::clone(&document_obj));
    reviewer.display_stats();
}
