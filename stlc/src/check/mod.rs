use super::{types::Type, Var};
use std::collections::HashMap;

pub mod ascribe;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod optional;
pub mod pair;
pub mod record;
pub mod sum;
pub mod term;
pub mod tup;
pub mod unit;
pub mod variant;

pub struct TypingEnv {
    pub used_vars: HashMap<Var, Type>,
}

pub trait Check {
    fn check(&self, env: &mut TypingEnv) -> Option<Type>;
    fn check_local(&self, env: &TypingEnv) -> Option<Type> {
        let mut new_env = TypingEnv {
            used_vars: env.used_vars.clone(),
        };
        self.check(&mut new_env)
    }
}

impl Check for Var {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        env.used_vars.get(self).cloned()
    }
}