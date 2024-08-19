use recursion::{Collapsible, MappableFrame, PartiallyApplied};

use super::types::{
    BinaryExpression, BinaryOperator, BinaryPart, Digest, Expr, FnArgType, Identifier, KclNone, Literal, Parameter,
    PipeSubstitution, TagDeclarator,
};

pub enum ExprFrame<A> {
    Literal(Box<Literal>),
    Identifier(Box<Identifier>),
    TagDeclarator(Box<TagDeclarator>),
    BinaryExpression(Box<BinaryExpressionFrame<A>>),
    FunctionExpression(Box<FunctionExpressionFrame<A>>),
    CallExpression(Box<CallExpressionFrame<A>>),
    PipeExpression(Box<PipeExpressionFrame<A>>),
    PipeSubstitution(Box<PipeSubstitution>),
    ArrayExpression(Box<ArrayExpressionFrame<A>>),
    ObjectExpression(Box<ObjectExpressionFrame<A>>),
    MemberExpression(Box<MemberExpressionFrame<A>>),
    UnaryExpression(Box<UnaryExpressionFrame<A>>),
    None(KclNone),
}

impl MappableFrame for ExprFrame<PartiallyApplied> {
    type Frame<X> = ExprFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            ExprFrame::Literal(x) => ExprFrame::Literal(x),
            ExprFrame::Identifier(x) => ExprFrame::Identifier(x),
            ExprFrame::TagDeclarator(x) => ExprFrame::TagDeclarator(x),
            ExprFrame::BinaryExpression(x) => ExprFrame::BinaryExpression(MappableFrame::map_frame(x, &mut f)),
            ExprFrame::FunctionExpression(x) => ExprFrame::FunctionExpression(MappableFrame::map_frame(x, &mut f)),
            ExprFrame::CallExpression(x) => ExprFrame::CallExpression(MappableFrame::map_frame(x, &mut f)),
            ExprFrame::PipeExpression(x) => ExprFrame::PipeExpression(MappableFrame::map_frame(x, &mut f)),
            ExprFrame::PipeSubstitution(x) => ExprFrame::PipeSubstitution(x),
            ExprFrame::ArrayExpression(x) => ExprFrame::ArrayExpression(MappableFrame::map_frame(x, &mut f)),
            ExprFrame::ObjectExpression(x) => ExprFrame::ObjectExpression(MappableFrame::map_frame(x, &mut f)),
            ExprFrame::MemberExpression(x) => ExprFrame::MemberExpression(MappableFrame::map_frame(x, &mut f)),
            ExprFrame::UnaryExpression(x) => ExprFrame::UnaryExpression(MappableFrame::map_frame(x, &mut f)),
            ExprFrame::None(x) => ExprFrame::None(x),
        }
    }
}

impl<'a> Collapsible for &'a Expr {
    type FrameToken = ExprFrame<PartiallyApplied>;

    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        match self {
            Expr::Literal(x) => ExprFrame::Literal(x.clone()),
            Expr::Identifier(x) => ExprFrame::Identifier(x.clone()),
            Expr::TagDeclarator(x) => ExprFrame::TagDeclarator(x.clone()),
            Expr::BinaryExpression(x) => ExprFrame::BinaryExpression(Box::new(x.into_frame())),
            Expr::FunctionExpression(x) => ExprFrame::FunctionExpression(Box::new(x.into_frame())),
            Expr::CallExpression(x) => ExprFrame::CallExpression(Box::new(x.into_frame())),
            Expr::PipeExpression(x) => ExprFrame::PipeExpression(Box::new(x.into_frame())),
            Expr::PipeSubstitution(x) => ExprFrame::PipeSubstitution(x.clone()),
            Expr::ArrayExpression(x) => ExprFrame::ArrayExpression(Box::new(x.into_frame())),
            Expr::ObjectExpression(x) => ExprFrame::ObjectExpression(Box::new(x.into_frame())),
            Expr::MemberExpression(x) => ExprFrame::MemberExpression(Box::new(x.into_frame())),
            Expr::UnaryExpression(x) => ExprFrame::UnaryExpression(Box::new(x.into_frame())),
            Expr::None(x) => ExprFrame::None(x.clone()),
        }
    }
}

pub struct BinaryExpressionFrame<A> {
    pub start: usize,
    pub end: usize,
    pub operator: BinaryOperator,
    pub left: BinaryPartFrame<A>,
    pub right: BinaryPartFrame<A>,

    pub digest: Option<Digest>,
}

impl MappableFrame for BinaryExpressionFrame<PartiallyApplied> {
    type Frame<X> = BinaryExpressionFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        BinaryExpressionFrame::<B> {
            start: input.start,
            end: input.end,
            operator: input.operator,
            left: <BinaryPartFrame<PartiallyApplied> as MappableFrame>::map_frame(input.left, &mut f),
            right: <BinaryPartFrame<PartiallyApplied> as MappableFrame>::map_frame(input.right, &mut f),
            digest: input.digest,
        }
    }
}

impl<'a> Collapsible for &'a BinaryExpression {
    type FrameToken = BinaryExpressionFrame<PartiallyApplied>;

    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        BinaryExpressionFrame::<BinaryExpression> {
            start: self.start,
            end: self.end,
            operator: self.operator.clone(),
            left: self.left.into_frame(),
            right: self.right.into_frame(),
            digest: self.digest,
        }
    }
}

pub enum BinaryPartFrame<A> {
    Literal(Box<Literal>),
    Identifier(Box<Identifier>),
    BinaryExpression(Box<BinaryExpressionFrame<A>>),
    CallExpression(Box<CallExpressionFrame<A>>),
    UnaryExpression(Box<UnaryExpressionFrame<A>>),
    MemberExpression(Box<MemberExpressionFrame<A>>),
}

impl MappableFrame for BinaryPartFrame<PartiallyApplied> {
    type Frame<X> = BinaryPartFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            BinaryPartFrame::Literal(x) => BinaryPartFrame::Literal(x),
            BinaryPartFrame::Identifier(x) => BinaryPartFrame::Identifier(x),
            BinaryPartFrame::BinaryExpression(x) => BinaryPartFrame::BinaryExpression(MappableFrame::map_frame(x, f)),
            BinaryPartFrame::CallExpression(x) => BinaryPartFrame::CallExpression(MappableFrame::map_frame(x, f)),
            BinaryPartFrame::UnaryExpression(x) => BinaryPartFrame::UnaryExpression(MappableFrame::map_frame(x, f)),
            BinaryPartFrame::MemberExpression(x) => BinaryPartFrame::MemberExpression(MappableFrame::map_frame(x, f)),
        }
    }
}

impl<'a> Collapsible for &'a BinaryPart {
    type FrameToken = BinaryPartFrame<PartiallyApplied>;

    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        match self {
            BinaryPart::Literal(x) => BinaryPartFrame::Literal(x.clone()),
            BinaryPart::Identifier(x) => BinaryPartFrame::Identifier(x.clone()),
            BinaryPart::BinaryExpression(x) => BinaryPartFrame::BinaryExpression(x.into_frame()),
            BinaryPart::CallExpression(x) => BinaryPartFrame::CallExpression(x.into_frame()),
            BinaryPart::UnaryExpression(x) => BinaryPartFrame::UnaryExpression(x.into_frame()),
            BinaryPart::MemberExpression(x) => BinaryPartFrame::MemberExpression(x.into_frame()),
        }
    }
}

pub struct FunctionExpressionFrame<A> {
    pub start: usize,
    pub end: usize,
    pub params: Vec<Parameter>,
    pub body: ProgramFrame<A>,
    pub return_type: Option<FnArgType>,

    pub digest: Option<Digest>,
}

pub struct CallExpressionFrame<A> {
    pub start: usize,
    pub end: usize,
    pub callee: Identifier,
    pub arguments: Vec<ExprFrame<A>>,
    pub optional: bool,

    pub digest: Option<Digest>,
}

// More...
