use std::{
    fs::{create_dir_all, read_dir},
    io::Result,
    path::{Path, PathBuf},
};

mod generator;

use crate::generator::GenerateAst;

fn main() {
    println!("Running ast code generation function files");

    const AST_DIR: &str = "./oxa/src/ast/";
    let path = PathBuf::from(AST_DIR);
    create_or_empty_dir(&path).unwrap();

    // Creates the generator
    let generator = GenerateAst::for_path(path).unwrap();

    // Generate expression ast
    let expressions = vec![
        (
            "Binary",
            vec![
                ("left", "Box<dyn Expr<T, V>>"),
                ("operator", "token::Token"),
                ("right", "Box<dyn Expr<T, V>>"),
            ],
        ),
        ("Grouping", vec![("expression", "Box<dyn Expr<T, V>>")]),
        ("Literal", vec![("value", "token::Literal")]),
        (
            "Unary",
            vec![
                ("operator", "token::Token"),
                ("right", "Box<dyn Expr<T, V>>"),
            ],
        ),
        ("Variable", vec![("name", "token::Token")]),
    ];
    generator.define_expr_ast("Expr", &expressions).unwrap();

    // Generate statement ast
    let statements = vec![
        ("Expression", vec![("expression", "Box<dyn Expr<T, V>>")]),
        ("Print", vec![("expression", "Box<dyn Expr<T, V>>")]),
        (
            "Let",
            vec![
                ("name", "token::Token"),
                ("initializer", "Box<dyn Expr<T, V>>"),
            ],
        ),
        (
            "Const",
            vec![
                ("name", "token::Token"),
                ("initializer", "Box<dyn Expr<T, V>>"),
            ],
        ),
    ];
    generator.define_stmt_ast("Stmt", &statements).unwrap();

    // TODO: Use the cli rustfmt to format the file

    println!("Successfully generated ast files");
}

/// Created a directory with the provided path or delete the directory if it exist and has a file
fn create_or_empty_dir(path: &Path) -> Result<()> {
    if read_dir(path).is_ok() {
        return Ok(());
    }
    let res = create_dir_all(path);
    match &res {
        Ok(()) => {}
        Err(err) => {
            eprintln!(
                "Failed to create a directory at location {:?}, encountered error {:?}.  Aborting...",
                path, err
            );
        }
    }

    res
}
