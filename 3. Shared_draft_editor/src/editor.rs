use crate::document::Document;
use std::{cell::RefCell, rc::Rc};

pub struct TitleEditor {
    operator: Rc<RefCell<Document>>,
}

impl TitleEditor {
    pub fn new(operator: Rc<RefCell<Document>>) -> Self {
        Self { operator }
    }
    pub fn update_title(&self) {
        self.operator.borrow_mut().update_title("new updated title");
    }
}

pub struct BodyEditor {
    operator: Rc<RefCell<Document>>,
}
impl BodyEditor {
    pub fn new(operator: Rc<RefCell<Document>>) -> Self {
        Self { operator }
    }

    pub fn append_paragraph(&self) {
        self.operator
            .borrow_mut()
            .append_body("\nmy new paragraph appended to the body\n");
    }
}

pub struct Reviewer {
    operator: Rc<RefCell<Document>>,
}

impl Reviewer {
    pub fn new(operator: Rc<RefCell<Document>>) -> Self {
        Self { operator }
    }

    pub fn display_stats(&self) {
        self.operator.borrow().display();
    }
}
