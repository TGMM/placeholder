#[cfg(test)]
mod test {
    lalrpop_mod!(pub assemblyscript); // syntesized by LALRPOP
    use crate::ast::{
        ExprResult, OpType, PrimitiveInfo, QualifierType, SignedPrimitiveInfo, VarDeclInfo,
    };

    #[test]
    fn identifier() {
        let identifier_parser = assemblyscript::IdentifierParser::new();

        assert!(identifier_parser.parse("x").is_ok());
        assert!(identifier_parser.parse("x1y2z3").is_ok());
        assert!(identifier_parser.parse("busin").is_ok());
        assert!(identifier_parser.parse("banana").is_ok());

        assert!(identifier_parser.parse("1x").is_err());
        assert!(identifier_parser.parse("x$xasxads").is_err());
        assert!(identifier_parser.parse("&xxasxads").is_err());
    }

    #[test]
    fn qualifier_type() {
        let qualifier_parser = assemblyscript::QualifierParser::new();

        assert!(qualifier_parser.parse("let").unwrap() == QualifierType::LET);
        assert!(qualifier_parser.parse("const").unwrap() == QualifierType::CONST);
        assert!(qualifier_parser.parse("var").unwrap() == QualifierType::VAR);
        assert!(qualifier_parser.parse("other").is_err());
    }

    macro_rules! primitive_assert {
        ( $parser:ident, $input:expr, $mType:ident ) => {{
            match $parser.parse($input).unwrap() {
                PrimitiveInfo::$mType => (),
                _ => panic!("in the disco"),
            };
        }};
        ( $parser:ident, $input:expr, $mType:ident, $eq:expr  ) => {{
            match $parser.parse($input).unwrap() {
                PrimitiveInfo::$mType(n) => assert!(n == $eq),
                _ => panic!("in the disco"),
            };
        }};
    }

    #[test]
    fn primitive_info() {
        let primitive_parser = assemblyscript::PrimitiveParser::new();

        primitive_assert!(primitive_parser, "69", Number, 69);
        primitive_assert!(
            primitive_parser,
            "\"banana\"",
            String,
            String::from("banana")
        );
        primitive_assert!(primitive_parser, "true", Boolean, true);
        primitive_assert!(primitive_parser, "false", Boolean, false);
        primitive_assert!(primitive_parser, "null", Null);
        primitive_assert!(primitive_parser, "undefined", Undefined);

        primitive_parser.parse("321n").unwrap_err();
        primitive_parser.parse("a@#Q#@!").unwrap_err();
        primitive_parser.parse("@#Q#@!").unwrap_err();
        primitive_parser.parse("truer").unwrap_err();
    }

    #[test]
    fn declare_var() {
        let var_decl_parser = assemblyscript::VarDeclParser::new();

        let x10 = var_decl_parser.parse("let x = 10").unwrap();
        assert_eq!(
            x10,
            VarDeclInfo(
                QualifierType::LET,
                String::from("x"),
                None,
                ExprResult::Primitive(PrimitiveInfo::Number(10))
            )
        );

        let x10w_type = var_decl_parser.parse("let x: number = 10").unwrap();
        assert_eq!(
            x10w_type,
            VarDeclInfo(
                QualifierType::LET,
                String::from("x"),
                Some(String::from("number")),
                ExprResult::Primitive(PrimitiveInfo::Number(10))
            )
        );
    }

    #[test]
    fn expr() {
        let expr_parser = assemblyscript::ExprParser::new();

        let p_false = expr_parser.parse("-false").unwrap();
        assert_eq!(
            p_false,
            ExprResult::SignedPrimitive(SignedPrimitiveInfo(
                OpType::SUB,
                PrimitiveInfo::Boolean(false)
            ))
        );

        let false_p_mtrue = expr_parser.parse("false + -true").unwrap();
        assert_eq!(
            false_p_mtrue,
            ExprResult::ExprResult(
                Box::new(ExprResult::Primitive(PrimitiveInfo::Boolean(false))),
                OpType::ADD,
                Box::new(ExprResult::SignedPrimitive(SignedPrimitiveInfo(
                    OpType::SUB,
                    PrimitiveInfo::Boolean(true)
                )))
            )
        );

        let var_add = expr_parser.parse("x + x").unwrap();
        assert_eq!(
            var_add,
            ExprResult::ExprResult(
                Box::new(ExprResult::Identifier(String::from("x"))),
                OpType::ADD,
                Box::new(ExprResult::Identifier(String::from("x"))),
            )
        );

        expr_parser.parse("(x + x)").unwrap();

        let var_add_2 = expr_parser.parse("x + (x + x)").unwrap();
        assert_eq!(
            var_add_2,
            ExprResult::ExprResult(
                Box::new(ExprResult::Identifier(String::from("x"))),
                OpType::ADD,
                Box::new(ExprResult::ExprResult(Box::new(ExprResult::Identifier(String::from("x"))), OpType::ADD, Box::new(ExprResult::Identifier(String::from("x")))))
            )
        );
    }
}
