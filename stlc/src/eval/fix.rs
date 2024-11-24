use super::{errors::Error, Eval, Value};
use crate::terms::{subst::Subst, syntax::Fix};

impl Eval for Fix {
    fn eval(self) -> Result<Value, Error> {
        let fix_val = self.term.eval()?;
        if let Value::Lambda { var, annot, body } = fix_val {
            body.clone()
                .subst(var.clone(), Value::Lambda { var, annot, body }.into())
                .eval()
        } else {
            Err(Error::BadValue { val: fix_val })
        }
    }
}
