#![allow(dead_code, unused)]
use pest::{iterators::Pair, iterators::Pairs, Parser};
use std::collections::*;
use pest_derive::Parser;
use std::*;

include!("structs.rs");
include!("document.rs");
include!("element.rs");
include!("dom.rs");

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;