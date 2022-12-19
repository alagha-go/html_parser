use pest::{iterators::Pairs, Parser};
use std::collections::HashMap;
use std::collections::VecDeque;
use pest_derive::Parser;
// use self::Result::*;

// type Result<T> = Result<T, dyn std::error::Error + 'static>;

include!("structs.rs");
include!("node.rs");
include!("dom.rs");
include!("document.rs");
include!("error.rs");