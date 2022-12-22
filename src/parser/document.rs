impl Document {
    pub fn new<'a>(html: &str) -> Result<Self> {
        let pairs = match HtmlParser::parse(Rule::html, html) {
            Ok(pairs) => pairs,
            Err(err) => return Err(err)?,
        };
        Ok(pairs.try_into()?)
    }


    pub fn for_each<'a>(&'a self, mut function: impl FnMut(&'a Document) -> bool) {
        let mut que: VecDeque<Action<&Document>> = VecDeque::new();
        use Action::*;
        que.push_back(Call(self));

        while let Some(action) = que.pop_front() {
            match action {
                Call(document) => {
                    match document {
                        Document::Element(element) => {
                            que.push_back(Handle(document));
                            for document in &element.children {
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
                Document::Element(element) => {
                    if element.attributes.get("id") == Some(id) {
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
                Document::Element(element) => {
                    if element.attributes.get("id") == Some(id) {
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
                Document::Element(element) => {
                    if element.attributes.get("class") == Some(class) {
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
                Document::Element(element) => {
                    if element.attributes.get("class") == Some(class) {
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
                Document::Element(element) => {
                    if element.name == *name {
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
                Document::Element(element) => {
                    if element.name == *name {
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

    pub fn get_all_elements_by_attribute<'a>(&'a self, key: &str, value: &String) -> Vec<&Document> {
        let mut documents = Vec::new();

        let function = |document: &'a Document| {
            match document {
                Self::Element(element) => {
                    if element.attributes.get(key) == Some(value) {
                        documents.push(document)
                    }
                    false
                },
                _ => false,
            }
        };

        self.for_each(function);
        documents
    }

    pub fn get_element_by_attribute<'a>(&'a self, nth: usize, key: &str, value: &String) -> Option<&Document> {
        let mut res = None;
        let mut index = 0;

        let function = |document: &'a Document| {
            match document {
                Self::Element(element) => {
                    if element.attributes.get(key) == Some(value) {
                        if nth == index {
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
                _ => false,
            }
        };

        self.for_each(function);
        res
    }

    pub fn attribute<'a>(&self, key: &str) -> Option<&String> {
        match self {
            Document::Element(element) => {
                element.attributes.get(key)
            },
            _ => None
        }
    }

    pub fn child_attribute<'a>(&self, nth: usize, key: &str) -> Option<&String> {
        match self.children() {
            Some(children) => {
                let mut index = 0;
                for child in children {
                    match child.attributes() {
                        Some(attributes) => {
                            match attributes.get(key) {
                                Some(attribute) => {
                                    if index == nth {
                                        return Some(attribute)
                                    }
                                    index+=1;
                                },
                                _ => ()
                            }
                        }
                        _ => ()
                    }
                }
                None
            },
            None => None,
        }
    }

    pub fn text<'a>(&'a self) -> Option<&String> {
        match self {
            Self::Comment(_) => None,
            Self::Text(value) => Some(value),
            Self::Element(element) => {
                for child in &element.children {
                    match child {
                        Self::Text(value) => return Some(value),
                        _ => ()
                    }
                }
                None
            },
            Self::None => None
        }
    }

    pub fn comment<'a>(&'a self) -> Option<&String> {
        match self {
            Self::Text(_) => None,
            Self::Comment(comment) => Some(comment),
            Self::Element(element) => {
                for child in &element.children {
                    match child {
                        Self::Comment(comment) => return Some(comment),
                        _ => ()
                    }
                }
                None
            },
            Self::None => None
        }
    }

    pub fn name<'a>(&'a self) -> Option<&String> {
        match self {
            Self::Element(element) => Some(&element.name),
            _ => None
        }
    }

    pub fn attributes<'a>(&'a self) -> Option<&Attributes> {
        match self {
            Self::Element(element) => Some(&element.attributes),
            _ => None
        }
    }

    pub fn children<'a>(&'a self) -> Option<&Vec<Document>> {
        match self {
            Self::Element(element) => Some(&element.children),
            _ => None
        }
    }

    pub fn is_element(&self) -> bool {
        match self {
            Self::Element(_) => true,
            _ => false
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            Self::Text(_) => true,
            _ => false
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            Self::Comment(_) => true,
            _ => false
        }
    }

    pub fn id<'a>(&'a self) -> Option<&String> {
        match self {
            Self::Element(element) => element.attributes.get("id"),
            _ => None
        }
    }

    pub fn class<'a>(&'a self) -> Option<&String> {
        match self {
            Self::Element(element) => element.attributes.get("class"),
            _ => None
        }
    }
}


impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = String::new();
        match self {
            Self::Element(element) => {
                let string = format!("{element:#?}");
                debug.push_str(&string);
            },
            Self::Text(text) => {
                let string = format!("{text:?}");
                debug.push_str(&string);
            },
            Self::Comment(comment) => {
                let string = format!("<!-- {comment:?} -->");
                debug.push_str(&string);
            },
            Self::None => ()
        }
        write!(f, "{debug}", )
    }
}


impl<'i> TryFrom<Pairs<'i, Rule>> for Document {
    type Error = Box<dyn std::error::Error>;
    fn try_from(pairs: Pairs<Rule>) -> Result<Self> {
        let mut document = Document::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::comment => {
                    document = Document::Comment(pair.into_inner().as_str().to_string());
                    break
                },
                Rule::text => {
                    document = Document::Text(pair.as_str().to_string());
                    break
                },
                Rule::element => {
                    document = Document::Element(pair.into_inner().try_into()?);
                    break
                },
                _ => ()
            }
        }
        Ok(document)
    }
}