#[macro_export]
macro_rules! prod {
    ($p: ident, $lhs: ident -> #Epsilon, $compute: expr) => {{
        #[allow(non_snake_case, unused_variables)]
        {
            let $lhs = $p.get_or_create_non_terminal(stringify!($lhs).to_string());
            $p.production(*$lhs.as_non_terminal_id().unwrap(), vec![], $compute)
        }
    }};

    ($p: ident, $lhs: ident -> $($rhs: ident)+, $compute: expr) => {{
        #[allow(non_snake_case, unused_variables)]
        {
            let $lhs = $p.get_or_create_non_terminal(stringify!($lhs).to_string());
            let mut vec = Vec::new();
            $(
                let $rhs = $p.get_or_create_non_terminal(stringify!($rhs).to_string());
                vec.push($rhs);
            )+
            $p.production(*$lhs.as_non_terminal_id().unwrap(), vec, $compute)
        }
    }};
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! productions {
    ($p: ident, $($lhs: ident -> $rhs1:tt $($rhs: ident)* = $action:expr);+ $(;)?) => {{
        $(
            $crate::prod!($p, $lhs -> $rhs1 $($rhs)*, $action);
        )+
    }};
}

#[macro_export]
macro_rules! terminals {
    ($p:ident, $default_action:expr, $(($ty:ident, $re:expr $(, $action:expr)?)),+ $(,)?) => {{
        let mut rules: Vec<(TokenType, String)> = Vec::new();
        $(
            // Pick the override if provided, otherwise use the default
            let action = $crate::terminals!(@pick_action $default_action $(, $action)?);
            $p.define_terminal(TokenType::$ty, action, Some(stringify!($ty).to_string())).unwrap();
            rules.push((TokenType::$ty, $re.to_string()));
        )+

        rules
    }};

    // Internal matcher to pick the correct action
    (@pick_action $default_action:expr) => { $default_action };
    (@pick_action $default_action:expr, $override_action:expr) => { $override_action };
}

#[macro_export]
macro_rules! skip {
    ($(($ty: ident, $re: expr)),* $(,)?) => {{
        let mut rules: Vec<(TokenType, String)> = Vec::new();
        $(
            rules.push((TokenType::$ty, $re.to_string()));
        )*

        rules
    }};
}

#[macro_export]
macro_rules! grammar {
    (
        token_type: $T:ty,
        return_type: $R:ty,
        lexer_definer_type: $LexDef:ty,
        first_symbol: $first_symbol:ident,
        default_token_action: $tok_action:expr,

        productions: {
            $($lhs:ident -> $rhs1:tt $($rhs:ident)* = $action:expr);+ ;
        }

        terminals: {
            $(($term_name:ident, $regex:expr $(, $specific_tok_action:expr)?)),+ $(,)?
        }

        skip: {
            $(($skip_term_name:ident, $skip_re:literal)),* $(,)?
        }

    ) => {{
        #[allow(unused_mut)]
        {
            use $crate::Grammar;
            use $crate::DefineLexer;
            use $T as TokenType;
            use $LexDef as LexerDefiner;

            let mut p: Grammar<TokenType, $R> = Grammar::new();

            // Define first symbol
            p.define_first_symbol(Some(stringify!($first_symbol).to_string())).unwrap();

            // Register terminals with optional per-token action
            let mut term_rules = $crate::terminals!(
                p,
                $tok_action,
                $(($term_name, $regex $(, $specific_tok_action)?)),+
            );

            // Register productions
            $crate::productions!(p, $($lhs -> $rhs1 $($rhs)* = $action);+);

            let skip_rules = $crate::skip!($(($skip_term_name, $skip_re)),*);


            match p.build_parser() {
                Err(err) => {
                    panic!("Parser building errors: \n{}", err.join("\n\n"))
                },
                Ok(parser) => {

                    let mut lex_def = LexerDefiner::new();
                    lex_def.rules(term_rules);
                    lex_def.skip_rules(skip_rules);
                    let lexer = lex_def.compile();

                    (lexer, parser)
                }
            }
        }
    }};
}
