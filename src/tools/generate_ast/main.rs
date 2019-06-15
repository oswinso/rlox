use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        std::process::exit(1);
    }

    let path = &args[1];

    define_ast(
        path,
        vec![
            (
                "Expr",
                vec![
                    "Binary     : Expr left, Token operator, Expr right",
                    "Grouping   : Expr expression",
                    "Literal    : Literal value",
                    "Unary      : Token operator, Expr right",
                ],
            ),
            (
                "Literal",
                vec!["String    : String string", "Number    : f64 number"],
            ),
        ],
    );
}

fn define_ast(path: &str, types: Vec<(&str, Vec<&str>)>) {
    println!("Path: {}", path);
    let file = File::create(path).unwrap_or(File::open(path).unwrap());
    let mut writer = BufWriter::new(file);
    writer
        .write_all(b"use crate::front::token::Token;\n\n")
        .unwrap();

    for (base_name, definition) in types {
        writer
            .write_all(format!("pub enum {} {{\n", base_name).as_bytes())
            .unwrap();

        for typedef in definition {
            let vec: Vec<&str> = typedef.split(":").collect::<Vec<&str>>();
            let (class_name, fields) = (vec[0].trim(), vec[1].trim());
            define_type(&mut writer, base_name, class_name, fields);
        }
        writer.write(b"}\n\n").unwrap();
    }
}

fn define_type(writer: &mut BufWriter<File>, base_name: &str, class_name: &str, fields: &str) {
    writer
        .write(format!("    {} {{", class_name).as_bytes())
        .unwrap();

    let mut field_string = Vec::new();
    for field in fields.split(", ") {
        let vec: Vec<&str> = field.split(" ").collect::<Vec<&str>>();
        let (field_type, field_name) = (vec[0], vec[1]);
        field_string.push(format!("{}: Box<{}>", field_name, field_type));
    }
    writer
        .write(format!("{}}},\n", field_string.join(", ")).as_bytes())
        .unwrap();
}
