use crate::ast::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Char,
    Void,
    Custom(String),
    Unknown,
}

impl Type {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Int" => Type::Int,
            "Float" => Type::Float,
            "Bool" => Type::Bool,
            "String" => Type::String,
            "Char" => Type::Char,
            "Void" => Type::Void,
            other => Type::Custom(other.to_string()),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::String => write!(f, "String"),
            Type::Char => write!(f, "Char"),
            Type::Void => write!(f, "Void"),
            Type::Custom(name) => write!(f, "{}", name),
            Type::Unknown => write!(f, "<unknown>"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub var_type: Type,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Semantic error: {}", self.message)
    }
}

impl std::error::Error for SemanticError {}

pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, Symbol>>,
    functions: HashMap<String, FunctionSignature>,
    current_return_type: Option<Type>,
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            current_return_type: None,
            errors: Vec::new(),
        };

        // Built-in print function accepting any single argument
        analyzer.register_builtin_functions();
        analyzer
    }

    fn register_builtin_functions(&mut self) {
        self.functions.insert(
            "print".to_string(),
            FunctionSignature {
                name: "print".to_string(),
                parameters: vec![("value".to_string(), Type::Unknown)],
                return_type: Type::Void,
            },
        );
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        // First pass: collect function signatures
        for decl in &program.declarations {
            self.collect_declaration(decl);
        }

        // Second pass: analyze function bodies and statements
        for decl in &program.declarations {
            self.analyze_declaration(decl);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn collect_declaration(&mut self, decl: &Declaration) {
        match decl {
            Declaration::Function(func) => {
                let params = func
                    .parameters
                    .iter()
                    .map(|p| (p.name.clone(), Type::from_str(&p.type_name)))
                    .collect();
                let ret = Type::from_str(&func.return_type);

                if self.functions.contains_key(&func.name) && func.name != "print" {
                    self.errors.push(SemanticError {
                        message: format!("Function '{}' is already defined", func.name),
                    });
                } else {
                    self.functions.insert(
                        func.name.clone(),
                        FunctionSignature {
                            name: func.name.clone(),
                            parameters: params,
                            return_type: ret,
                        },
                    );
                }
            }
            Declaration::Module(module) => {
                for sub_decl in &module.declarations {
                    self.collect_declaration(sub_decl);
                }
            }
        }
    }

    fn analyze_declaration(&mut self, decl: &Declaration) {
        match decl {
            Declaration::Function(func) => {
                let ret_type = Type::from_str(&func.return_type);
                self.current_return_type = Some(ret_type);

                self.push_scope();

                // Add parameters to the local scope
                for param in &func.parameters {
                    let p_type = Type::from_str(&param.type_name);
                    self.define_symbol(param.name.clone(), p_type, false);
                }

                self.analyze_block(&func.body);

                self.pop_scope();
                self.current_return_type = None;
            }
            Declaration::Module(module) => {
                for sub_decl in &module.declarations {
                    self.analyze_declaration(sub_decl);
                }
            }
        }
    }

    fn analyze_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.analyze_statement(stmt);
        }
    }

    fn analyze_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDeclaration {
                mutable,
                name,
                type_name,
                initializer,
            } => {
                let init_type = self.analyze_expression(initializer);

                let declared_type = if let Some(t_str) = type_name {
                    let t = Type::from_str(t_str);
                    if init_type != Type::Unknown && init_type != t {
                        self.errors.push(SemanticError {
                            message: format!(
                                "Mismatched types in declaration of '{}': declared '{}', found '{}'",
                                name, t, init_type
                            ),
                        });
                    }
                    t
                } else {
                    init_type
                };

                self.define_symbol(name.clone(), declared_type, *mutable);
            }
            Statement::Assignment { name, value } => {
                let val_type = self.analyze_expression(value);

                if let Some(sym) = self.resolve_symbol(name) {
                    if !sym.mutable {
                        self.errors.push(SemanticError {
                            message: format!(
                                "Cannot assign to immutable variable '{}'. Declare it with 'mut' to allow mutation",
                                name
                            ),
                        });
                    } else if sym.var_type != Type::Unknown
                        && val_type != Type::Unknown
                        && sym.var_type != val_type
                    {
                        self.errors.push(SemanticError {
                            message: format!(
                                "Cannot assign value of type '{}' to variable '{}' of type '{}'",
                                val_type, name, sym.var_type
                            ),
                        });
                    }
                } else {
                    self.errors.push(SemanticError {
                        message: format!("Cannot assign to undeclared variable '{}'", name),
                    });
                }
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_type = self.analyze_expression(condition);
                if cond_type != Type::Unknown && cond_type != Type::Bool {
                    self.errors.push(SemanticError {
                        message: format!("'if' condition must be 'Bool', found '{}'", cond_type),
                    });
                }

                self.push_scope();
                self.analyze_block(then_branch);
                self.pop_scope();

                if let Some(else_stmt) = else_branch {
                    self.analyze_statement(else_stmt);
                }
            }
            Statement::While { condition, body } => {
                let cond_type = self.analyze_expression(condition);
                if cond_type != Type::Unknown && cond_type != Type::Bool {
                    self.errors.push(SemanticError {
                        message: format!("'while' condition must be 'Bool', found '{}'", cond_type),
                    });
                }

                self.push_scope();
                self.analyze_block(body);
                self.pop_scope();
            }
            Statement::For {
                variable,
                iterable,
                body,
            } => {
                let iter_type = self.analyze_expression(iterable);
                if iter_type != Type::Unknown && iter_type != Type::Int {
                    self.errors.push(SemanticError {
                        message: format!("'for' loop iterable must be a range of 'Int', found '{}'", iter_type),
                    });
                }

                self.push_scope();
                // Range elements are Int
                self.define_symbol(variable.clone(), Type::Int, false);
                self.analyze_block(body);
                self.pop_scope();
            }
            Statement::Return(opt_expr) => {
                let actual_type = match opt_expr {
                    Some(expr) => self.analyze_expression(expr),
                    None => Type::Void,
                };

                if let Some(expected) = &self.current_return_type {
                    if *expected != Type::Void && actual_type == Type::Void {
                        self.errors.push(SemanticError {
                            message: format!("Function expected return type '{}', but returned 'Void'", expected),
                        });
                    } else if actual_type != Type::Unknown
                        && *expected != actual_type
                        && (*expected != Type::Void || actual_type != Type::Void)
                    {
                        self.errors.push(SemanticError {
                            message: format!(
                                "Mismatched return type: expected '{}', found '{}'",
                                expected, actual_type
                            ),
                        });
                    }
                }
            }
            Statement::Expression(expr) => {
                self.analyze_expression(expr);
            }
            Statement::Block(inner_block) => {
                self.push_scope();
                self.analyze_block(inner_block);
                self.pop_scope();
            }
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Integer(_) => Type::Int,
            Expression::Float(_) => Type::Float,
            Expression::String(_) => Type::String,
            Expression::Char(_) => Type::Char,
            Expression::Bool(_) => Type::Bool,
            Expression::Identifier(name) => {
                if let Some(sym) = self.resolve_symbol(name) {
                    sym.var_type.clone()
                } else {
                    self.errors.push(SemanticError {
                        message: format!("Variable '{}' is used before being declared", name),
                    });
                    Type::Unknown
                }
            }
            Expression::Unary { operator, right } => {
                let r_type = self.analyze_expression(right);
                match operator.as_str() {
                    "!" => {
                        if r_type != Type::Unknown && r_type != Type::Bool {
                            self.errors.push(SemanticError {
                                message: format!("Operator '!' cannot be applied to type '{}'", r_type),
                            });
                        }
                        Type::Bool
                    }
                    "-" => {
                        if r_type != Type::Unknown && r_type != Type::Int && r_type != Type::Float {
                            self.errors.push(SemanticError {
                                message: format!("Unary '-' cannot be applied to type '{}'", r_type),
                            });
                        }
                        r_type
                    }
                    _ => Type::Unknown,
                }
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let l_type = self.analyze_expression(left);
                let r_type = self.analyze_expression(right);

                match operator.as_str() {
                    "+" | "-" | "*" | "/" | "%" => {
                        if l_type != Type::Unknown && r_type != Type::Unknown && l_type != r_type {
                            self.errors.push(SemanticError {
                                message: format!(
                                    "Binary '{}' cannot be applied to mismatched types '{}' and '{}'",
                                    operator, l_type, r_type
                                ),
                            });
                        }
                        l_type
                    }
                    "==" | "!=" => {
                        if l_type != Type::Unknown && r_type != Type::Unknown && l_type != r_type {
                            self.errors.push(SemanticError {
                                message: format!(
                                    "Comparison '{}' between incompatible types '{}' and '{}'",
                                    operator, l_type, r_type
                                ),
                            });
                        }
                        Type::Bool
                    }
                    "<" | "<=" | ">" | ">=" => {
                        if l_type != Type::Unknown && r_type != Type::Unknown && l_type != r_type {
                            self.errors.push(SemanticError {
                                message: format!(
                                    "Ordered comparison '{}' between incompatible types '{}' and '{}'",
                                    operator, l_type, r_type
                                ),
                            });
                        }
                        Type::Bool
                    }
                    "&&" | "||" => {
                        if (l_type != Type::Unknown && l_type != Type::Bool)
                            || (r_type != Type::Unknown && r_type != Type::Bool)
                        {
                            self.errors.push(SemanticError {
                                message: format!("Logical '{}' requires 'Bool' operands", operator),
                            });
                        }
                        Type::Bool
                    }
                    _ => Type::Unknown,
                }
            }
            Expression::Range {
                start,
                inclusive: _,
                end,
            } => {
                let s_type = self.analyze_expression(start);
                let e_type = self.analyze_expression(end);

                if s_type != Type::Unknown && s_type != Type::Int {
                    self.errors.push(SemanticError {
                        message: format!("Range start must be 'Int', found '{}'", s_type),
                    });
                }
                if e_type != Type::Unknown && e_type != Type::Int {
                    self.errors.push(SemanticError {
                        message: format!("Range end must be 'Int', found '{}'", e_type),
                    });
                }
                Type::Int
            }
            Expression::Call { callee, arguments } => {
                if let Expression::Identifier(func_name) = callee.as_ref() {
                    if func_name == "print" {
                        for arg in arguments {
                            self.analyze_expression(arg);
                        }
                        return Type::Void;
                    }

                    if let Some(sig) = self.functions.get(func_name).cloned() {
                        if arguments.len() != sig.parameters.len() {
                            self.errors.push(SemanticError {
                                message: format!(
                                    "Function '{}' expects {} arguments, but {} were provided",
                                    func_name,
                                    sig.parameters.len(),
                                    arguments.len()
                                ),
                            });
                        } else {
                            for (idx, arg) in arguments.iter().enumerate() {
                                let arg_type = self.analyze_expression(arg);
                                let expected = &sig.parameters[idx].1;
                                if *expected != Type::Unknown
                                    && arg_type != Type::Unknown
                                    && *expected != arg_type
                                {
                                    self.errors.push(SemanticError {
                                        message: format!(
                                            "Argument {} of function '{}' expects '{}', found '{}'",
                                            idx + 1,
                                            func_name,
                                            expected,
                                            arg_type
                                        ),
                                    });
                                }
                            }
                        }
                        sig.return_type
                    } else {
                        self.errors.push(SemanticError {
                            message: format!("Call to undefined function '{}'", func_name),
                        });
                        Type::Unknown
                    }
                } else {
                    Type::Unknown
                }
            }
            Expression::Grouping(inner) => self.analyze_expression(inner),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    fn define_symbol(&mut self, name: String, var_type: Type, mutable: bool) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(
                name.clone(),
                Symbol {
                    name,
                    var_type,
                    mutable,
                },
            );
        }
    }

    fn resolve_symbol(&self, name: &str) -> Option<Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym.clone());
            }
        }
        None
    }
}

pub fn check_semantics(program: &Program) -> Result<(), Vec<SemanticError>> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parse;

    fn parse_src(src: &str) -> Program {
        let tokens = lex(src).expect("Lexing failed");
        parse(&tokens).expect("Parsing failed")
    }

    #[test]
    fn test_valid_types_and_functions() {
        let src = r#"
            fn add(a: Int, b: Int) -> Int {
                return a + b;
            }

            fn main() -> Void {
                let x: Int = 10;
                let y: Int = 20;
                let res: Int = add(x, y);
                print(res);
            }
        "#;
        let prog = parse_src(src);
        assert!(check_semantics(&prog).is_ok());
    }

    #[test]
    fn test_immutable_variable_reassignment() {
        let src = r#"
            fn main() -> Void {
                let x: Int = 10;
                x = 20;
            }
        "#;
        let prog = parse_src(src);
        let res = check_semantics(&prog);
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert!(errs[0].message.contains("Cannot assign to immutable variable"));
    }

    #[test]
    fn test_type_mismatch_in_assignment() {
        let src = r#"
            fn main() -> Void {
                mut x: Int = 10;
                x = "hello";
            }
        "#;
        let prog = parse_src(src);
        let res = check_semantics(&prog);
        assert!(res.is_err());
        assert!(res.unwrap_err()[0].message.contains("Cannot assign value of type 'String'"));
    }

    #[test]
    fn test_undefined_variable_detection() {
        let src = r#"
            fn main() -> Void {
                print(unknown_var);
            }
        "#;
        let prog = parse_src(src);
        let res = check_semantics(&prog);
        assert!(res.is_err());
        assert!(res.unwrap_err()[0].message.contains("used before being declared"));
    }

    #[test]
    fn test_invalid_condition_type() {
        let src = r#"
            fn main() -> Void {
                if 123 {
                    print("bad");
                }
            }
        "#;
        let prog = parse_src(src);
        let res = check_semantics(&prog);
        assert!(res.is_err());
        assert!(res.unwrap_err()[0].message.contains("'if' condition must be 'Bool'"));
    }
}
