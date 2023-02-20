use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            define_ast(
                &args[1],
                "expr".into(),
                vec![
                    "Binary   : Expr left, Token operator, Expr right".into(),
                    "Grouping : Expr expression".into(),
                    "Literal  : Object value".into(),
                    "Unary    : Token operator, Expr right".into(),
                ],
            );
        }
        _ => {
            println!("Usage: generate_ast <output directory>");
        }
    }
}

fn define_ast(output_directory: &str, base_name: String, types: Vec<String>) {
    let path = output_directory.to_owned() + "/" + &base_name + ".rs";
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);

    writer.write(b"trait Visitor<T> { \n").unwrap();

    for tp in types {
        let slices: Vec<&str> = tp.split(":").collect();
        let struct_name = slices.get(0).unwrap().to_owned().trim();
        let fields = slices.get(1).unwrap().to_owned().trim();

        define_type(&mut writer, &base_name, struct_name, fields);
    }

    writer.write(b"}\n").unwrap();
    writer.flush().unwrap();
}

fn define_type(writer: &mut BufWriter<File>, base_name: &str, struct_name: &str, field_list: &str) {
    writeln!(writer, "struct {} {{", struct_name).unwrap();

    let fields: Vec<&str> = field_list.split(", ").collect();

    for field in fields {
        let splitted_field: Vec<&str> = field.split(" ").collect();
        let field_type = splitted_field[0];
        let field_name = splitted_field[1];
        writeln!(writer, "{}: {}", field_name, field_type).unwrap();
    }

    writeln!(writer, "}}\n").unwrap();
}
