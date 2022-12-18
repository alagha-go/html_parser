impl Document {
    pub fn for_each<'a>(&'a self, mut function: impl FnMut(&'a Document) -> bool) {
        let mut que: VecDeque<Action<&Document>> = VecDeque::new();
        use Action::*;
        que.push_back(Call(self));

        while let Some(action) = que.pop_front() {
            match action {
                Call(document) => {
                    match document {
                        Document::Element(node) => {
                            que.push_back(Handle(document));
                            for document in &node.children {
                                que.push_back(Call(document))
                            }
                        },
                        _ => que.push_back(Handle(document))
                    }
                },
                Handle(document) => {
                    if function(document) {
                        return
                    }
                }
            }
        }
    }

    pub fn get_all_elements_by_id<'a>(&'a self, id: Option<String>) -> Vec<&Document> {
        let mut documents = Vec::new();

        let function = |document: &'a Document| {
            match document {
                Document::Element(node) => {
                    if node.id == id {
                        documents.push(document)
                    }
                    false
                },
                _ => false
            }
        };

        self.for_each(function);
        documents
    }

    pub fn get_element_by_id<'a>(&'a self, nth: usize, id: Option<String>) -> Option<&Document> {
        let mut res = None;
        let mut index = 0;
        let function = |document: &'a Document| {
            match document {
                Document::Element(node) => {
                    if node.id == id {
                        if index == nth {
                            res = Some(document);
                            true
                        }else {
                            index+=1;
                            false
                        }
                    }else {
                        false
                    }
                },
                _ => false
            }
        };
        self.for_each(function);
        res
    }

    pub fn get_all_elements_by_class<'a>(&'a self, class: &String) -> Vec<&Document> {
        let mut documents = Vec::new();

        let function = |document: &'a Document| {
            match document {
                Document::Element(node) => {
                    if node.attributes.get("class") == Some(class) {
                        documents.push(document)
                    }
                    false
                },
                _ => false
            }
        };

        self.for_each(function);
        documents
    }

    pub fn get_element_by_class<'a>(&'a self, nth: usize, class: &String) -> Option<&Document> {
        let mut res = None;
        let mut index = 0;
        let function = |document: &'a Document| {
            match document {
                Document::Element(node) => {
                    if node.attributes.get("class") == Some(class) {
                        if index == nth {
                            res = Some(document);
                            true
                        }else {
                            index+=1;
                            false
                        }
                    }else {
                        false
                    }
                },
                _ => false
            }
        };
        self.for_each(function);
        res
    }
}