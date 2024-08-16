//! How to traverse a KCL AST, visiting each node along the way.
use crate::ast::types::{
    ArrayExpression, BinaryExpression, BodyItem, CallExpression, Expr, FunctionExpression, Identifier, KclNone,
    Literal, MemberExpression, ObjectExpression, PipeExpression, PipeSubstitution, Program, ReturnStatement,
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
    fn compose(self, other: Self) -> Self;
}

/// Traverse the KCL AST, visiting each node with the given visitor.
pub fn traverse<V>(mut visitor: V, program: Program) -> Result<V::Output, V::Error>
where
    V: Visitor,
{
    let mut state = V::Output::default();
    for statement in &program.body {
        match statement {
            BodyItem::ExpressionStatement(stmt) => match &stmt.expression {
                Expr::Literal(expr) => state = visitor.visit_literal_expr(expr)?,
                Expr::Identifier(expr) => state = visitor.visit_identifier_expr(expr)?,
                Expr::TagDeclarator(expr) => state = visitor.visit_tag_decl_expr(expr)?,
                Expr::BinaryExpression(expr) => state = visitor.visit_binary_expr(expr)?,
                Expr::FunctionExpression(expr) => state = visitor.visit_function_expr(expr)?,
                Expr::CallExpression(expr) => state = visitor.visit_call_expr(expr)?,
                Expr::PipeExpression(expr) => state = visitor.visit_pipe_expr(expr)?,
                Expr::PipeSubstitution(expr) => state = visitor.visit_pipe_sub_expr(expr)?,
                Expr::ArrayExpression(expr) => state = visitor.visit_array_expr(expr)?,
                Expr::ObjectExpression(expr) => state = visitor.visit_object_expr(expr)?,
                Expr::MemberExpression(expr) => state = visitor.visit_member_expr(expr)?,
                Expr::UnaryExpression(expr) => state = visitor.visit_unary_expr(expr)?,
                Expr::None(expr) => state = visitor.visit_none_expr(expr)?,
            },
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
