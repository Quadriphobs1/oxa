#![feature(io_error_more)]

use std::io::{Error, ErrorKind};
use std::{
    fs::{create_dir_all, read_dir, File},
    io::{LineWriter, Result, Write},
    path::{Path, PathBuf},
};

/// Generates dummy ast file with some code function, defines default struct and methods
///
/// # Example of generated file
/// ```
/// use crate::token::Token;
///
/// trait Expr<T> {
///     fn accept(&self, visitor: &impl Visitor<T>) -> T;
/// }
///
/// trait Visitor<T> {
///     fn visit_binary_expr(&self, expr: &Binary) T;
/// }
///
/// pub struct Binary<T> {
///     left: Box<dyn Expr<T> + 'static>,
///     operator: Token,
///     right: Box<dyn Expr<T> + 'static>,
///     _marker: marker::PhantomData<T>,
/// }
///
/// impl <T> for Binary<T> {
///     fn new(left: ) -> Self {
///
///     }
/// }
/// impl<T> Expr<T> for Binary<T {
///     fn accept(&self, visitor: &Box<dyn Visitor<T>>) -> T {
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
    /// This function check for the existence of the provided path.
    /// * The provided `path` doesn't exist
    /// * The `path` points at a non-directory file.
    pub fn for_path(path: PathBuf) -> Result<GenerateAst> {
        // Check if the path exist
        let mut dir = read_dir(&path)?;
        if dir.next().is_none() {
            return Err(Error::new(
                ErrorKind::NotADirectory,
                "directory doesn't exist",
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

        writer.write_all(b"use std::marker;\n")?;
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
        writer.write_all(format!("pub trait {}<T, V: Visitor<T>> {{", base_name).as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(b"\tfn accept(&self, visitor: &V) -> T;")?;
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
        writer.write_all(b"pub trait Visitor<T> {")?;
        writer.write_all(b"\n")?;

        for (struct_name, ..) in types {
            writer.write_all(
                format!(
                    "\tfn visit_{}_{}(&self, expr: &{}<T, Self>) -> T;",
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
        writer.write_all(format!("pub struct {}<T, V: ?Sized> {{", struct_name).as_bytes())?;
        writer.write_all(b"\n")?;

        // Store parameters in fields.
        for (name, field_type) in struct_fields {
            writer.write_all(format!("\tpub {}: {},", name, field_type).as_bytes())?;
            writer.write_all(b"\n")?;
        }

        writer.write_all(b"\t_marker_1: marker::PhantomData<T>,")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"\t_marker_2: marker::PhantomData<V>,")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;

        writer.write_all(format!("impl<T, V> {struct_name}<T, V> {{").as_bytes())?;
        writer.write_all(b"\n")?;
        // Pass arguments to constructor
        let arguments = struct_fields
            .into_iter()
            .map(|(a, b)| format!("{}: {}", a, b))
            .collect::<Vec<String>>()
            .join(", ");
        writer.write_all(format!("\tpub fn new({}) -> Self  {{", arguments).as_bytes())?;
        writer.write_all(b"\n")?;

        writer.write_all(format!("\t\t{} {{", struct_name).as_bytes())?;
        writer.write_all(b"\n")?;
        for (name, ..) in struct_fields {
            writer.write_all(format!("\t\t\t{},", name).as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.write_all(b"\t\t\t_marker_1: marker::PhantomData::default(),")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"\t\t\t_marker_2: marker::PhantomData::default(),")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"\t\t}")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"\t}")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;
        writer.write_all(
            format!(
                "impl<T, V: Visitor<T>> {}<T, V> for {}<T, V> {{",
                base_name, struct_name
            )
            .as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(b"\tfn accept(&self, visitor: &V) -> T {")?;
        writer.write_all(b"\n")?;
        writer.write_all(
            format!(
                "\t\treturn visitor.visit_{}_{}(self);",
                struct_name.to_lowercase(),
                base_name.to_lowercase()
            )
            .as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(b"\t}")?;
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

    const AST_DIR: &str = "./src/ast/";
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
                ("operator", "Token"),
                ("right", "Box<dyn Expr<T, V>>"),
            ],
        ),
        ("Grouping", vec![("expression", "Box<dyn Expr<T, V>>")]),
        // TODO: Fix Object to struct field mapping
        ("Literal", vec![("value", "String")]),
        (
            "Unary",
            vec![("operator", "Token"), ("right", "Box<dyn Expr<T, V>>")],
        ),
    ];
    generator.define_ast("Expr", &expressions).unwrap();
    // TODO: Use the cli rustfmt to format the file

    println!("Running build function file")
}

/// Created a directory with the provided path or delete the directory if it exist and has a file
fn create_or_empty_dir(path: &Path) -> Result<()> {
    if read_dir(&path).is_ok() {
        return Ok(());
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
