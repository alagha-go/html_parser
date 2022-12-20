use pest::{iterators::Pairs, Parser};
use std::collections::HashMap;
use std::collections::VecDeque;
use pest_derive::Parser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

include!("structs.rs");
include!("node.rs");
include!("dom.rs");
include!("document.rs");