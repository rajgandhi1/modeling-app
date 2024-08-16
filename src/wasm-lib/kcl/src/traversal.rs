//! How to traverse a KCL AST, visiting each node along the way.
use std::convert::Infallible;

use crate::ast::types::{
    ArrayExpression, BinaryExpression, BinaryPart, BodyItem, CallExpression, Expr, FunctionExpression, Identifier,
    KclNone, Literal, MemberExpression, ObjectExpression, PipeExpression, PipeSubstitution, Program, ReturnStatement,
    TagDeclarator, UnaryExpression, VariableDeclarator,
};

/// Result of visiting.
type R<V> = Result<<V as Visitor>::Output, <V as Visitor>::Error>;

pub trait Visitor {
    type Output: Compose;
    type Error;
    /// Visit a variable declaration statement.
    fn visit_decl_stmt(&mut self, stmt: &VariableDeclarator) -> R<Self>;
    /// Visit a return statement.
    fn visit_rtrn_stmt(&mut self, stmt: &ReturnStatement) -> R<Self>;
    // Visit various expressions.
    fn visit_literal_expr(&mut self, expr: &Literal) -> R<Self>;
    fn visit_identifier_expr(&mut self, expr: &Identifier) -> R<Self>;
    fn visit_tag_decl_expr(&mut self, expr: &TagDeclarator) -> R<Self>;
    fn visit_binary_expr(&mut self, expr: &BinaryExpression) -> R<Self>;
    fn visit_function_expr(&mut self, expr: &FunctionExpression) -> R<Self>;
    fn visit_call_expr(&mut self, expr: &CallExpression) -> R<Self>;
    fn visit_pipe_expr(&mut self, expr: &PipeExpression) -> R<Self>;
    fn visit_pipe_sub_expr(&mut self, expr: &PipeSubstitution) -> R<Self>;
    fn visit_array_expr(&mut self, expr: &ArrayExpression) -> R<Self>;
    fn visit_object_expr(&mut self, expr: &ObjectExpression) -> R<Self>;
    fn visit_member_expr(&mut self, expr: &MemberExpression) -> R<Self>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpression) -> R<Self>;
    fn visit_none_expr(&mut self, expr: &KclNone) -> R<Self>;
}

pub trait Compose: Default {
    fn compose(&mut self, other: Self);
}

/// Traverse the KCL AST, visiting each node with the given visitor.
pub fn traverse<V>(mut visitor: V, program: Program) -> Result<V::Output, V::Error>
where
    V: Visitor,
{
    let mut state = V::Output::default();
    for statement in &program.body {
        match statement {
            BodyItem::ExpressionStatement(stmt) => {
                state = accept_expr(&mut visitor, &stmt.expression)?;
            }
            BodyItem::VariableDeclaration(stmt) => {
                for decl in &stmt.declarations {
                    state = visitor.visit_decl_stmt(decl)?;
                }
            }
            BodyItem::ReturnStatement(stmt) => {
                state = visitor.visit_rtrn_stmt(stmt)?;
            }
        }
    }
    Ok(state)
}

struct Printer;

impl Compose for Vec<String> {
    fn compose(&mut self, other: Self) {
        self.extend(other);
    }
}

impl Visitor for Printer {
    type Output = Vec<String>;
    type Error = Infallible;

    fn visit_decl_stmt(&mut self, stmt: &VariableDeclarator) -> R<Self> {
        let mut out = Self::Output::default();
        let name = &stmt.id.name;
        out.push(format!("Declaring {name}"));
        out.compose(accept_expr(self, &stmt.init)?);
        out.push(format!("Declared {name}"));
        Ok(out)
    }

    fn visit_rtrn_stmt(&mut self, stmt: &ReturnStatement) -> R<Self> {
        let mut out = Self::Output::default();
        out.push("Calculating a return value".to_owned());
        out.compose(accept_expr(self, &stmt.argument)?);
        out.push("Calculated the return value".to_owned());
        Ok(out)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!("Evaluated a literal {:?}", expr.value));
        Ok(out)
    }

    fn visit_identifier_expr(&mut self, expr: &Identifier) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!("Evaluating the variable {}", expr.name));
        Ok(out)
    }

    fn visit_tag_decl_expr(&mut self, expr: &TagDeclarator) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!("Declaring tag {}", expr.name));
        Ok(out)
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpression) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!(
            "Evaluating a binary expression, operation is {:?}",
            expr.operator
        ));
        out.push("Evaluating L:".to_string());
        out.compose(accept_binary_part(self, &expr.left)?);
        out.push("Evaluating R:".to_string());
        out.compose(accept_binary_part(self, &expr.right)?);
        out.push("Evaluated a binary expression.".to_string());
        Ok(out)
    }

    fn visit_function_expr(&mut self, expr: &FunctionExpression) -> R<Self> {
        let mut out = Self::Output::default();
        let nargs = expr.number_of_args();
        let min_args = nargs.start();
        let max_args = nargs.end();
        let msg = if min_args == max_args {
            format!("Defining a function with {min_args} args")
        } else {
            format!("Defining a function with {min_args} to {max_args} args")
        };
        out.push(msg);
        Ok(out)
    }

    fn visit_call_expr(&mut self, expr: &CallExpression) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!("Preparing to call {}", expr.callee.name));
        for (i, arg) in expr.arguments.iter().enumerate() {
            out.push(format!("Evaluating argument {i}"));
            out.compose(accept_expr(self, arg)?);
        }
        out.push(format!("Calling {}", expr.callee.name));
        out.push(format!("Called {}", expr.callee.name));
        Ok(out)
    }

    fn visit_pipe_expr(&mut self, expr: &PipeExpression) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!("Starting pipe expression with {} children", expr.body.len()));
        for child in &expr.body {
            out.compose(accept_expr(self, child)?);
        }
        out.push("Finished pipe expression".to_string());
        Ok(out)
    }

    fn visit_pipe_sub_expr(&mut self, _: &PipeSubstitution) -> R<Self> {
        Ok(vec!["%".to_owned()])
    }

    fn visit_array_expr(&mut self, expr: &ArrayExpression) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!(
            "Starting array expression with {} children",
            expr.elements.len()
        ));
        for (i, child) in expr.elements.iter().enumerate() {
            out.push(format!("Calculating element {i}"));
            out.compose(accept_expr(self, child)?);
        }
        out.push("Finished array expression".to_string());
        Ok(out)
    }

    fn visit_object_expr(&mut self, expr: &ObjectExpression) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!(
            "Starting object expression with {} properties",
            expr.properties.len()
        ));
        for child in &expr.properties {
            out.push(format!("Calculating property {}", child.key.name));
            out.compose(accept_expr(self, &child.value)?);
        }
        out.push("Finished array expression".to_string());
        Ok(out)
    }

    fn visit_member_expr(&mut self, expr: &MemberExpression) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!("Evaluating property {:?}", expr.property));
        match &expr.object {
            crate::ast::types::MemberObject::MemberExpression(mem) => {
                out.compose(self.visit_member_expr(mem)?);
            }
            crate::ast::types::MemberObject::Identifier(id) => out.push(format!("Object is variable {}", id.name)),
        }
        out.push("Finished evaluating member expression".to_string());
        Ok(out)
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpression) -> R<Self> {
        let mut out = Self::Output::default();
        out.push(format!(
            "Applying operation '{}'",
            match expr.operator {
                crate::ast::types::UnaryOperator::Neg => "- (neg)",
                crate::ast::types::UnaryOperator::Not => "! (not)",
            }
        ));
        out.compose(accept_binary_part(self, &expr.argument)?);
        out.push("Applied".to_string());
        Ok(out)
    }

    fn visit_none_expr(&mut self, _: &KclNone) -> R<Self> {
        Ok(vec!["KCL none".to_owned()])
    }
}

fn accept_expr<V>(visitor: &mut V, expr: &Expr) -> R<V>
where
    V: Visitor,
{
    match &expr {
        Expr::Literal(expr) => visitor.visit_literal_expr(expr),
        Expr::Identifier(expr) => visitor.visit_identifier_expr(expr),
        Expr::TagDeclarator(expr) => visitor.visit_tag_decl_expr(expr),
        Expr::BinaryExpression(expr) => visitor.visit_binary_expr(expr),
        Expr::FunctionExpression(expr) => visitor.visit_function_expr(expr),
        Expr::CallExpression(expr) => visitor.visit_call_expr(expr),
        Expr::PipeExpression(expr) => visitor.visit_pipe_expr(expr),
        Expr::PipeSubstitution(expr) => visitor.visit_pipe_sub_expr(expr),
        Expr::ArrayExpression(expr) => visitor.visit_array_expr(expr),
        Expr::ObjectExpression(expr) => visitor.visit_object_expr(expr),
        Expr::MemberExpression(expr) => visitor.visit_member_expr(expr),
        Expr::UnaryExpression(expr) => visitor.visit_unary_expr(expr),
        Expr::None(expr) => visitor.visit_none_expr(expr),
    }
}

fn accept_binary_part<V>(visitor: &mut V, expr: &BinaryPart) -> R<V>
where
    V: Visitor,
{
    match &expr {
        BinaryPart::Literal(expr) => visitor.visit_literal_expr(expr),
        BinaryPart::Identifier(expr) => visitor.visit_identifier_expr(expr),
        BinaryPart::BinaryExpression(expr) => visitor.visit_binary_expr(expr),
        BinaryPart::CallExpression(expr) => visitor.visit_call_expr(expr),
        BinaryPart::MemberExpression(expr) => visitor.visit_member_expr(expr),
        BinaryPart::UnaryExpression(expr) => visitor.visit_unary_expr(expr),
    }
}
