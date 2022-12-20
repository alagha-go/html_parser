impl Dom {
    pub fn new(html: &str) -> Result<Self> {
        Self::parse(html)
    }

     pub fn parse(input: &str) -> Result<Self> {
        let pairs = match Grammar::parse(Rule::html, input) {
            Ok(pairs) => pairs,
            Err(error) => return Result::Err(Box::new(Error::new(format!("{}", error)))),
        };
        Self::build_dom(pairs)
    }


    fn build_dom(pairs: Pairs<Rule>) -> Result<Self> {
        let mut dom = Self{documents: Vec::new()};

        for pair in pairs {
            match pair.as_rule() {
                Rule::doctype => (),
                Rule::node_element => match Self::build_element(pair.into_inner(), &mut dom) {
                    Ok(value) => {
                        if let Some(node) = value {
                            dom.documents.push(Document::Element(node));
                        }
                    }
                    Err(error) => return Err(Box::new(Error::new(format!("{}", error))))
                },
                Rule::node_text => {
                    dom.documents.push(Document::Text(pair.as_str().to_string()));
                },
                Rule::node_comment => {
                    dom.documents.push(Document::Comment(pair.into_inner().as_str().to_string()));
                },
                Rule::EOI => (),
                _ => unreachable!("[build dom] unknown rule: {:?}", pair.as_rule()),
            }
        }

        Ok(dom)
    }

    fn build_element(pairs: Pairs<Rule>, dom: &mut Dom) -> Result<Option<Node>> {
        let mut element = Node::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::node_element | Rule::el_raw_text => {
                    match Self::build_element(pair.into_inner(), dom) {
                        Ok(value) => {
                            if let Some(child_element) = value {
                                element.children.push(Document::Element(child_element))
                            }
                        }
                        Err(error) => return Err(Box::new(Error::new(format!("{}", error))))
                    }
                },
                Rule::node_text | Rule::el_raw_text_content => {
                    element.children.push(Document::Text(pair.as_str().to_string()));
                },
                Rule::node_comment => {
                    element
                        .children
                        .push(Document::Comment(pair.into_inner().as_str().to_string()));
                },
                Rule::el_name | Rule::el_void_name | Rule::el_raw_text_name => {
                    element.name = pair.as_str().to_string();
                },
                Rule::attr => match Self::build_attribute(pair.into_inner()) {
                    Ok((attr_key, attr_value)) => {
                        match attr_value {
                            None => (),
                            Some(value) => {
                                element.attributes.insert(attr_key, value);
                            }
                        }
                    }
                    Err(_error) => {
                        ()
                    }
                },
                Rule::el_normal_end | Rule::el_raw_text_end => {
                    break;
                }
                Rule::el_dangling => (),
                Rule::EOI => (),
                _ => return Err(Box::new(Error::new(format!("unexpected token: {:?}", pair))))
            }
        }
        Ok(Some(element))
    }



    fn build_attribute(pairs: Pairs<Rule>) -> Result<(String, Option<String>)> {
        let mut attribute = ("".to_string(), None);
        for pair in pairs {
            match pair.as_rule() {
                Rule::attr_key => {
                    attribute.0 = pair.as_str().to_string();
                }
                Rule::attr_non_quoted => {
                    attribute.1 = Some(pair.as_str().to_string());
                }
                Rule::attr_quoted => {
                    let inner_pair = pair
                        .into_inner()
                        .into_iter()
                        .next()
                        .expect("attribute value");

                    match inner_pair.as_rule() {
                        Rule::attr_value => attribute.1 = Some(inner_pair.as_str().to_string()),
                        _ => {
                            ()
                        }
                    }
                }
                _ => {
                    ()
                }
            }
        }
        Ok(attribute)
    }


}