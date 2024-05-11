use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use serde::Serialize;
use string_interner::symbol::SymbolU32;

#[derive(Debug, Clone, Serialize)]
pub enum Node {
    Terminal(String),
    RegexString(String),
    Nonterminal(String),
    Multiple(Vec<Node>),
    RegexExt(Box<Node>, RegexExtKind),
    Symbol(Box<Node>, SymbolKind, Box<Node>),
    Group(Box<Node>),
    ANY,
    EXCEPT(Excepted, Option<usize>),
}

#[derive(Debug, Clone)]
pub enum NodeWithID {
    Terminal(SymbolU32),
    RegexString(SymbolU32),
    Nonterminal(SymbolU32),
    Multiple(Vec<NodeWithID>),
    RegexExt(Box<NodeWithID>, RegexExtKind),
    Symbol(Box<NodeWithID>, SymbolKind, Box<NodeWithID>),
    Group(Box<NodeWithID>),
    ANY,
    EXCEPT(ExceptedWithID, Option<usize>),
    Unknown
}

#[derive(Debug, Clone)]
pub(crate) enum NoNestingNode {
    Unknown,
    Terminal(SymbolU32),
    RegexString(SymbolU32),
    Nonterminal(SymbolU32),
    Concatenations(Vec<NoNestingNode>),
    Alternations(Vec<NoNestingNode>),
    ANY,
    EXCEPT(ExceptedWithID, Option<usize>),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) enum OperatorFlattenedNode {
    Terminal(SymbolU32),
    RegexString(SymbolU32),
    Nonterminal(SymbolU32),
    ANY,
    EXCEPT(ExceptedWithID, Option<usize>),
}
#[derive(Debug, Clone)]
pub(crate) struct Rhs {
    pub alternations: Vec<Alternation>,
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Alternation {
    pub concatenations: Vec<OperatorFlattenedNode>,
}
#[derive(Debug, Clone, Serialize)]
pub enum Excepted {
    Terminal(String),
    Nonterminal(String),
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ExceptedWithID {
    Terminal(SymbolU32),
    Nonterminal(SymbolU32),
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Hash)]
pub enum RegexExtKind {
    Repeat0,
    Repeat1,
    Optional,
}

#[derive(Debug, Clone, Serialize)]
pub enum SymbolKind {
    Concatenation,
    Alternation,
}
