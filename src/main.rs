extern crate html5ever;
extern crate postgres;

use std::{fs, env, io, str};
use std::default::Default;

use crate::postgres::{Connection, TlsMode};

use crate::html5ever::parse_document;
use crate::html5ever::rcdom::{Handle, NodeData, RcDom};
use crate::html5ever::tendril::TendrilSink;

fn walk(conn: &Connection, node: &Handle) {
    match node.data {
        NodeData::Document => println!("Document"),
        NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

        NodeData::Text { contents: _ } => {
        },

        NodeData::Comment { ref contents } => {
            conn.execute("INSERT INTO elements (comment) VALUES ($1)", &[&contents.to_string()]).unwrap();
        }

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            conn.execute("INSERT INTO elements (name) VALUES ($1)", &[&name.local.to_string()]).unwrap();

            print!("<{}", name.local);

            for attr in attrs.borrow().iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        },

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(&conn, child)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let DATABASE_URL = env::var("HTML_DATABASE_URL").unwrap();
    let conn = Connection::connect(DATABASE_URL, TlsMode::None).unwrap();

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .from_file(&args[1])
        .unwrap();

    walk(&conn, &dom.document);

    if !dom.errors.is_empty() {
        for err in dom.errors.iter() {
            println!("Error: {}", err)
        }
    }
}
