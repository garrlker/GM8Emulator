use super::{
    ast,
    runtime::{Instruction, Node},
    Value,
};
use std::collections::HashMap;

pub struct Compiler {
    /// List of identifiers which represent const values
    constants: HashMap<String, Value>,

    /// Table of script names to IDs
    script_names: HashMap<String, usize>,

    /// Lookup table of unique field names
    fields: Vec<String>,
}

pub enum Error {
    ASTError(String),
    GMLError(String),
}

impl Compiler {
    /// Create a compiler.
    pub fn new() -> Self {
        Self {
            constants: HashMap::new(),
            script_names: HashMap::new(),
            fields: Vec::new(),
        }
    }

    /// Reserve space to register at least the given number of constants.
    pub fn reserve_constants(&mut self, size: usize) {
        self.constants.reserve(size)
    }

    /// Reserve space to register at least the given number of script names.
    pub fn reserve_scripts(&mut self, size: usize) {
        self.script_names.reserve(size)
    }

    /// Add a constant and its associated f64 value, such as an asset name.
    /// These constants will override built-in ones, such as c_red. However, if the same constant name is
    /// registered twice, the old one will NOT be overwritten and the value will be dropped, as per GM8.
    pub fn register_constant(&mut self, name: String, value: f64) {
        self.constants.entry(name).or_insert(Value::Real(value));
    }

    /// Register a script name and its index.
    /// Panics if two identical script names are registered - GM8 does not allow this.
    pub fn register_script(&mut self, name: String, index: usize) {
        if let Some(v) = self.script_names.insert(name, index) {
            panic!("Two scripts with the same name registered: at index {} and {}", v, index);
        }
    }

    /// Compile a GML string into instructions.
    pub fn compile(&mut self, source: &str) -> Result<Vec<Instruction>, Error> {
        let ast = ast::AST::new(source).map_err(|e| Error::ASTError(e.message))?;

        let instructions = Vec::new();
        for _node in ast.into_iter() {
            // TODO: this
        }
        Ok(instructions)
    }

    /// Compile an expression into a format which can be evaluated.
    pub fn compile_expression(&mut self, source: &str) -> Result<Node, Error> {
        let expr = ast::AST::expression(source).map_err(|e| Error::ASTError(e.message))?;
        self.compile_ast_expr(expr)
    }

    fn compile_ast_expr(&mut self, _expr: ast::Expr) -> Result<Node, Error> {
        unimplemented!()
    }
}