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

    pub fn get_all_elements_by_id<'a>(&'a self, id: &String) -> Vec<&Document> {
        let mut documents = Vec::new();

        let function = |document: &'a Document| {
            match document {
                Document::Element(node) => {
                    if node.attributes.get("id") == Some(id) {
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

    pub fn get_element_by_id<'a>(&'a self, nth: usize, id: &String) -> Option<&Document> {
        let mut res = None;
        let mut index = 0;
        let function = |document: &'a Document| {
            match document {
                Document::Element(node) => {
                    if node.attributes.get("id") == Some(id) {
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

    pub fn get_all_elements_by_name<'a>(&'a self, name: &String) -> Vec<&Document> {
        let mut documents = Vec::new();

        let function = |document: &'a Document| {
            match document {
                Document::Element(node) => {
                    if node.name == *name {
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

    pub fn get_element_by_name<'a>(&'a self, name: &String, nth: usize) -> Option<&Document> {
        let mut res = None;
        let mut index = 0;
        let function = |document: &'a Document| {
            match document {
                Document::Element(node) => {
                    if node.name == *name {
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

    pub fn text<'a>(&'a self) -> Option<&String> {
        match self {
            Self::Comment(_) => None,
            Self::Text(value) => Some(value),
            Self::Element(node) => {
                for child in &node.children {
                    match child {
                        Self::Text(value) => return Some(value),
                        _ => ()
                    }
                }
                None
            }
        }
    }

    pub fn comment<'a>(&'a self) -> Option<&String> {
        match self {
            Self::Text(_) => None,
            Self::Comment(comment) => Some(comment),
            Self::Element(node) => {
                for child in &node.children {
                    match child {
                        Self::Comment(comment) => return Some(comment),
                        _ => ()
                    }
                }
                None
            }
        }
    }
}