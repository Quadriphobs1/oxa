#![feature(io_error_more)]

use std::io::{Error, ErrorKind};
use std::{
    fs::{create_dir_all, read_dir, remove_dir_all, File},
    io::{LineWriter, Result, Write},
    path::{Path, PathBuf},
};

/// Generates dummy ast file with some code function, defines default struct and methods
///
/// # Example of generated file
/// ```
/// use crate::token::Token;
///
/// trait Expr {
///     fn accept(&self, visitor: Visitor) {}
/// }
///
/// struct Visitor {}
///
/// impl Visitor {
///     pub fn visit_binary_expr(&self, expr: &Binary) {}
///
///     pub fn visit_literal_expr(&self, expr: &Literal) {}
/// }
///
/// pub struct Binary {
///     left: Box<dyn Expr + 'static>,
///     operator: Token,
///     right: Box<dyn Expr + 'static>,
/// }
///
/// impl Expr for Binary {
///     fn accept(&self, visitor: Visitor) {
///         return visitor.visit_binary_expr(self);
///     }
/// }
/// ```
pub struct GenerateAst {
    out_dir: PathBuf,
}

impl GenerateAst {
    /// Creates struct for ready to be used for the provided path
    /// checks if the `path` exist and empty, error if not
    ///
    /// # Example
    ///
    /// ```
    /// let path = PathBuf::from("/some/dir");
    /// let generator = GenerateAst::for_path(path);
    /// ```
    ///
    /// # Errors
    ///
    /// This function check for the existence of the provided path and ensures the path is empty.
    /// * The provided `path` doesn't exist
    /// * The `path` points at a non-directory file.
    /// * The provided directory is not empty
    pub fn for_path(path: PathBuf) -> Result<GenerateAst> {
        // Check if the path exist
        let mut dir = read_dir(&path)?;
        if dir.next().is_some() {
            return Err(Error::new(
                ErrorKind::DirectoryNotEmpty,
                "empty the directory to continue",
            ));
        }
        Ok(GenerateAst { out_dir: path })
    }
}

impl GenerateAst {
    pub fn define_ast(
        &self,
        base_name: &str,
        types: &Vec<(&str, Vec<(&str, &str)>)>,
    ) -> Result<()> {
        let dir_str = self.out_dir.to_str();
        if dir_str.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "path cannot be empty"));
        }
        let file_path = format!("{}/{}.rs", &dir_str.unwrap(), &base_name.to_lowercase());
        let file = File::create(&file_path)?;
        let mut writer = LineWriter::new(file);

        writer.write_all(b"use crate::token::Token;")?;
        writer.write_all(b"\n\n")?;

        // Expr
        self.define_trait(&mut writer, base_name)?;

        // Visitor
        self.define_visitor(&mut writer, base_name, types)?;

        // Types
        self.define_types(&mut writer, base_name, types)?;

        writer.write_all(b"\n")?;
        writer.flush()?;

        Ok(())
    }

    fn define_trait(&self, writer: &mut LineWriter<File>, base_name: &str) -> Result<()> {
        writer.write_all(format!("pub trait {} {{", base_name).as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    fn accept(&self, visitor: Visitor) {}")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;

        Ok(())
    }

    fn define_visitor(
        &self,
        writer: &mut LineWriter<File>,
        base_name: &str,
        types: &Vec<(&str, Vec<(&str, &str)>)>,
    ) -> Result<()> {
        writer.write_all(b"pub struct Visitor {}")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"impl Visitor {")?;
        writer.write_all(b"\n")?;

        for (struct_name, ..) in types {
            writer.write_all(
                format!(
                    "  pub fn visit_{}_{}(&self, expr: &{}) {{}}",
                    struct_name.to_lowercase(),
                    base_name.to_lowercase(),
                    struct_name
                )
                .as_bytes(),
            )?;
            writer.write_all(b"\n")?;
        }

        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;
        Ok(())
    }

    fn define_types(
        &self,
        writer: &mut LineWriter<File>,
        base_name: &str,
        types: &Vec<(&str, Vec<(&str, &str)>)>,
    ) -> Result<()> {
        // The AST structs.
        for (struct_name, struct_fields) in types {
            self.define_type(writer, base_name, struct_name, &struct_fields)?;
        }

        Ok(())
    }

    fn define_type(
        &self,
        writer: &mut LineWriter<File>,
        base_name: &str,
        struct_name: &str,
        struct_fields: &Vec<(&str, &str)>,
    ) -> Result<()> {
        writer.write_all(format!("pub struct {} {{", struct_name).as_bytes())?;
        writer.write_all(b"\n")?;

        // Store parameters in fields.
        for (name, field_type) in struct_fields {
            writer.write_all(format!("    {}: {},", name, field_type).as_bytes())?;
            writer.write_all(b"\n")?;
        }

        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;
        writer.write_all(format!("impl {} for {} {{", base_name, struct_name).as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    fn accept(&self, visitor: Visitor) {")?;
        writer.write_all(b"\n")?;
        writer.write_all(
            format!(
                "        return visitor.visit_{}_{}();",
                struct_name.to_lowercase(),
                base_name.to_lowercase()
            )
            .as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    }")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;
        Ok(())
    }
}

fn main() {
    // Setup
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 1 {
    //     eprintln!("Usage: generate_ast <output directory>");
    //     std::process::exit(64);
    // }

    const AST_DIR: &str = "./src/generated/";
    let path = PathBuf::from(AST_DIR);
    create_or_empty_dir(&path).unwrap();

    // Creates the generator
    let generator = GenerateAst::for_path(path).unwrap();

    // Generate expression ast
    let expressions = vec![
        (
            "Binary",
            vec![
                ("left", "dyn Box<dyn Expr + 'static>"),
                ("operator", "Token"),
                ("right", "dyn Box<dyn Expr + 'static>"),
            ],
        ),
        (
            "Grouping",
            vec![("expression", "dyn Box<dyn Expr + 'static>")],
        ),
        // TODO: Fix Object to struct field mapping
        ("Literal", vec![("value", "String")]),
        (
            "Unary",
            vec![
                ("operator", "Token"),
                ("right", "dyn Box<dyn Expr + 'static>"),
            ],
        ),
    ];
    generator.define_ast("Expr", &expressions).unwrap();
    // TODO: Use the cli rustfmt to format the file

    println!("Running build function file")
}

/// Created a directory with the provided path or delete the directory if it exist and has a file
fn create_or_empty_dir(path: &Path) -> Result<()> {
    if read_dir(&path).is_ok() {
        remove_dir_all(path).unwrap()
    }
    let res = create_dir_all(&path);
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
