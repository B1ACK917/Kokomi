use std::env::var;
use crate::debug_fn_inline;
use colored::Colorize;

pub trait ParseVar {
    type Variable;
    fn parse_var(key: &str) -> Self::Variable;
    fn parse_var_or(key: &str, value: Self::Variable) -> Self::Variable;
}

impl ParseVar for bool {
    type Variable = bool;

    fn parse_var(key: &str) -> Self::Variable {
        debug_fn_inline!(key);
        match var(key) {
            Ok(_) => { true }
            Err(_) => { false }
        }
    }
    fn parse_var_or(key: &str, value: Self::Variable) -> Self::Variable {
        debug_fn_inline!(key);
        match var(key) {
            Ok(_) => { true }
            Err(_) => { value }
        }
    }
}

impl ParseVar for usize {
    type Variable = usize;

    fn parse_var(key: &str) -> Self::Variable {
        debug_fn_inline!(key);
        match var(key) {
            Ok(value) => { value.parse().unwrap() }
            Err(_) => { 0 }
        }
    }

    fn parse_var_or(key: &str, value: Self::Variable) -> Self::Variable {
        debug_fn_inline!(key);
        match var(key) {
            Ok(value) => { value.parse().unwrap() }
            Err(_) => { value }
        }
    }
}

impl ParseVar for f32 {
    type Variable = f32;

    fn parse_var(key: &str) -> Self::Variable {
        debug_fn_inline!(key);
        match var(key) {
            Ok(value) => { value.parse().unwrap() }
            Err(_) => { 0f32 }
        }
    }

    fn parse_var_or(key: &str, value: Self::Variable) -> Self::Variable {
        debug_fn_inline!(key);
        match var(key) {
            Ok(value) => { value.parse().unwrap() }
            Err(_) => { value }
        }
    }
}

impl ParseVar for String {
    type Variable = String;

    fn parse_var(key: &str) -> Self::Variable {
        debug_fn_inline!(key);
        match var(key) {
            Ok(value) => { value }
            Err(_) => { String::new() }
        }
    }

    fn parse_var_or(key: &str, value: Self::Variable) -> Self::Variable {
        debug_fn_inline!(key);
        match var(key) {
            Ok(value) => { value }
            Err(_) => { value }
        }
    }
}