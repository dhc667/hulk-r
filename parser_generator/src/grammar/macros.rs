#[macro_export]
macro_rules! prod {
    ($p: ident, $lhs: ident -> #Epsilon, $compute: expr) => {{
        #[allow(non_snake_case, unused_variables)]
        {
            let $lhs = $p.get_or_create_non_terminal(stringify!($lhs).to_string());
            $p.production(*$lhs.as_non_terminal_id().unwrap(), Vec::new(), $compute)
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
        use lexer::lexer_generator::rule::Rule;
        let mut rules = Vec::new();
        $(
            // Pick the override if provided, otherwise use the default
            let action = $crate::terminals!(@pick_action $default_action $(, $action)?);
            $p.define_terminal(TokenType::$ty, action, Some(stringify!($ty).to_string())).unwrap();
            rules.push((TokenType::$ty, $re));
        )+
        let rules: Vec<Rule<TokenType>> = rules
            .into_iter()
            .map(|(tok, s)| (tok, s.to_string()))
            .map(|(tok, re)| Rule::new(tok, re))
            .collect();
        rules
    }};

    // Internal matcher to pick the correct action
    (@pick_action $default_action:expr) => { $default_action };
    (@pick_action $default_action:expr, $override_action:expr) => { $override_action };
}

#[macro_export]
macro_rules! skip {
    ($(($ty: ident, $re: expr)),+ $(,)?) => {{
        use lexer::lexer_generator::rule::Rule;
        let mut rules = Vec::new();
        $(
            rules.push(($ty, $re));
        )+
        let rules: Vec<Rule<TokenType>> = rules
            .into_iter()
            .map(|(tok, s)| (tok, s.to_string()))
            .map(|(tok, re)| Rule::new_skip(tok, re))
            .collect();

        rules
    }};
}

#[macro_export]
macro_rules! grammar {
    (
        token_type: $TokenType:ty,
        return_type: $R:ty,
        lexer_type: $Lexer:ty,
        rule_type: $Rule:ty,
        first_symbol: $first_symbol:ident,
        default_token_action: $tok_action:expr,

        productions: {
            $($lhs:ident -> $rhs1:tt $($rhs:ident)* = $action:expr);+ ;
        }

        terminals: {
            $(($term_name:ident, $regex:expr $(, $specific_tok_action:expr)?)),+ $(,)?
        }

        $(
            SKIP $skip_name:ident $skip_re:literal;
        )*
    ) => {{
        #[allow(unused_mut)]
        {
            use $crate::Grammar;
            use $Lexer as Lexer;
            use $Rule as Rule;
            use $TokenType as TokenType;

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

            // Register skip rules
            let mut skip_rules: Vec<Rule<TokenType>> = Vec::new();
            $(
                skip_rules.push(Rule::new_skip(TokenType::$skip_name, $skip_re.to_string()));
            )*

            term_rules.extend(skip_rules);

            match p.build_parser() {
                Err(err) => {
                    panic!("Parser building errors: \n{}", err.join("\n\n"))
                },
                Ok(parser) => {
                    (Lexer::new(term_rules), parser)
                }
            }
        }
    }};
}
