use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::DataTypes;
use crate::core::operator_types::{OperatorTypes, SpecialOperatorTypes};

#[derive(Clone)]
pub enum Nodes {
    Keyword(String),
    Value(Rc<RefCell<DataTypes>>),
    Operator(OperatorTypes),
    VariableDef {
        name: String,
        value: Box<Nodes>
    },
    SpecialAssignment {
        op: SpecialOperatorTypes,
        keyword: String,
        value: Box<Nodes>
    },
    FnDef {
        name: String,
        params: Vec<String>,
        body: Box<Nodes>
    },
    FnCall {
        name: Option<String>,
        func: Option<Box<Nodes>>,
        args: Vec<Box<Nodes>>
    },
    BinaryExpr {
        op: OperatorTypes,
        left: Box<Nodes>,
        right: Box<Nodes>
    },
    Punc(String),
    Scope(Vec<Box<Nodes>>)
}

impl Nodes {
    pub fn is_op(&self) -> bool {
        match self {
            Nodes::Operator(_) => true,
            _ => false
        }
    }

    pub fn is_kw(&self) -> bool {
        match self {
            Nodes::Keyword(_) => true,
            _ => false
        }
    }

    pub fn is_dyn_fn(&self) -> bool {
        match self {
            Nodes::Value(v) => {
                (*v.borrow()).is_dyn_fn()
            }
            _ => false
        }
    }

    pub fn is_fn(&self) -> bool {
        match self {
            Nodes::FnDef { .. } => true,
            _ => false
        }
    }
}

impl Nodes {
    pub fn to_op(&self) -> &OperatorTypes {
        match self {
            Nodes::Operator(op) => op,
            _ => panic!("Node not an operator.")
        }
    }

    pub fn to_special_assignment(&self) -> (&SpecialOperatorTypes, &String, &Box<Nodes>) {
        match self {
            Nodes::SpecialAssignment { keyword, op, value } => (op, keyword, value),
            _ => panic!("Node is not sp assign")
        }
    }

    pub fn to_bin(&self) -> (&OperatorTypes, &Box<Nodes>, &Box<Nodes>) {
        match self {
            Nodes::BinaryExpr { op, left, right } => (op, left, right),
            _ => panic!("Node not binary.")
        }
    }

    pub fn to_variable(&self) -> (&String, &Box<Nodes>) {
        match self {
            Nodes::VariableDef { name, value } => (name, value),
            _ => panic!("Node is not variable.")
        }
    }

    pub fn to_fn_def(&self) -> (&String, &Vec<String>, &Box<Nodes>) {
        match self {
            Nodes::FnDef { name, params, body } => (name, params, body),
            _ => panic!("Node not fn def.")
        }
    }

    pub fn to_fn_c(&self) -> (&Option<String>, &Vec<Box<Nodes>>, &Option<Box<Nodes>>) {
        match self {
            Nodes::FnCall { name, args, func } => (name, args, func),
            _ => panic!("Node not func call.")
        }
    }

    pub fn to_kw(&self) -> &String {
        match self {
            Nodes::Keyword(s) => s,
            _ => panic!("Node is not keyword.")
        }
    }
}