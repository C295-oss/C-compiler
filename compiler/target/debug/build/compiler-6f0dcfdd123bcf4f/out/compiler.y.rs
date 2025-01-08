 mod compiler_y {

// User code from the program section

fn parse_int(s: &str) -> Result<u64, ()> {
    match s.parse::<u64>() {
        Ok(val) => Ok(val),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}
// End of user code from the program section


    // User actions

    // Expr
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_0<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                     __gt_span: ::cfgrammar::Span,
                     _: (),
                     mut __gt_arg_1: Result<u64, ()>,
                     mut __gt_arg_2: ::std::result::Result<lrlex::defaults::DefaultLexeme, lrlex::defaults::DefaultLexeme>,
                     mut __gt_arg_3: Result<u64, ()>)
                 -> Result<u64, ()> {
Ok(__gt_arg_1? + __gt_arg_3?)
    }

    // Expr
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_1<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                     __gt_span: ::cfgrammar::Span,
                     _: (),
                     mut __gt_arg_1: Result<u64, ()>)
                 -> Result<u64, ()> {
__gt_arg_1
    }

    // Term
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_2<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                     __gt_span: ::cfgrammar::Span,
                     _: (),
                     mut __gt_arg_1: Result<u64, ()>,
                     mut __gt_arg_2: ::std::result::Result<lrlex::defaults::DefaultLexeme, lrlex::defaults::DefaultLexeme>,
                     mut __gt_arg_3: Result<u64, ()>)
                 -> Result<u64, ()> {
Ok(__gt_arg_1? * __gt_arg_3?)
    }

    // Term
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_3<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                     __gt_span: ::cfgrammar::Span,
                     _: (),
                     mut __gt_arg_1: Result<u64, ()>)
                 -> Result<u64, ()> {
__gt_arg_1
    }

    // Factor
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_4<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                     __gt_span: ::cfgrammar::Span,
                     _: (),
                     mut __gt_arg_1: ::std::result::Result<lrlex::defaults::DefaultLexeme, lrlex::defaults::DefaultLexeme>,
                     mut __gt_arg_2: Result<u64, ()>,
                     mut __gt_arg_3: ::std::result::Result<lrlex::defaults::DefaultLexeme, lrlex::defaults::DefaultLexeme>)
                 -> Result<u64, ()> {
__gt_arg_2
    }

    // Factor
    #[allow(clippy::too_many_arguments)]
    fn __gt_action_5<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                     __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                     __gt_span: ::cfgrammar::Span,
                     _: (),
                     mut __gt_arg_1: ::std::result::Result<lrlex::defaults::DefaultLexeme, lrlex::defaults::DefaultLexeme>)
                 -> Result<u64, ()> {
let v = __gt_arg_1.map_err(|_| ())?;
          parse_int(__gt_lexer.span_str(v.span()))
    }

    mod _parser_ {
        #![allow(clippy::type_complexity)]
        #![allow(clippy::unnecessary_wraps)]
        #![deny(unsafe_code)]
        #[allow(unused_imports)]
        use super::*;
#[allow(dead_code)] const __GRM_DATA: &[u8] = &[4,0,0,0,4,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,94,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,69,120,112,114,38,0,0,0,0,0,0,0,42,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,84,101,114,109,131,0,0,0,0,0,0,0,135,0,0,0,0,0,0,0,6,0,0,0,0,0,0,0,70,97,99,116,111,114,228,0,0,0,0,0,0,0,234,0,0,0,0,0,0,0,6,0,0,0,0,0,0,0,1,28,0,0,0,0,0,0,0,31,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,73,78,84,1,76,0,0,0,0,0,0,0,77,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,43,1,169,0,0,0,0,0,0,0,170,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,42,1,7,1,0,0,0,0,0,0,8,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,40,1,16,1,0,0,0,0,0,0,17,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,41,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,6,0,0,0,0,0,0,0,1,3,0,0,0,0,0,0,0,73,78,84,1,1,0,0,0,0,0,0,0,43,1,1,0,0,0,0,0,0,0,42,1,1,0,0,0,0,0,0,0,40,1,1,0,0,0,0,0,0,0,41,0,6,0,0,0,5,0,0,0,7,0,0,0,6,0,0,0,7,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,2,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,1,0,0,0,2,0,0,0,0,0,0,0,3,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,3,0,0,0,0,0,0,0,1,0,0,0,3,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,4,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,4,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,6,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,2,0,0,0,0,0,0,0,2,0,0,0,3,0,0,0,2,0,0,0,0,0,0,0,4,0,0,0,5,0,0,0,7,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,2,0,0,0,2,0,0,0,3,0,0,0,3,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,1,13,0,0,0,0,0,0,0,79,107,40,36,49,63,32,43,32,36,51,63,41,1,2,0,0,0,0,0,0,0,36,49,1,13,0,0,0,0,0,0,0,79,107,40,36,49,63,32,42,32,36,51,63,41,1,2,0,0,0,0,0,0,0,36,49,1,2,0,0,0,0,0,0,0,36,50,1,76,0,0,0,0,0,0,0,108,101,116,32,118,32,61,32,36,49,46,109,97,112,95,101,114,114,40,124,95,124,32,40,41,41,63,59,13,10,32,32,32,32,32,32,32,32,32,32,112,97,114,115,101,95,105,110,116,40,36,108,101,120,101,114,46,115,112,97,110,95,115,116,114,40,118,46,115,112,97,110,40,41,41,41,0,0,1,228,0,0,0,0,0,0,0,102,110,32,112,97,114,115,101,95,105,110,116,40,115,58,32,38,115,116,114,41,32,45,62,32,82,101,115,117,108,116,60,117,54,52,44,32,40,41,62,32,123,13,10,32,32,32,32,109,97,116,99,104,32,115,46,112,97,114,115,101,58,58,60,117,54,52,62,40,41,32,123,13,10,32,32,32,32,32,32,32,32,79,107,40,118,97,108,41,32,61,62,32,79,107,40,118,97,108,41,44,13,10,32,32,32,32,32,32,32,32,69,114,114,40,95,41,32,61,62,32,123,13,10,32,32,32,32,32,32,32,32,32,32,32,32,101,112,114,105,110,116,108,110,33,40,34,123,125,32,99,97,110,110,111,116,32,98,101,32,114,101,112,114,101,115,101,110,116,101,100,32,97,115,32,97,32,117,54,52,34,44,32,115,41,59,13,10,32,32,32,32,32,32,32,32,32,32,32,32,69,114,114,40,40,41,41,13,10,32,32,32,32,32,32,32,32,125,13,10,32,32,32,32,125,13,10,125,4,0,0,0,0,0,0,0,0,1,15,0,0,0,0,0,0,0,82,101,115,117,108,116,60,117,54,52,44,32,40,41,62,1,15,0,0,0,0,0,0,0,82,101,115,117,108,116,60,117,54,52,44,32,40,41,62,1,15,0,0,0,0,0,0,0,82,101,115,117,108,116,60,117,54,52,44,32,40,41,62,1,6,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,];
#[allow(dead_code)] const __STABLE_DATA: &[u8] = &[12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,27,0,0,0,0,0,0,0,5,0,0,0,0,0,0,0,10,0,0,0,0,0,0,0,27,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,15,0,0,0,0,0,0,0,20,0,0,0,0,0,0,0,25,0,0,0,0,0,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,72,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,118,98,119,73,210,218,118,146,36,0,0,0,0,0,0,0,33,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,97,0,161,97,89,137,101,21,72,18,32,73,142,3,56,142,165,162,116,138,34,8,64,40,0,0,0,0,0,0,0,12,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,72,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,137,157,136,182,45,37,137,109,219,0,0,0,0,0,0,0,12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,0,0,0,0,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,48,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,241,241,255,63,247,255,0,0,10,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,108,107,117,86,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,84,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,16,0,32,64,0,0,0,72,128,0,0,0,0,0,0,72,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,9,144,8,4,32,37,9,64,0,0,0,0,0,0,0,0,12,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,34,10,0,0,0,0,0,0,7,0,0,0,6,0,0,0,0,];

    #[allow(dead_code)]
    pub fn parse<'lexer, 'input: 'lexer>(
        lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>)
          -> (::std::option::Option<Result<u64, ()>>, ::std::vec::Vec<::lrpar::LexParseError<u32, lrlex::defaults::DefaultLexerTypes>>)
    {
        let (grm, stable) = ::lrpar::ctbuilder::_reconstitute(__GRM_DATA, __STABLE_DATA);
        #[allow(clippy::type_complexity)]
        let actions: ::std::vec::Vec<&dyn Fn(::cfgrammar::RIdx<u32>,
                       &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                       ::cfgrammar::Span,
                       ::std::vec::Drain<'_,  ::lrpar::parser::AStackType<<lrlex::defaults::DefaultLexerTypes as ::lrpar::LexerTypes>::LexemeT, __GtActionsKind<'input>>>,
                       ())
                    -> __GtActionsKind<'input>> = ::std::vec![&__gt_wrapper_0,
                        &__gt_wrapper_1,
                        &__gt_wrapper_2,
                        &__gt_wrapper_3,
                        &__gt_wrapper_4,
                        &__gt_wrapper_5,
                        &__gt_wrapper_6];

        match ::lrpar::RTParserBuilder::new(&grm, &stable)
            .recoverer(::lrpar::RecoveryKind::CPCTPlus)
            .parse_actions(lexer, &actions, ()) {
                (Some(__GtActionsKind::Ak1(x)), y) => (Some(x), y),
                (None, y) => (None, y),
                _ => unreachable!()
        }
    }

    #[allow(dead_code)]
    pub const R_EXPR: u32 = 1;
    #[allow(dead_code)]
    pub const R_TERM: u32 = 2;
    #[allow(dead_code)]
    pub const R_FACTOR: u32 = 3;
    const __GT_EPP: &[::std::option::Option<&str>] = &[Some("INT"), Some("+"), Some("*"), Some("("), Some(")"), None];

    /// Return the %epp entry for token `tidx` (where `None` indicates "the token has no
    /// pretty-printed value"). Panics if `tidx` doesn't exist.
    #[allow(dead_code)]
    pub fn token_epp<'a>(tidx: ::cfgrammar::TIdx<u32>) -> ::std::option::Option<&'a str> {
        __GT_EPP[usize::from(tidx)]
    }

    // Wrappers

    fn __gt_wrapper_0<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                      __gt_span: ::cfgrammar::Span,
                      mut __gt_args: ::std::vec::Drain<'_,  ::lrpar::parser::AStackType<<lrlex::defaults::DefaultLexerTypes as ::lrpar::LexerTypes>::LexemeT, __GtActionsKind<'input>>>,
                      _: ())
                   -> __GtActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GtActionsKind::Ak1(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_2 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.faulty() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_3 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GtActionsKind::Ak2(x)) => x,
            _ => unreachable!()
        };
        __GtActionsKind::Ak1(__gt_action_0(__gt_ridx, __gt_lexer, __gt_span, (), __gt_arg_1, __gt_arg_2, __gt_arg_3))
    }

    fn __gt_wrapper_1<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                      __gt_span: ::cfgrammar::Span,
                      mut __gt_args: ::std::vec::Drain<'_,  ::lrpar::parser::AStackType<<lrlex::defaults::DefaultLexerTypes as ::lrpar::LexerTypes>::LexemeT, __GtActionsKind<'input>>>,
                      _: ())
                   -> __GtActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GtActionsKind::Ak2(x)) => x,
            _ => unreachable!()
        };
        __GtActionsKind::Ak1(__gt_action_1(__gt_ridx, __gt_lexer, __gt_span, (), __gt_arg_1))
    }

    fn __gt_wrapper_2<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                      __gt_span: ::cfgrammar::Span,
                      mut __gt_args: ::std::vec::Drain<'_,  ::lrpar::parser::AStackType<<lrlex::defaults::DefaultLexerTypes as ::lrpar::LexerTypes>::LexemeT, __GtActionsKind<'input>>>,
                      _: ())
                   -> __GtActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GtActionsKind::Ak2(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_2 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.faulty() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_3 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GtActionsKind::Ak3(x)) => x,
            _ => unreachable!()
        };
        __GtActionsKind::Ak2(__gt_action_2(__gt_ridx, __gt_lexer, __gt_span, (), __gt_arg_1, __gt_arg_2, __gt_arg_3))
    }

    fn __gt_wrapper_3<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                      __gt_span: ::cfgrammar::Span,
                      mut __gt_args: ::std::vec::Drain<'_,  ::lrpar::parser::AStackType<<lrlex::defaults::DefaultLexerTypes as ::lrpar::LexerTypes>::LexemeT, __GtActionsKind<'input>>>,
                      _: ())
                   -> __GtActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GtActionsKind::Ak3(x)) => x,
            _ => unreachable!()
        };
        __GtActionsKind::Ak2(__gt_action_3(__gt_ridx, __gt_lexer, __gt_span, (), __gt_arg_1))
    }

    fn __gt_wrapper_4<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                      __gt_span: ::cfgrammar::Span,
                      mut __gt_args: ::std::vec::Drain<'_,  ::lrpar::parser::AStackType<<lrlex::defaults::DefaultLexerTypes as ::lrpar::LexerTypes>::LexemeT, __GtActionsKind<'input>>>,
                      _: ())
                   -> __GtActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.faulty() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        let __gt_arg_2 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::ActionType(__GtActionsKind::Ak1(x)) => x,
            _ => unreachable!()
        };
        let __gt_arg_3 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.faulty() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        __GtActionsKind::Ak3(__gt_action_4(__gt_ridx, __gt_lexer, __gt_span, (), __gt_arg_1, __gt_arg_2, __gt_arg_3))
    }

    fn __gt_wrapper_5<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                      __gt_span: ::cfgrammar::Span,
                      mut __gt_args: ::std::vec::Drain<'_,  ::lrpar::parser::AStackType<<lrlex::defaults::DefaultLexerTypes as ::lrpar::LexerTypes>::LexemeT, __GtActionsKind<'input>>>,
                      _: ())
                   -> __GtActionsKind<'input> {
        let __gt_arg_1 = match __gt_args.next().unwrap() {
            ::lrpar::parser::AStackType::Lexeme(l) => {
                if l.faulty() {
                    Err(l)
                } else {
                    Ok(l)
                }
            },
            ::lrpar::parser::AStackType::ActionType(_) => unreachable!()
        };
        __GtActionsKind::Ak3(__gt_action_5(__gt_ridx, __gt_lexer, __gt_span, (), __gt_arg_1))
    }

    fn __gt_wrapper_6<'lexer, 'input: 'lexer>(__gt_ridx: ::cfgrammar::RIdx<u32>,
                      __gt_lexer: &'lexer dyn ::lrpar::NonStreamingLexer<'input, lrlex::defaults::DefaultLexerTypes>,
                      __gt_span: ::cfgrammar::Span,
                      mut __gt_args: ::std::vec::Drain<'_,  ::lrpar::parser::AStackType<<lrlex::defaults::DefaultLexerTypes as ::lrpar::LexerTypes>::LexemeT, __GtActionsKind<'input>>>,
                      _: ())
                   -> __GtActionsKind<'input> {
        unreachable!()
    }

    #[allow(dead_code)]
    enum __GtActionsKind<'input> {
        Ak1(Result<u64, ()>),
        Ak2(Result<u64, ()>),
        Ak3(Result<u64, ()>),
    ___GtActionsKindHidden(::std::marker::PhantomData<&'input ()>)
    }

    } // End of `mod _parser_`

    #[allow(unused_imports)]
    pub use _parser_::*;
    #[allow(unused_imports)]
    use ::lrpar::Lexeme;
} // End of `mod {mod_name}` 


/* CACHE INFORMATION
   Build time: "2025-01-07T19:50:37.752987200Z"
   Grammar path: Some("C:\\Users\\charl\\OneDrive\\Desktop\\sysProg\\cCompiler\\C-compiler\\compiler\\src\\compiler.y")
   Mod name: None
   Recoverer: CPCTPlus
   YaccKind: Some(Grmtools)
   Visibility: ""
   Error on conflicts: true

   0 'INT'
   1 '+'
   2 '*'
   3 '('
   4 ')'
   5 <unknown>
*/
