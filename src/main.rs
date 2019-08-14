extern crate html5ever;
extern crate redis;

use std::env;
use std::string::String;
use std::default::Default;

use redis::Commands;

use crate::html5ever::parse_document;
use crate::html5ever::rcdom::{Handle, NodeData, RcDom};
use crate::html5ever::tendril::TendrilSink;

fn walk(conn: &mut redis::Connection, node: &Handle) {
    match node.data {
        NodeData::Document => {},
        NodeData::Doctype { .. } => {},
        NodeData::Text { .. } => {},
        NodeData::Comment { .. } => {},

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let element_name = name.local.to_lowercase();

            redis::cmd("INCR")
                .arg(format!("element_count:{}", element_name))
                .execute(conn);

            for attr in attrs.borrow().iter() {
                redis::cmd("INCR")
                    .arg(format!("attribute_count:{}:{}", element_name, attr.name.local.to_lowercase()))
                    .execute(conn);
            }
        },

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(conn, child)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let REDIS_URL = env::var("HTML_REDIS_URL").unwrap();
    let client = redis::Client::open(&*REDIS_URL).unwrap();
    let mut conn = client.get_connection().unwrap();

    redis::cmd("INCR").arg("page_count").execute(&mut conn);

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .from_file(&args[1])
        .unwrap();

    walk(&mut conn, &dom.document);
}
