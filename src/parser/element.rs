impl Element {
    pub fn build_attribute(pairs: Pairs<Rule>) -> Result<(String, Option<String>)> {
        let mut attribute = ("".to_string(), None);
        for pair in pairs {
            match pair.as_rule() {
                Rule::attribute_key => {
                    attribute.0 = pair.as_str().to_string();
                }
                Rule::attribute_non_quoted => {
                    attribute.1 = Some(pair.as_str().to_string());
                }
                Rule::attribute_quoted => {
                    let inner_pair = pair
                        .into_inner()
                        .into_iter()
                        .next()
                        .expect("attribute value");

                    match inner_pair.as_rule() {
                        Rule::attribute_value => attribute.1 = Some(inner_pair.as_str().to_string()),
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


impl<'i> TryFrom<Pairs<'i, Rule>> for Element {
    type Error = Box<dyn std::error::Error>;
    fn try_from(pairs: Pairs<Rule>) -> Result<Self> {
        let mut element = Self::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::element | Rule::raw_text => {
                    element.children.push(Document::Element(pair.into_inner().try_into()?));
                },
                Rule::text | Rule::raw_text_content => {
                    element.children.push(Document::Text(pair.as_str().to_string()));
                },
                Rule::comment => {
                    element.children.push(Document::Comment(pair.into_inner().as_str().to_string()));
                },
                Rule::name | Rule::void_name | Rule::raw_text_name => {
                    element.name = pair.as_str().to_string();
                },
                Rule::attribute => {
                    let (key, value) = Self::build_attribute(pair.into_inner())?;
                    match value {
                        Some(value) => {
                            element.attributes.insert(key, value);
                        },
                        None => ()
                    }
                }
                _ => ()
            }
        }
        Ok(element)
    }
}