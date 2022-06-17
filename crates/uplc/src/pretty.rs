use pretty::RcDoc;

use crate::ast::{Constant, Name, Program, Term};

impl Program<Name> {
    pub fn to_pretty(&self) -> String {
        let mut w = Vec::new();

        self.to_doc().render(20, &mut w).unwrap();

        String::from_utf8(w).unwrap()
    }

    fn to_doc(&self) -> RcDoc<()> {
        let version = format!("{}.{}.{}", self.version.0, self.version.1, self.version.2);

        RcDoc::text("(")
            .append(RcDoc::text("program"))
            .append(RcDoc::line())
            .append(RcDoc::text(version))
            .append(RcDoc::line())
            .append(self.term.to_doc())
            .append(RcDoc::line_())
            .append(RcDoc::text(")"))
    }
}

impl Term<Name> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Term::Var(name) => RcDoc::text(format!("{}_{}", name.text, name.unique)),
            Term::Delay(term) => RcDoc::text("(")
                .append(RcDoc::line_())
                .append(RcDoc::text("delay"))
                .append(RcDoc::line())
                .append(term.to_doc())
                .append(RcDoc::line_())
                .append(RcDoc::text(")")),
            Term::Lambda {
                parameter_name,
                body,
            } => RcDoc::text("(")
                .append(RcDoc::line_())
                .append(RcDoc::text(format!(
                    "{}_{}",
                    parameter_name.text, parameter_name.unique
                )))
                .append(RcDoc::line())
                .append(body.to_doc())
                .append(RcDoc::line_())
                .append(RcDoc::text(")")),
            Term::Apply { function, argument } => RcDoc::text("[")
                .append(RcDoc::line_())
                .append(function.to_doc())
                .append(RcDoc::line())
                .append(argument.to_doc())
                .append(RcDoc::line_())
                .append(RcDoc::text("]")),
            Term::Constant(constant) => RcDoc::text("(")
                .append(RcDoc::text("con"))
                .append(RcDoc::text(" "))
                .append(constant.to_doc())
                .append(RcDoc::text(")")),
            Term::Force(term) => RcDoc::text("(")
                .append(RcDoc::line_())
                .append(RcDoc::text("force"))
                .append(RcDoc::line())
                .append(term.to_doc())
                .append(RcDoc::line_())
                .append(RcDoc::text(")")),
            Term::Error => RcDoc::text("(")
                .append(RcDoc::line_())
                .append(RcDoc::text("error"))
                .append(RcDoc::line_())
                .append(RcDoc::text(")")),
            Term::Builtin(builtin) => RcDoc::text("(")
                .append(RcDoc::line_())
                .append(RcDoc::text("builtin"))
                .append(RcDoc::line())
                .append(RcDoc::text(builtin.to_string()))
                .append(RcDoc::line_())
                .append(RcDoc::text(")")),
        }
    }
}

impl Constant {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Constant::Integer(i) => RcDoc::text("integer")
                .append(RcDoc::space())
                .append(RcDoc::as_string(i)),
            Constant::ByteString(bs) => RcDoc::text("bytestring")
                .append(RcDoc::space())
                .append(RcDoc::text(hex::encode(bs))),
            Constant::String(s) => RcDoc::text("string")
                .append(RcDoc::space())
                .append(RcDoc::text(s)),
            Constant::Char(c) => unimplemented!("char: {}", c),
            Constant::Unit => RcDoc::text("unit")
                .append(RcDoc::space())
                .append(RcDoc::text("()")),
            Constant::Bool(b) => RcDoc::text("bool")
                .append(RcDoc::space())
                .append(RcDoc::text(if *b { "true" } else { "false" })),
        }
    }
}