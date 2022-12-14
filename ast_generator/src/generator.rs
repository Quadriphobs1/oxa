use std::io::{Error, ErrorKind};
use std::{
    fs::{read_dir, File},
    io::{LineWriter, Result, Write},
    path::PathBuf,
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

/// public methods
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
                ErrorKind::InvalidInput,
                "directory doesn't exist",
            ));
        }
        Ok(GenerateAst { out_dir: path })
    }
}

impl GenerateAst {
    /// Expression ast generator
    ///
    /// # Expression Grammar
    /// ```
    /// expression     → literal
    ///                | unary
    ///                | binary
    ///                | grouping ;
    ///
    /// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
    /// grouping       → "(" expression ")" ;
    /// unary          → ( "-" | "!" ) expression ;
    /// binary         → expression operator expression ;
    /// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
    ///                | "+"  | "-"  | "*" | "/" ;
    /// ```
    pub fn define_expr_ast(
        &self,
        base_name: &str,
        types: &Vec<(&str, Vec<(&str, &str)>)>,
    ) -> Result<()> {
        let dir_str = self.out_dir.to_str();
        if dir_str.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "path cannot be empty"));
        }
        let file_path = format!("{}/{}.rs", &dir_str.unwrap(), &base_name.to_lowercase());
        let file = File::create(file_path)?;
        let mut writer = LineWriter::new(file);

        writer.write_all(b"use crate::token;\n")?;
        writer.write_all(b"use std::fmt::{Display, Formatter, Result};\n")?;
        writer.write_all(b"use std::marker;\n")?;
        writer.write_all(b"\n")?;

        // Expr
        self.define_trait(&mut writer, base_name, "<T, V: Visitor<T>>", "&V")?;

        // Visitor
        self.define_expr_visitor(&mut writer, base_name, types)?;

        // Types
        self.define_expr_types(&mut writer, base_name, types)?;
        writer.flush()?;

        Ok(())
    }

    /// Statement ast generator
    ///
    /// # Statement Grammar
    /// ```
    /// program        → statement* EOF ;
    ///
    /// statement      → exprStmt
    ///                | printStmt ;
    ///
    /// exprStmt       → expression ";" ;
    /// printStmt      → "print" expression ";" ;
    /// ```
    pub fn define_stmt_ast(
        &self,
        base_name: &str,
        types: &Vec<(&str, Vec<(&str, &str)>)>,
    ) -> Result<()> {
        let dir_str = self.out_dir.to_str();
        if dir_str.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "path cannot be empty"));
        }
        let file_path = format!("{}/{}.rs", &dir_str.unwrap(), &base_name.to_lowercase());
        let file = File::create(file_path)?;
        let mut writer = LineWriter::new(file);

        writer.write_all(b"use crate::ast::expr::Expr;\n")?;
        writer.write_all(b"use crate::token;\n")?;
        writer.write_all(b"use std::fmt::{Display, Formatter, Result};\n")?;
        writer.write_all(b"use std::marker;\n")?;
        writer.write_all(b"\n")?;

        // Expr
        self.define_trait(&mut writer, base_name, "<T, U: Visitor<T, V>, V>", "&mut U")?;

        // Visitor
        self.define_stmt_visitor(&mut writer, base_name, types)?;

        // Types
        self.define_stmt_types(&mut writer, base_name, types)?;
        writer.flush()?;

        Ok(())
    }
}

/// private methods
impl GenerateAst {
    fn define_trait(
        &self,
        writer: &mut LineWriter<File>,
        base_name: &str,
        generic: &str,
        visitor: &str,
    ) -> Result<()> {
        writer.write_all(format!("pub trait {}{}: Display {{", base_name, generic).as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(format!("    fn accept(&self, visitor: {}) -> T;", visitor).as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;

        Ok(())
    }

    fn define_stmt_visitor(
        &self,
        writer: &mut LineWriter<File>,
        base_name: &str,
        types: &Vec<(&str, Vec<(&str, &str)>)>,
    ) -> Result<()> {
        writer.write_all(b"pub trait Visitor<T, V> {")?;
        writer.write_all(b"\n")?;

        for (struct_name, ..) in types {
            writer.write_all(
                format!(
                    "    fn visit_{}_{}(&mut self, {}: &{}<T, Self, V>) -> T;",
                    struct_name.to_lowercase(),
                    base_name.to_lowercase(),
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

    fn define_expr_visitor(
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
                    "    fn visit_{}_{}(&self, {}: &{}<T, Self>) -> T;",
                    struct_name.to_lowercase(),
                    base_name.to_lowercase(),
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

    fn define_stmt_types(
        &self,
        writer: &mut LineWriter<File>,
        base_name: &str,
        types: &Vec<(&str, Vec<(&str, &str)>)>,
    ) -> Result<()> {
        // The AST structs.
        for (struct_name, struct_fields) in types {
            self.define_stmt_type(writer, base_name, struct_name, struct_fields)?;
        }

        Ok(())
    }

    fn define_stmt_type(
        &self,
        writer: &mut LineWriter<File>,
        base_name: &str,
        struct_name: &str,
        struct_fields: &Vec<(&str, &str)>,
    ) -> Result<()> {
        // struct definition
        writer.write_all(
            format!("pub struct {}<T, U: ?Sized, V: ?Sized> {{", struct_name).as_bytes(),
        )?;
        writer.write_all(b"\n")?;

        // Store parameters in fields.
        for (name, field_type) in struct_fields {
            writer.write_all(format!("    pub {}: {},", name, field_type).as_bytes())?;
            writer.write_all(b"\n")?;
        }

        writer.write_all(b"    _marker_1: marker::PhantomData<T>,")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    _marker_2: marker::PhantomData<U>,")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    _marker_3: marker::PhantomData<V>,")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;

        // struct constructor
        writer.write_all(format!("impl<T, U, V> {struct_name}<T, U, V> {{").as_bytes())?;
        writer.write_all(b"\n")?;
        // Pass arguments to constructor
        let arguments = struct_fields
            .iter()
            .map(|(a, b)| format!("{}: {}", a, b))
            .collect::<Vec<String>>()
            .join(", ");
        writer.write_all(format!("\tpub fn new({}) -> Self {{", arguments).as_bytes())?;
        writer.write_all(b"\n")?;

        writer.write_all(format!("        {} {{", struct_name).as_bytes())?;
        writer.write_all(b"\n")?;
        for (name, ..) in struct_fields {
            writer.write_all(format!("            {},", name).as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.write_all(b"            _marker_1: marker::PhantomData::default(),")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"            _marker_2: marker::PhantomData::default(),")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"            _marker_3: marker::PhantomData::default(),")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"        }")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    }")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;

        // struct trait impl
        writer.write_all(
            format!(
                "impl<T, U: Visitor<T, V>, V> {}<T, U, V> for {}<T, U, V> {{",
                base_name, struct_name
            )
            .as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    fn accept(&self, visitor: &mut U) -> T {")?;
        writer.write_all(b"\n")?;
        writer.write_all(
            format!(
                "\t\tvisitor.visit_{}_{}(self)",
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

        // struct display trait impl
        writer.write_all(
            format!(
                "impl<T, U: Visitor<T, V>, V> Display for {}<T, U, V> {{",
                struct_name
            )
            .as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    fn fmt(&self, f: &mut Formatter<'_>) -> Result {")?;
        writer.write_all(b"\n")?;

        let inner_brace: String = struct_fields
            .iter()
            .map(|_| "{}".to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let field_ref: String = struct_fields
            .iter()
            .map(|(a, _)| format!("self.{}", a))
            .collect::<Vec<_>>()
            .join(", ");

        writer.write_all(
            format!("        write!(f, \"{}\", {})", inner_brace, field_ref,).as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    }")?;

        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;
        writer.write_all(b"\n\n")?;
        Ok(())
    }

    fn define_expr_types(
        &self,
        writer: &mut LineWriter<File>,
        base_name: &str,
        types: &Vec<(&str, Vec<(&str, &str)>)>,
    ) -> Result<()> {
        // The AST structs.
        for (struct_name, struct_fields) in types {
            self.define_expr_type(writer, base_name, struct_name, struct_fields)?;
        }

        Ok(())
    }

    fn define_expr_type(
        &self,
        writer: &mut LineWriter<File>,
        base_name: &str,
        struct_name: &str,
        struct_fields: &Vec<(&str, &str)>,
    ) -> Result<()> {
        // struct definition
        writer.write_all(format!("pub struct {}<T, V: ?Sized> {{", struct_name).as_bytes())?;
        writer.write_all(b"\n")?;

        // Store parameters in fields.
        for (name, field_type) in struct_fields {
            writer.write_all(format!("    pub {}: {},", name, field_type).as_bytes())?;
            writer.write_all(b"\n")?;
        }

        writer.write_all(b"    _marker_1: marker::PhantomData<T>,")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    _marker_2: marker::PhantomData<V>,")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;

        // struct constructor
        writer.write_all(format!("impl<T, V> {struct_name}<T, V> {{").as_bytes())?;
        writer.write_all(b"\n")?;
        // Pass arguments to constructor
        let arguments = struct_fields
            .iter()
            .map(|(a, b)| format!("{}: {}", a, b))
            .collect::<Vec<String>>()
            .join(", ");
        writer.write_all(format!("\tpub fn new({}) -> Self  {{", arguments).as_bytes())?;
        writer.write_all(b"\n")?;

        writer.write_all(format!("        {} {{", struct_name).as_bytes())?;
        writer.write_all(b"\n")?;
        for (name, ..) in struct_fields {
            writer.write_all(format!("            {},", name).as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.write_all(b"            _marker_1: marker::PhantomData::default(),")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"            _marker_2: marker::PhantomData::default(),")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"        }")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    }")?;
        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;

        writer.write_all(b"\n\n")?;

        // struct trait impl
        writer.write_all(
            format!(
                "impl<T, V: Visitor<T>> {}<T, V> for {}<T, V> {{",
                base_name, struct_name
            )
            .as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    fn accept(&self, visitor: &V) -> T {")?;
        writer.write_all(b"\n")?;
        writer.write_all(
            format!(
                "\t\tvisitor.visit_{}_{}(self)",
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

        // struct display trait impl
        writer.write_all(
            format!(
                "impl<T, V: Visitor<T>> Display for {}<T, V> {{",
                struct_name
            )
            .as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    fn fmt(&self, f: &mut Formatter<'_>) -> Result {")?;
        writer.write_all(b"\n")?;

        let inner_brace: String = struct_fields
            .iter()
            .map(|_| "{}".to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let field_ref: String = struct_fields
            .iter()
            .map(|(a, _)| format!("self.{}", a))
            .collect::<Vec<_>>()
            .join(", ");

        writer.write_all(
            format!("        write!(f, \"{}\", {})", inner_brace, field_ref,).as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        writer.write_all(b"    }")?;

        writer.write_all(b"\n")?;
        writer.write_all(b"}")?;
        writer.write_all(b"\n\n")?;
        Ok(())
    }
}
