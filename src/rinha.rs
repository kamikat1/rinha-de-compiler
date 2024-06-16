// auto-generated: "lalrpop 0.20.2"
// sha3: c694bc521f05e9c04233430f8f9ce726da519122cbd46749ad50a34bc897a56e
use crate::ast::Element;
use std::str::FromStr;
use lalrpop_util::ErrorRecovery;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::never_loop, clippy::match_single_binding, clippy::needless_raw_string_hashes)]
mod __parse__File {

    use crate::ast::Element;
    use std::str::FromStr;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(__lalrpop_util::ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>),
        Variant2(crate::parser::Var),
        Variant3(alloc::vec::Vec<crate::parser::Var>),
        Variant4(crate::ast::Term),
        Variant5(alloc::vec::Vec<crate::ast::Term>),
        Variant6(usize),
        Variant7(crate::ast::BinaryOp),
        Variant8(crate::ast::File),
        Variant9(i32),
        Variant10(bool),
        Variant11(core::option::Option<crate::parser::Var>),
        Variant12(std::string::String),
        Variant13(Vec<crate::parser::Var>),
        Variant14(Vec<crate::ast::Term>),
        Variant15(core::option::Option<crate::ast::Term>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 1
        -26, 51, -26, 9, -26, 52, -26, -26, -26, 10, 53, -26, -26, -26, 0, -26, 0, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, 0, 0, 0, 0,
        // State 2
        54, 0, 55, 0, -35, 0, 0, -35, 0, 0, 0, -35, 56, 57, 0, 58, 0, 59, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, -35, 0, 0, 0, 0,
        // State 3
        -15, 0, -15, 0, -15, 0, 62, -15, 63, 0, 0, -15, -15, -15, 0, -15, 0, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0,
        // State 6
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 7
        0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 0, 0, 0, 44, 45, 46, 0, 0, 0, 47, 48, 49, 0,
        // State 8
        0, 0, 0, 5, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0,
        // State 10
        0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 0, 0, 0, 44, 45, 46, 0, 0, 0, 47, 48, 49, 0,
        // State 11
        0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 0, 0, 0, 44, 45, 46, 0, 0, 0, 47, 48, 49, 0,
        // State 12
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 13
        0, 0, 0, 0, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0,
        // State 14
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 15
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 16
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 17
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 18
        0, 0, 0, 5, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 19
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 20
        0, 0, 0, 0, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0,
        // State 21
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 22
        0, 0, 0, 5, -72, 0, 0, -72, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, -72, 47, 48, 49, 50,
        // State 23
        0, 0, 0, 5, -76, 0, 0, -76, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, -76, 47, 48, 49, 50,
        // State 24
        0, 0, 0, 5, -70, 0, 0, -70, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, -70, 47, 48, 49, 50,
        // State 25
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 26
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 27
        0, 0, 0, 5, -74, 0, 0, -74, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, -74, 47, 48, 49, 50,
        // State 28
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 40, 41, 42, 43, 6, 44, 45, 46, 7, 0, 0, 47, 48, 49, 50,
        // State 29
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, 0, -14, 0, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, -50, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, -50, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, -64, 0, 0, -64, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0,
        // State 33
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, -13, 0, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, 0, 0, 0, 0,
        // State 34
        -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, -48, 0, 0, 0, 0,
        // State 35
        -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, -49, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, -51, 0, 0, 0, 0,
        // State 38
        -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, 0, -80, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, -80, 0, 0, 0, 0,
        // State 39
        -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, -47, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, 0, 0, 0, 0,
        // State 46
        -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, 0, -63, 0, -63, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, -63, 0, 0, 0, 0,
        // State 47
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, -33, 0, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, 0, 0, 0, 0,
        // State 48
        -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, 0, -81, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, -81, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, -77, 0, 0, -77, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0,
        // State 50
        0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, -31, -31, 0, 0, 0, -31, -31, -31, 0, 0, 0, -31, -31, -31, 0,
        // State 51
        0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, -29, -29, 0, 0, 0, -29, -29, -29, 0, 0, 0, -29, -29, -29, 0,
        // State 52
        0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, -30, -30, 0, 0, 0, -30, -30, -30, 0, 0, 0, -30, -30, -30, 0,
        // State 53
        0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, -40, -40, 0, 0, 0, -40, -40, -40, 0, 0, 0, -40, -40, -40, 0,
        // State 54
        0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, -37, -37, 0, 0, 0, -37, -37, -37, 0, 0, 0, -37, -37, -37, 0,
        // State 55
        0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, -43, -43, 0, 0, 0, -43, -43, -43, 0, 0, 0, -43, -43, -43, 0,
        // State 56
        0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, -41, -41, 0, 0, 0, -41, -41, -41, 0, 0, 0, -41, -41, -41, 0,
        // State 57
        0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, -39, -39, 0, 0, 0, -39, -39, -39, 0, 0, 0, -39, -39, -39, 0,
        // State 58
        0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, -44, -44, 0, 0, 0, -44, -44, -44, 0, 0, 0, -44, -44, -44, 0,
        // State 59
        0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, -42, -42, 0, 0, 0, -42, -42, -42, 0, 0, 0, -42, -42, -42, 0,
        // State 60
        0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, -38, -38, 0, 0, 0, -38, -38, -38, 0, 0, 0, -38, -38, -38, 0,
        // State 61
        0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, -17, -17, 0, 0, 0, -17, -17, -17, 0, 0, 0, -17, -17, -17, 0,
        // State 62
        0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, -18, -18, 0, 0, 0, -18, -18, -18, 0, 0, 0, -18, -18, -18, 0,
        // State 63
        0, 0, 0, 0, 73, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 0, 0, 0, 0,
        // State 66
        -28, 0, -28, 0, -28, 0, -28, -28, -28, 0, 0, -28, -28, -28, 0, -28, 0, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 84, 0, 0, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, 0, 0, 0, 0,
        // State 69
        -27, 0, -27, 0, -27, 0, -27, -27, -27, 0, 0, -27, -27, -27, 0, -27, 0, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, -36, 0, 0, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0,
        // State 71
        -16, 0, -16, 0, -16, 0, 0, -16, 0, 0, 0, -16, -16, -16, 0, -16, 0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, 0, 0, 0, 0,
        // State 72
        -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 90, 0, 0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, -66, 0, 0, -66, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 96, 0, 0, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, 0, 0, 0, 0,
        // State 83
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, -22, 0, 0, 0, 0,
        // State 84
        0, 0, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, -9, -9, -9, -9, -9, -9, -9, -9, -9, 0, 0, -9, -9, -9, -9,
        // State 85
        0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 0, -20, 0, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 99, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, 0, -19, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0,
        // State 94
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 0, -21, 0, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, 0, 0, 0, 0,
        // State 95
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, 0, 0, 0, 0,
        // State 96
        0, 0, 0, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, -10, -10, -10, -10, -10, -10, -10, -10, -10, 0, 0, -10, -10, -10, -10,
        // State 97
        0, 0, 0, 0, -65, 0, 0, -65, 0, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, 0,
        // State 98
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 100
        0, 0, 0, 0, -71, 0, 0, -71, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, 0,
        // State 101
        0, 0, 0, 0, -75, 0, 0, -75, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, -69, 0, 0, -69, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0,
        // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 107, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, -67, 0, 0, -67, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, -73, 0, 0, -73, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0,
        // State 108
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 0, 0, 0, 0,
        // State 109
        0, 0, 0, 0, -68, 0, 0, -68, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 37 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -26,
        // State 2
        -35,
        // State 3
        -15,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -72,
        // State 23
        -76,
        // State 24
        -70,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -74,
        // State 28
        0,
        // State 29
        -14,
        // State 30
        -82,
        // State 31
        -50,
        // State 32
        -64,
        // State 33
        -13,
        // State 34
        -48,
        // State 35
        -49,
        // State 36
        -32,
        // State 37
        -51,
        // State 38
        -80,
        // State 39
        -47,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -46,
        // State 46
        -63,
        // State 47
        -33,
        // State 48
        -81,
        // State 49
        -77,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        -28,
        // State 67
        0,
        // State 68
        -23,
        // State 69
        -27,
        // State 70
        -36,
        // State 71
        -16,
        // State 72
        -45,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        -66,
        // State 80
        0,
        // State 81
        0,
        // State 82
        -25,
        // State 83
        -22,
        // State 84
        0,
        // State 85
        0,
        // State 86
        -20,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        0,
        // State 91
        0,
        // State 92
        0,
        // State 93
        -19,
        // State 94
        -21,
        // State 95
        -24,
        // State 96
        0,
        // State 97
        -65,
        // State 98
        0,
        // State 99
        0,
        // State 100
        -71,
        // State 101
        -75,
        // State 102
        -69,
        // State 103
        0,
        // State 104
        -67,
        // State 105
        -73,
        // State 106
        0,
        // State 107
        0,
        // State 108
        0,
        // State 109
        -68,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 20,
            5 => 18,
            8 => 1,
            9 => match state {
                11 => 71,
                _ => 2,
            },
            10 => 11,
            11 => 29,
            12 => match state {
                7 => 66,
                _ => 3,
            },
            13 => 7,
            14 => 30,
            15 => 31,
            17 => match state {
                10 => 70,
                _ => 32,
            },
            18 => 10,
            19 => 33,
            20 => match state {
                5 => 64,
                9 => 69,
                13 => 74,
                20 => 87,
                _ => 34,
            },
            25 => 35,
            26 => match state {
                4 => 63,
                6 => 65,
                8 => 67,
                12 => 73,
                14 => 76,
                15 => 77,
                16 => 78,
                17 => 80,
                18 => 81,
                19 => 85,
                21 => 92,
                22 => 100,
                23 => 101,
                24 => 102,
                25 => 103,
                26 => 104,
                27 => 105,
                28 => 108,
                _ => 36,
            },
            28 => 37,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""!=""###,
        r###""%""###,
        r###""&&""###,
        r###""(""###,
        r###"")""###,
        r###""*""###,
        r###""+""###,
        r###"",""###,
        r###""-""###,
        r###"".""###,
        r###""/""###,
        r###"";""###,
        r###""<""###,
        r###""<=""###,
        r###""=""###,
        r###""==""###,
        r###""=>""###,
        r###"">""###,
        r###"">=""###,
        r###""_""###,
        r###""else""###,
        r###""external""###,
        r###""false""###,
        r###""first""###,
        r###""fn""###,
        r###""if""###,
        r###""let""###,
        r###""print""###,
        r###""second""###,
        r###""true""###,
        r###""{""###,
        r###""||""###,
        r###""}""###,
        r###"r#"\"(\\\\\\\\|\\\\\"|[^\"\\\\])*\""#"###,
        r###"r#"[0123456789]+"#"###,
        r###"r#"[a-zA-Z][a-zA-Z0-9/_]*"#"###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'err,
        '__2,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    where
        'input: 'err,
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'err, '__2>
    where 'input: 'err
    {
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &'__2 str,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err, '__2> __state_machine::ParserDefinition for __StateMachine<'input, 'err, '__2>
    where 'input: 'err
    {
        type Location = usize;
        type Error = crate::parser::InnerError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = crate::ast::File;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 37 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant1(recovery)
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.errors,
                self.filename,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(3, _) if true => Some(0),
            Token(4, _) if true => Some(1),
            Token(5, _) if true => Some(2),
            Token(6, _) if true => Some(3),
            Token(7, _) if true => Some(4),
            Token(8, _) if true => Some(5),
            Token(9, _) if true => Some(6),
            Token(10, _) if true => Some(7),
            Token(11, _) if true => Some(8),
            Token(12, _) if true => Some(9),
            Token(13, _) if true => Some(10),
            Token(14, _) if true => Some(11),
            Token(15, _) if true => Some(12),
            Token(16, _) if true => Some(13),
            Token(17, _) if true => Some(14),
            Token(18, _) if true => Some(15),
            Token(19, _) if true => Some(16),
            Token(20, _) if true => Some(17),
            Token(21, _) if true => Some(18),
            Token(22, _) if true => Some(19),
            Token(23, _) if true => Some(20),
            Token(24, _) if true => Some(21),
            Token(25, _) if true => Some(22),
            Token(26, _) if true => Some(23),
            Token(27, _) if true => Some(24),
            Token(28, _) if true => Some(25),
            Token(29, _) if true => Some(26),
            Token(30, _) if true => Some(27),
            Token(31, _) if true => Some(28),
            Token(32, _) if true => Some(29),
            Token(33, _) if true => Some(30),
            Token(34, _) if true => Some(31),
            Token(35, _) if true => Some(32),
            Token(0, _) if true => Some(33),
            Token(1, _) if true => Some(34),
            Token(2, _) if true => Some(35),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 => match __token {
                Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(30, __tok0) | Token(31, __tok0) | Token(32, __tok0) | Token(33, __tok0) | Token(34, __tok0) | Token(35, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
        '__2,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err, '__2>>
    where
        'input: 'err,
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 11,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 21,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 23,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 24,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 26,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 11,
                    nonterminal_produced: 26,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 26,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 26,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 26,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 26,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 26,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 27,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            81 => __state_machine::SimulatedReduce::Accept,
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct FileParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for FileParser { fn default() -> Self { Self::new() } }
    impl FileParser {
        pub fn new() -> FileParser {
            let __builder = super::__intern_token::new_builder();
            FileParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
        >(
            &self,
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
            filename: &str,
            input: &'input str,
        ) -> Result<crate::ast::File, __lalrpop_util::ParseError<usize, Token<'input>, crate::parser::InnerError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    errors,
                    filename,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
        '__2,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> bool
    where
        'input: 'err,
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<crate::ast::File,__lalrpop_util::ParseError<usize, Token<'input>, crate::parser::InnerError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                __reduce54(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            55 => {
                __reduce55(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                __reduce56(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            57 => {
                __reduce57(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                __reduce70(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            71 => {
                __reduce71(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            72 => {
                __reduce72(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            73 => {
                __reduce73(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            74 => {
                __reduce74(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            75 => {
                __reduce75(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            76 => {
                __reduce76(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            77 => {
                __reduce77(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            78 => {
                __reduce78(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            79 => {
                __reduce79(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            80 => {
                __reduce80(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            81 => {
                // __File = File => ActionFn(0);
                let __sym0 = __pop_Variant8(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(errors, filename, input, __sym0);
                return Some(Ok(__nt));
            }
            82 => {
                __reduce82(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<crate::ast::Term>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<crate::parser::Var>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<crate::ast::Term>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<crate::parser::Var>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<crate::ast::Term>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<crate::parser::Var>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, crate::ast::BinaryOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, crate::ast::File, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, crate::ast::Term, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, crate::parser::Var, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, std::string::String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",") = Reference, "," => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action62::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",")* =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",")* = (<Reference> ",")+ => ActionFn(61);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action61::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",")+ = Reference, "," => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action67::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",")+ = (<Reference> ",")+, Reference, "," => ActionFn(68);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action68::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",") = Term, "," => ActionFn(57);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action57::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce6<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action55::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 4)
    }
    fn __reduce7<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")* = (<Term> ",")+ => ActionFn(56);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")+ = Term, "," => ActionFn(71);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action71::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 5)
    }
    fn __reduce9<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")+ = (<Term> ",")+, Term, "," => ActionFn(72);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action72::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 5)
    }
    fn __reduce10<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action54::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce11<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(53);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action53::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    fn __reduce12<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Apply = Primary => ActionFn(35);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action35::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    fn __reduce13<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Apply = Call => ActionFn(36);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    fn __reduce14<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arithmetic = Factor => ActionFn(23);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 9)
    }
    fn __reduce15<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arithmetic = Factor, ArithmeticOp, Arithmetic => ActionFn(94);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action94::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 9)
    }
    fn __reduce16<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ArithmeticOp = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce17<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ArithmeticOp = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action22::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce18<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = "print", "(", Term, ")" => ActionFn(95);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action95::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce19<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = "first", "(", Term, ")" => ActionFn(96);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action96::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce20<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = "second", "(", Term, ")" => ActionFn(97);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action97::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce21<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = Apply, "(", Term, ")" => ActionFn(133);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action133::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce22<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = Apply, "(", ")" => ActionFn(134);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action134::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 11)
    }
    fn __reduce23<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = Apply, "(", (<Term> ",")+, Term, ")" => ActionFn(135);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action135::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 11)
    }
    fn __reduce24<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = Apply, "(", (<Term> ",")+, ")" => ActionFn(136);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action136::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce25<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Factor = Apply => ActionFn(18);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action18::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 12)
    }
    fn __reduce26<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Factor = Apply, ".", Reference => ActionFn(99);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action99::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 12)
    }
    fn __reduce27<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Factor = Apply, FactorOp, Factor => ActionFn(100);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action100::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action15::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce29<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce30<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "%" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce31<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // File = Term => ActionFn(101);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action101::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    fn __reduce32<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Int = r#"[0123456789]+"# => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 15)
    }
    fn __reduce33<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // IsExternal = "external" => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 16)
    }
    fn __reduce34<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Logical = Arithmetic => ActionFn(33);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action33::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    fn __reduce35<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Logical = Arithmetic, LogicalOp, Logical => ActionFn(102);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action102::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 17)
    }
    fn __reduce36<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "&&" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce37<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "||" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce38<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "==" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action27::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce39<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "!=" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action28::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce40<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "<=" => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action29::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce41<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = ">=" => ActionFn(30);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce42<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "<" => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce43<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = ">" => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce44<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = "(", Term, ")" => ActionFn(5);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action5::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    fn __reduce45<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action103::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce46<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action104::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce47<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = Reference => ActionFn(8);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce48<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = String => ActionFn(105);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action105::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce49<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = Int => ActionFn(106);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action106::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce50<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Reference = Text => ActionFn(107);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action107::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 20)
    }
    fn __reduce51<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Reference? = Reference => ActionFn(58);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    fn __reduce52<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Reference? =  => ActionFn(59);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action59::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    fn __reduce53<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Semi = ";" => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 22)
    }
    fn __reduce54<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Reference> = Reference => ActionFn(113);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action113::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 23)
    }
    fn __reduce55<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Reference> =  => ActionFn(114);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action114::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 23)
    }
    fn __reduce56<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Reference> = (<Reference> ",")+, Reference => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action115::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 23)
    }
    fn __reduce57<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Reference> = (<Reference> ",")+ => ActionFn(116);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action116::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 23)
    }
    fn __reduce58<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Term> = Term => ActionFn(121);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action121::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 24)
    }
    fn __reduce59<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Term> =  => ActionFn(122);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action122::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 24)
    }
    fn __reduce60<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Term> = (<Term> ",")+, Term => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action123::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 24)
    }
    fn __reduce61<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Term> = (<Term> ",")+ => ActionFn(124);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action124::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 24)
    }
    fn __reduce62<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // String = r#"\"(\\\\\\\\|\\\\\"|[^\"\\\\])*\""# => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 25)
    }
    fn __reduce63<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = Logical => ActionFn(37);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action37::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 26)
    }
    fn __reduce64<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Term, ",", Term, ")" => ActionFn(108);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action108::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 26)
    }
    fn __reduce65<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "{", Term, "}" => ActionFn(39);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action39::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 26)
    }
    fn __reduce66<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "let", Reference, "=", Term, ";", Term => ActionFn(109);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action109::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 26)
    }
    fn __reduce67<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "if", "(", Term, ")", "{", Term, "}", "else", "{", Term, "}" => ActionFn(110);
        assert!(__symbols.len() >= 11);
        let __sym10 = __pop_Variant0(__symbols);
        let __sym9 = __pop_Variant4(__symbols);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym10.2;
        let __nt = super::__action110::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (11, 26)
    }
    fn __reduce68<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", Reference, ")", "=>", Term => ActionFn(125);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action125::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 26)
    }
    fn __reduce69<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", Reference, ")", "=>" => ActionFn(126);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action126::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 26)
    }
    fn __reduce70<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", ")", "=>", Term => ActionFn(127);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action127::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 26)
    }
    fn __reduce71<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", ")", "=>" => ActionFn(128);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action128::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 26)
    }
    fn __reduce72<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", (<Reference> ",")+, Reference, ")", "=>", Term => ActionFn(129);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action129::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 26)
    }
    fn __reduce73<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", (<Reference> ",")+, Reference, ")", "=>" => ActionFn(130);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action130::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 26)
    }
    fn __reduce74<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", (<Reference> ",")+, ")", "=>", Term => ActionFn(131);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action131::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 26)
    }
    fn __reduce75<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", (<Reference> ",")+, ")", "=>" => ActionFn(132);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action132::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 26)
    }
    fn __reduce76<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = error => ActionFn(112);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action112::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 26)
    }
    fn __reduce77<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term? = Term => ActionFn(49);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 27)
    }
    fn __reduce78<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term? =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action50::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 27)
    }
    fn __reduce79<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Text = "_" => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 28)
    }
    fn __reduce80<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Text = r#"[a-zA-Z][a-zA-Z0-9/_]*"# => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 28)
    }
    fn __reduce82<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // __Term = Term => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 30)
    }
}
#[allow(unused_imports)]
pub use self::__parse__File::FileParser;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::never_loop, clippy::match_single_binding, clippy::needless_raw_string_hashes)]
mod __parse__Term {

    use crate::ast::Element;
    use std::str::FromStr;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(__lalrpop_util::ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>),
        Variant2(crate::parser::Var),
        Variant3(alloc::vec::Vec<crate::parser::Var>),
        Variant4(crate::ast::Term),
        Variant5(alloc::vec::Vec<crate::ast::Term>),
        Variant6(usize),
        Variant7(crate::ast::BinaryOp),
        Variant8(crate::ast::File),
        Variant9(i32),
        Variant10(bool),
        Variant11(core::option::Option<crate::parser::Var>),
        Variant12(std::string::String),
        Variant13(Vec<crate::parser::Var>),
        Variant14(Vec<crate::ast::Term>),
        Variant15(core::option::Option<crate::ast::Term>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 1
        -26, 50, -26, 9, -26, 51, -26, -26, -26, 10, 52, -26, -26, -26, 0, -26, 0, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, 0, 0, 0, 0,
        // State 2
        53, 0, 54, 0, -35, 0, 0, -35, 0, 0, 0, -35, 55, 56, 0, 57, 0, 58, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, -35, 0, 0, 0, 0,
        // State 3
        -15, 0, -15, 0, -15, 0, 61, -15, 62, 0, 0, -15, -15, -15, 0, -15, 0, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0,
        // State 6
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 7
        0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 0, 0, 0, 43, 44, 45, 0, 0, 0, 46, 47, 48, 0,
        // State 8
        0, 0, 0, 5, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0,
        // State 10
        0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 0, 0, 0, 43, 44, 45, 0, 0, 0, 46, 47, 48, 0,
        // State 11
        0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 0, 0, 0, 43, 44, 45, 0, 0, 0, 46, 47, 48, 0,
        // State 12
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 13
        0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0,
        // State 14
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 15
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 16
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 17
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 18
        0, 0, 0, 5, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 19
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 20
        0, 0, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0,
        // State 21
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 22
        0, 0, 0, 5, -72, 0, 0, -72, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, -72, 46, 47, 48, 49,
        // State 23
        0, 0, 0, 5, -76, 0, 0, -76, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, -76, 46, 47, 48, 49,
        // State 24
        0, 0, 0, 5, -70, 0, 0, -70, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, -70, 46, 47, 48, 49,
        // State 25
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 26
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 27
        0, 0, 0, 5, -74, 0, 0, -74, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, -74, 46, 47, 48, 49,
        // State 28
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 39, 40, 41, 42, 6, 43, 44, 45, 7, 0, 0, 46, 47, 48, 49,
        // State 29
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, 0, -14, 0, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, 0, 0, 0, 0,
        // State 30
        -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, -50, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, -50, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, -64, 0, 0, -64, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0,
        // State 32
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, -13, 0, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, 0, 0, 0, 0,
        // State 33
        -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, -48, 0, 0, 0, 0,
        // State 34
        -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, -49, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, -51, 0, 0, 0, 0,
        // State 37
        -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, -80, 0, -80, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, -80, 0, 0, 0, 0,
        // State 38
        -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, -47, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, 0, 0, 0, 0,
        // State 45
        -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, 0, -63, 0, -63, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, -63, 0, 0, 0, 0,
        // State 46
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, -33, 0, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, 0, 0, 0, 0,
        // State 47
        -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, -81, 0, -81, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, -81, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, -77, 0, 0, -77, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0,
        // State 49
        0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, -31, -31, 0, 0, 0, -31, -31, -31, 0, 0, 0, -31, -31, -31, 0,
        // State 50
        0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, -29, -29, 0, 0, 0, -29, -29, -29, 0, 0, 0, -29, -29, -29, 0,
        // State 51
        0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, -30, -30, 0, 0, 0, -30, -30, -30, 0, 0, 0, -30, -30, -30, 0,
        // State 52
        0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, -40, -40, 0, 0, 0, -40, -40, -40, 0, 0, 0, -40, -40, -40, 0,
        // State 53
        0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, -37, -37, 0, 0, 0, -37, -37, -37, 0, 0, 0, -37, -37, -37, 0,
        // State 54
        0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, -43, -43, 0, 0, 0, -43, -43, -43, 0, 0, 0, -43, -43, -43, 0,
        // State 55
        0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, -41, -41, 0, 0, 0, -41, -41, -41, 0, 0, 0, -41, -41, -41, 0,
        // State 56
        0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, -39, -39, 0, 0, 0, -39, -39, -39, 0, 0, 0, -39, -39, -39, 0,
        // State 57
        0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, -44, -44, 0, 0, 0, -44, -44, -44, 0, 0, 0, -44, -44, -44, 0,
        // State 58
        0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, -42, -42, 0, 0, 0, -42, -42, -42, 0, 0, 0, -42, -42, -42, 0,
        // State 59
        0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, -38, -38, 0, 0, 0, -38, -38, -38, 0, 0, 0, -38, -38, -38, 0,
        // State 60
        0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, -17, -17, 0, 0, 0, -17, -17, -17, 0, 0, 0, -17, -17, -17, 0,
        // State 61
        0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, -18, -18, 0, 0, 0, -18, -18, -18, 0, 0, 0, -18, -18, -18, 0,
        // State 62
        0, 0, 0, 0, 72, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 0,
        // State 65
        -28, 0, -28, 0, -28, 0, -28, -28, -28, 0, 0, -28, -28, -28, 0, -28, 0, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 83, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, 0, 0, 0, 0,
        // State 68
        -27, 0, -27, 0, -27, 0, -27, -27, -27, 0, 0, -27, -27, -27, 0, -27, 0, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, -36, 0, 0, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0,
        // State 70
        -16, 0, -16, 0, -16, 0, 0, -16, 0, 0, 0, -16, -16, -16, 0, -16, 0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, 0, 0, 0, 0,
        // State 71
        -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 89, 0, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, -66, 0, 0, -66, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 95, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, 0, 0, 0, 0,
        // State 82
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, -22, 0, 0, 0, 0,
        // State 83
        0, 0, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, -9, -9, -9, -9, -9, -9, -9, -9, -9, 0, 0, -9, -9, -9, -9,
        // State 84
        0, 0, 0, 0, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 0, -20, 0, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 98, 0, 0, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, 0, -19, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0,
        // State 93
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 0, -21, 0, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, 0, 0, 0, 0,
        // State 94
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, 0, 0, 0, 0,
        // State 95
        0, 0, 0, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, -10, -10, -10, -10, -10, -10, -10, -10, -10, 0, 0, -10, -10, -10, -10,
        // State 96
        0, 0, 0, 0, -65, 0, 0, -65, 0, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, 0,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 99
        0, 0, 0, 0, -71, 0, 0, -71, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, 0,
        // State 100
        0, 0, 0, 0, -75, 0, 0, -75, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0,
        // State 101
        0, 0, 0, 0, -69, 0, 0, -69, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 106, 0, 0, 0, 0,
        // State 103
        0, 0, 0, 0, -67, 0, 0, -67, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, -73, 0, 0, -73, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0,
        // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 0, 0, 0, 0,
        // State 108
        0, 0, 0, 0, -68, 0, 0, -68, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 37 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -26,
        // State 2
        -35,
        // State 3
        -15,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -72,
        // State 23
        -76,
        // State 24
        -70,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -74,
        // State 28
        0,
        // State 29
        -14,
        // State 30
        -50,
        // State 31
        -64,
        // State 32
        -13,
        // State 33
        -48,
        // State 34
        -49,
        // State 35
        -83,
        // State 36
        -51,
        // State 37
        -80,
        // State 38
        -47,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        -46,
        // State 45
        -63,
        // State 46
        -33,
        // State 47
        -81,
        // State 48
        -77,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        -28,
        // State 66
        0,
        // State 67
        -23,
        // State 68
        -27,
        // State 69
        -36,
        // State 70
        -16,
        // State 71
        -45,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        -66,
        // State 79
        0,
        // State 80
        0,
        // State 81
        -25,
        // State 82
        -22,
        // State 83
        0,
        // State 84
        0,
        // State 85
        -20,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        0,
        // State 91
        0,
        // State 92
        -19,
        // State 93
        -21,
        // State 94
        -24,
        // State 95
        0,
        // State 96
        -65,
        // State 97
        0,
        // State 98
        0,
        // State 99
        -71,
        // State 100
        -75,
        // State 101
        -69,
        // State 102
        0,
        // State 103
        -67,
        // State 104
        -73,
        // State 105
        0,
        // State 106
        0,
        // State 107
        0,
        // State 108
        -68,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 20,
            5 => 18,
            8 => 1,
            9 => match state {
                11 => 70,
                _ => 2,
            },
            10 => 11,
            11 => 29,
            12 => match state {
                7 => 65,
                _ => 3,
            },
            13 => 7,
            15 => 30,
            17 => match state {
                10 => 69,
                _ => 31,
            },
            18 => 10,
            19 => 32,
            20 => match state {
                5 => 63,
                9 => 68,
                13 => 73,
                20 => 86,
                _ => 33,
            },
            25 => 34,
            26 => match state {
                4 => 62,
                6 => 64,
                8 => 66,
                12 => 72,
                14 => 75,
                15 => 76,
                16 => 77,
                17 => 79,
                18 => 80,
                19 => 84,
                21 => 91,
                22 => 99,
                23 => 100,
                24 => 101,
                25 => 102,
                26 => 103,
                27 => 104,
                28 => 107,
                _ => 35,
            },
            28 => 36,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""!=""###,
        r###""%""###,
        r###""&&""###,
        r###""(""###,
        r###"")""###,
        r###""*""###,
        r###""+""###,
        r###"",""###,
        r###""-""###,
        r###"".""###,
        r###""/""###,
        r###"";""###,
        r###""<""###,
        r###""<=""###,
        r###""=""###,
        r###""==""###,
        r###""=>""###,
        r###"">""###,
        r###"">=""###,
        r###""_""###,
        r###""else""###,
        r###""external""###,
        r###""false""###,
        r###""first""###,
        r###""fn""###,
        r###""if""###,
        r###""let""###,
        r###""print""###,
        r###""second""###,
        r###""true""###,
        r###""{""###,
        r###""||""###,
        r###""}""###,
        r###"r#"\"(\\\\\\\\|\\\\\"|[^\"\\\\])*\""#"###,
        r###"r#"[0123456789]+"#"###,
        r###"r#"[a-zA-Z][a-zA-Z0-9/_]*"#"###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'err,
        '__2,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    where
        'input: 'err,
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'err, '__2>
    where 'input: 'err
    {
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &'__2 str,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err, '__2> __state_machine::ParserDefinition for __StateMachine<'input, 'err, '__2>
    where 'input: 'err
    {
        type Location = usize;
        type Error = crate::parser::InnerError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = crate::ast::Term;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 37 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant1(recovery)
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.errors,
                self.filename,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(3, _) if true => Some(0),
            Token(4, _) if true => Some(1),
            Token(5, _) if true => Some(2),
            Token(6, _) if true => Some(3),
            Token(7, _) if true => Some(4),
            Token(8, _) if true => Some(5),
            Token(9, _) if true => Some(6),
            Token(10, _) if true => Some(7),
            Token(11, _) if true => Some(8),
            Token(12, _) if true => Some(9),
            Token(13, _) if true => Some(10),
            Token(14, _) if true => Some(11),
            Token(15, _) if true => Some(12),
            Token(16, _) if true => Some(13),
            Token(17, _) if true => Some(14),
            Token(18, _) if true => Some(15),
            Token(19, _) if true => Some(16),
            Token(20, _) if true => Some(17),
            Token(21, _) if true => Some(18),
            Token(22, _) if true => Some(19),
            Token(23, _) if true => Some(20),
            Token(24, _) if true => Some(21),
            Token(25, _) if true => Some(22),
            Token(26, _) if true => Some(23),
            Token(27, _) if true => Some(24),
            Token(28, _) if true => Some(25),
            Token(29, _) if true => Some(26),
            Token(30, _) if true => Some(27),
            Token(31, _) if true => Some(28),
            Token(32, _) if true => Some(29),
            Token(33, _) if true => Some(30),
            Token(34, _) if true => Some(31),
            Token(35, _) if true => Some(32),
            Token(0, _) if true => Some(33),
            Token(1, _) if true => Some(34),
            Token(2, _) if true => Some(35),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 => match __token {
                Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(30, __tok0) | Token(31, __tok0) | Token(32, __tok0) | Token(33, __tok0) | Token(34, __tok0) | Token(35, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
        '__2,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err, '__2>>
    where
        'input: 'err,
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 11,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 11,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 21,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 23,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 24,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 26,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 11,
                    nonterminal_produced: 26,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 26,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 26,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 26,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 26,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 26,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 27,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            82 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct TermParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for TermParser { fn default() -> Self { Self::new() } }
    impl TermParser {
        pub fn new() -> TermParser {
            let __builder = super::__intern_token::new_builder();
            TermParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
        >(
            &self,
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
            filename: &str,
            input: &'input str,
        ) -> Result<crate::ast::Term, __lalrpop_util::ParseError<usize, Token<'input>, crate::parser::InnerError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    errors,
                    filename,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
        '__2,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> bool
    where
        'input: 'err,
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<crate::ast::Term,__lalrpop_util::ParseError<usize, Token<'input>, crate::parser::InnerError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                __reduce54(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            55 => {
                __reduce55(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                __reduce56(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            57 => {
                __reduce57(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                __reduce70(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            71 => {
                __reduce71(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            72 => {
                __reduce72(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            73 => {
                __reduce73(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            74 => {
                __reduce74(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            75 => {
                __reduce75(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            76 => {
                __reduce76(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            77 => {
                __reduce77(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            78 => {
                __reduce78(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            79 => {
                __reduce79(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            80 => {
                __reduce80(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            81 => {
                __reduce81(errors, filename, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            82 => {
                // __Term = Term => ActionFn(1);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action1::<>(errors, filename, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<crate::ast::Term>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<crate::parser::Var>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<crate::ast::Term>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<crate::parser::Var>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<crate::ast::Term>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<crate::parser::Var>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, crate::ast::BinaryOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, crate::ast::File, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, crate::ast::Term, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, crate::parser::Var, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, std::string::String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",") = Reference, "," => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action62::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",")* =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",")* = (<Reference> ",")+ => ActionFn(61);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action61::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",")+ = Reference, "," => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action67::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Reference> ",")+ = (<Reference> ",")+, Reference, "," => ActionFn(68);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action68::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",") = Term, "," => ActionFn(57);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action57::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce6<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action55::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 4)
    }
    fn __reduce7<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")* = (<Term> ",")+ => ActionFn(56);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")+ = Term, "," => ActionFn(71);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action71::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 5)
    }
    fn __reduce9<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")+ = (<Term> ",")+, Term, "," => ActionFn(72);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action72::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 5)
    }
    fn __reduce10<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action54::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce11<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(53);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action53::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    fn __reduce12<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Apply = Primary => ActionFn(35);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action35::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    fn __reduce13<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Apply = Call => ActionFn(36);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    fn __reduce14<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arithmetic = Factor => ActionFn(23);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 9)
    }
    fn __reduce15<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Arithmetic = Factor, ArithmeticOp, Arithmetic => ActionFn(94);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action94::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 9)
    }
    fn __reduce16<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ArithmeticOp = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce17<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ArithmeticOp = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action22::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce18<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = "print", "(", Term, ")" => ActionFn(95);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action95::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce19<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = "first", "(", Term, ")" => ActionFn(96);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action96::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce20<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = "second", "(", Term, ")" => ActionFn(97);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action97::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce21<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = Apply, "(", Term, ")" => ActionFn(133);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action133::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce22<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = Apply, "(", ")" => ActionFn(134);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action134::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 11)
    }
    fn __reduce23<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = Apply, "(", (<Term> ",")+, Term, ")" => ActionFn(135);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action135::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 11)
    }
    fn __reduce24<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Call = Apply, "(", (<Term> ",")+, ")" => ActionFn(136);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action136::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 11)
    }
    fn __reduce25<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Factor = Apply => ActionFn(18);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action18::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 12)
    }
    fn __reduce26<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Factor = Apply, ".", Reference => ActionFn(99);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action99::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 12)
    }
    fn __reduce27<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Factor = Apply, FactorOp, Factor => ActionFn(100);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action100::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action15::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce29<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce30<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "%" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce31<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // File = Term => ActionFn(101);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action101::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    fn __reduce32<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Int = r#"[0123456789]+"# => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 15)
    }
    fn __reduce33<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // IsExternal = "external" => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 16)
    }
    fn __reduce34<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Logical = Arithmetic => ActionFn(33);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action33::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    fn __reduce35<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Logical = Arithmetic, LogicalOp, Logical => ActionFn(102);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action102::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 17)
    }
    fn __reduce36<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "&&" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce37<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "||" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce38<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "==" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action27::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce39<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "!=" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action28::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce40<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "<=" => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action29::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce41<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = ">=" => ActionFn(30);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce42<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "<" => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce43<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = ">" => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    fn __reduce44<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = "(", Term, ")" => ActionFn(5);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action5::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    fn __reduce45<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action103::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce46<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action104::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce47<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = Reference => ActionFn(8);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce48<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = String => ActionFn(105);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action105::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce49<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Primary = Int => ActionFn(106);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action106::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 19)
    }
    fn __reduce50<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Reference = Text => ActionFn(107);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action107::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 20)
    }
    fn __reduce51<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Reference? = Reference => ActionFn(58);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    fn __reduce52<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Reference? =  => ActionFn(59);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action59::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    fn __reduce53<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Semi = ";" => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 22)
    }
    fn __reduce54<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Reference> = Reference => ActionFn(113);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action113::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 23)
    }
    fn __reduce55<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Reference> =  => ActionFn(114);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action114::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 23)
    }
    fn __reduce56<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Reference> = (<Reference> ",")+, Reference => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action115::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 23)
    }
    fn __reduce57<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Reference> = (<Reference> ",")+ => ActionFn(116);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action116::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 23)
    }
    fn __reduce58<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Term> = Term => ActionFn(121);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action121::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 24)
    }
    fn __reduce59<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Term> =  => ActionFn(122);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action122::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 24)
    }
    fn __reduce60<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Term> = (<Term> ",")+, Term => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action123::<>(errors, filename, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 24)
    }
    fn __reduce61<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Sep<",", Term> = (<Term> ",")+ => ActionFn(124);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action124::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 24)
    }
    fn __reduce62<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // String = r#"\"(\\\\\\\\|\\\\\"|[^\"\\\\])*\""# => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 25)
    }
    fn __reduce63<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = Logical => ActionFn(37);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action37::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 26)
    }
    fn __reduce64<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Term, ",", Term, ")" => ActionFn(108);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action108::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 26)
    }
    fn __reduce65<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "{", Term, "}" => ActionFn(39);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action39::<>(errors, filename, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 26)
    }
    fn __reduce66<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "let", Reference, "=", Term, ";", Term => ActionFn(109);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action109::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 26)
    }
    fn __reduce67<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "if", "(", Term, ")", "{", Term, "}", "else", "{", Term, "}" => ActionFn(110);
        assert!(__symbols.len() >= 11);
        let __sym10 = __pop_Variant0(__symbols);
        let __sym9 = __pop_Variant4(__symbols);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym10.2;
        let __nt = super::__action110::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (11, 26)
    }
    fn __reduce68<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", Reference, ")", "=>", Term => ActionFn(125);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action125::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 26)
    }
    fn __reduce69<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", Reference, ")", "=>" => ActionFn(126);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action126::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 26)
    }
    fn __reduce70<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", ")", "=>", Term => ActionFn(127);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action127::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 26)
    }
    fn __reduce71<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", ")", "=>" => ActionFn(128);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action128::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 26)
    }
    fn __reduce72<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", (<Reference> ",")+, Reference, ")", "=>", Term => ActionFn(129);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action129::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 26)
    }
    fn __reduce73<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", (<Reference> ",")+, Reference, ")", "=>" => ActionFn(130);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action130::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 26)
    }
    fn __reduce74<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", (<Reference> ",")+, ")", "=>", Term => ActionFn(131);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action131::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 26)
    }
    fn __reduce75<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "fn", "(", (<Reference> ",")+, ")", "=>" => ActionFn(132);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action132::<>(errors, filename, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 26)
    }
    fn __reduce76<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = error => ActionFn(112);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action112::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 26)
    }
    fn __reduce77<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term? = Term => ActionFn(49);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 27)
    }
    fn __reduce78<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term? =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action50::<>(errors, filename, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 27)
    }
    fn __reduce79<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Text = "_" => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 28)
    }
    fn __reduce80<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Text = r#"[a-zA-Z][a-zA-Z0-9/_]*"# => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 28)
    }
    fn __reduce81<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
        filename: &str,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(errors, filename, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Term::TermParser;
#[rustfmt::skip]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::ast::Element;
    use std::str::FromStr;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("(?:\"((?:(?:\\\\\\\\)|(?:\\\\\")|[\0-!\\#-\\[\\]-\u{10ffff}]))*\")", false),
            ("[0-9]+", false),
            ("(?:[A-Za-z][/-9A-Z_a-z]*)", false),
            ("(?:!=)", false),
            ("%", false),
            ("(?:\\&\\&)", false),
            ("\\(", false),
            ("\\)", false),
            ("\\*", false),
            ("\\+", false),
            (",", false),
            ("\\-", false),
            ("\\.", false),
            ("/", false),
            (";", false),
            ("<", false),
            ("(?:<=)", false),
            ("=", false),
            ("(?:==)", false),
            ("(?:=>)", false),
            (">", false),
            ("(?:>=)", false),
            ("_", false),
            ("(?:else)", false),
            ("(?:external)", false),
            ("(?:false)", false),
            ("(?:first)", false),
            ("(?:fn)", false),
            ("(?:if)", false),
            ("(?:let)", false),
            ("(?:print)", false),
            ("(?:second)", false),
            ("(?:true)", false),
            ("\\{", false),
            ("(?:\\|\\|)", false),
            ("\\}", false),
            ("(?:(?://)[\0-\t\u{b}\u{c}\u{e}-\u{10ffff}]*[\n\r]*)", true),
            ("(?:(?:/\\*)[\0-\\)\\+-\u{10ffff}]*\\*+(?:[\0-\\)\\+-\\.0-\u{10ffff}][\0-\\)\\+-\u{10ffff}]*\\*+)*/)", true),
            ("[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]*", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action0<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::File, usize),
) -> crate::ast::File
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action1<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action2<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, expression, _): (usize, crate::ast::Term, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::File
{
    crate::ast::File {
    name: filename.to_string(),
    expression,
    location: crate::ast::Location::new(s, e, filename),
  }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action3<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    true
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action4<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> std::string::String
{
    ";".to_string()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action5<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> crate::ast::Term
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action6<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Bool(crate::ast::Bool {
    value: true,
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action7<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Bool(crate::ast::Bool {
    value: false,
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action8<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::parser::Var, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Var(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action9<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, value, _): (usize, std::string::String, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Str(crate::ast::Str {
    value,
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action10<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, value, _): (usize, i32, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Int(crate::ast::Int {
    value,
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action11<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, value, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Print(crate::ast::Print {
    value: Box::new(value),
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action12<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, value, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::First(crate::ast::First {
    value: Box::new(value),
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action13<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, value, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Second(crate::ast::Second {
    value: Box::new(value),
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action14<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, callee, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, arguments, _): (usize, Vec<crate::ast::Term>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Call(crate::ast::Call {
    callee: Box::new(callee),
    arguments,
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action15<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Mul
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action16<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Div
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action17<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Rem
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action18<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action19<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, value, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, crate::parser::Var, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    {
    // Report the error
    errors.push(lalrpop_util::ErrorRecovery {
        dropped_tokens: vec![],
        error: lalrpop_util::ParseError::User {
            error: crate::parser::InnerError::UnsupportedRecordIndex {
                err_span: name.location.into(),
            }
        },
    });

    crate::ast::Term::Error(crate::ast::Error {
      message: format!("unsupported record indexing `{}`", name.text),
      full_text: (&input[s..e]).to_string(),
      location: crate::ast::Location::new(s, e, filename),
    })
  }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action20<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, a, _): (usize, crate::ast::Term, usize),
    (_, op, _): (usize, crate::ast::BinaryOp, usize),
    (_, b, _): (usize, crate::ast::Term, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Binary(crate::ast::Binary {
    location: crate::ast::Location::new(s, e, filename),
    op,
    lhs: a.into(),
    rhs: b.into(),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action21<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Add
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action22<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Sub
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action23<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action24<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, a, _): (usize, crate::ast::Term, usize),
    (_, op, _): (usize, crate::ast::BinaryOp, usize),
    (_, b, _): (usize, crate::ast::Term, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Binary(crate::ast::Binary {
    location: crate::ast::Location::new(s, e, filename),
    op,
    lhs: a.into(),
    rhs: b.into(),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action25<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::And
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action26<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Or
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action27<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Eq
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action28<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Neq
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action29<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Lte
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action30<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Gte
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action31<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Lt
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action32<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> crate::ast::BinaryOp
{
    crate::ast::BinaryOp::Gt
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action33<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action34<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, a, _): (usize, crate::ast::Term, usize),
    (_, op, _): (usize, crate::ast::BinaryOp, usize),
    (_, b, _): (usize, crate::ast::Term, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Binary(crate::ast::Binary {
    location: crate::ast::Location::new(s, e, filename),
    op,
    lhs: a.into(),
    rhs: b.into(),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action35<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action36<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action37<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action38<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, first, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, second, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Tuple(crate::ast::Tuple {
    first: Box::new(first),
    second: Box::new(second),
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action39<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, term, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> crate::ast::Term
{
    term
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action40<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, crate::parser::Var, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, value, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, next, _): (usize, crate::ast::Term, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Let(crate::ast::Let {
    name,
    value: value.into(),
    next: next.into(),
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action41<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, condition, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, then, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, otherwise, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::If(crate::ast::If {
    condition: condition.into(),
    then: then.into(),
    otherwise: otherwise.into(),
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action42<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, parameters, _): (usize, Vec<crate::parser::Var>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, core::option::Option<crate::ast::Term>, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    crate::ast::Term::Function(crate::ast::Function {
    parameters,
    value: match body {
      Some(value) => Box::new(value),
      None => {
        // Report the error
        errors.push(lalrpop_util::ErrorRecovery {
            dropped_tokens: vec![],
            error: lalrpop_util::ParseError::User {
                error: crate::parser::InnerError::FunctionBodyMissing {
                    err_span: crate::ast::Location::new(s, e, filename).into(),
                }
            },
        });

        crate::ast::Term::Int(crate::ast::Int { value: 0, location: crate::ast::Location::new(s, e, filename).into() }).into()
      },
    },
    location: crate::ast::Location::new(s, e, filename),
  })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action43<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, error, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::ast::Term
{
    {
    let message = error.error.to_string();

    errors.push(error);

    crate::ast::Term::Error(crate::ast::Error {
      message,
      full_text: (&input[s..e]).to_string(),
      location: crate::ast::Location::new(s, e, filename),
    })
  }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action44<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(s).unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action45<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, text, _): (usize, &'input str, usize),
) -> std::string::String
{
    (&text[1..text.len() - 1]).to_string()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action46<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, text, _): (usize, &'input str, usize),
) -> std::string::String
{
    text.to_string()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action47<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, text, _): (usize, &'input str, usize),
) -> std::string::String
{
    text.to_string()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action48<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, s, _): (usize, usize, usize),
    (_, text, _): (usize, std::string::String, usize),
    (_, e, _): (usize, usize, usize),
) -> crate::parser::Var
{
    crate::parser::Var {
    text: text.to_string(),
    location: crate::ast::Location::new(s, e, filename),
  }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action49<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
) -> core::option::Option<crate::ast::Term>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action50<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<crate::ast::Term>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action51<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    (_, e, _): (usize, core::option::Option<crate::parser::Var>, usize),
) -> Vec<crate::parser::Var>
{
    match e {
    Some(e) => {
        v.push(e);
        v
    }
    None => v
  }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action52<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<crate::ast::Term>, usize),
    (_, e, _): (usize, core::option::Option<crate::ast::Term>, usize),
) -> Vec<crate::ast::Term>
{
    match e {
    Some(e) => {
        v.push(e);
        v
    }
    None => v
  }
}

#[allow(unused_variables)]
fn __action53<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookbehind
}

#[allow(unused_variables)]
fn __action54<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookahead
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action55<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<crate::ast::Term>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action56<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<crate::ast::Term>, usize),
) -> alloc::vec::Vec<crate::ast::Term>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action57<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> crate::ast::Term
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action58<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::parser::Var, usize),
) -> core::option::Option<crate::parser::Var>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action59<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<crate::parser::Var>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action60<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<crate::parser::Var>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action61<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<crate::parser::Var>, usize),
) -> alloc::vec::Vec<crate::parser::Var>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action62<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::parser::Var, usize),
    (_, _, _): (usize, &'input str, usize),
) -> crate::parser::Var
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action63<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::parser::Var, usize),
) -> alloc::vec::Vec<crate::parser::Var>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action64<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    (_, e, _): (usize, crate::parser::Var, usize),
) -> alloc::vec::Vec<crate::parser::Var>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action65<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, __0, _): (usize, crate::ast::Term, usize),
) -> alloc::vec::Vec<crate::ast::Term>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action66<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<crate::ast::Term>, usize),
    (_, e, _): (usize, crate::ast::Term, usize),
) -> alloc::vec::Vec<crate::ast::Term>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action67<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::parser::Var, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<crate::parser::Var>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action62(
        errors,
        filename,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        errors,
        filename,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action68<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    __1: (usize, crate::parser::Var, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<crate::parser::Var>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action62(
        errors,
        filename,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action69<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, core::option::Option<crate::parser::Var>, usize),
) -> Vec<crate::parser::Var>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action60(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        errors,
        filename,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action70<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    __1: (usize, core::option::Option<crate::parser::Var>, usize),
) -> Vec<crate::parser::Var>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action61(
        errors,
        filename,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        errors,
        filename,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action71<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<crate::ast::Term>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action57(
        errors,
        filename,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        errors,
        filename,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action72<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<crate::ast::Term>, usize),
    __1: (usize, crate::ast::Term, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<crate::ast::Term>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action57(
        errors,
        filename,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action73<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, core::option::Option<crate::ast::Term>, usize),
) -> Vec<crate::ast::Term>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action55(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        errors,
        filename,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action74<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<crate::ast::Term>, usize),
    __1: (usize, core::option::Option<crate::ast::Term>, usize),
) -> Vec<crate::ast::Term>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action56(
        errors,
        filename,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        errors,
        filename,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action75<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, crate::ast::BinaryOp, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action76<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action77<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action78<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action79<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<crate::ast::Term>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action80<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::parser::Var, usize),
    __3: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action81<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, crate::ast::BinaryOp, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action82<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, usize, usize),
) -> crate::ast::File
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action83<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, crate::ast::BinaryOp, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action84<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action85<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action86<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, std::string::String, usize),
    __1: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action87<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, i32, usize),
    __1: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action88<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, std::string::String, usize),
    __1: (usize, usize, usize),
) -> crate::parser::Var
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action89<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, crate::ast::Term, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, crate::ast::Term, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action90<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, crate::parser::Var, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, crate::ast::Term, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, crate::ast::Term, usize),
    __6: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action91<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, crate::ast::Term, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, &'input str, usize),
    __8: (usize, &'input str, usize),
    __9: (usize, crate::ast::Term, usize),
    __10: (usize, &'input str, usize),
    __11: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
        __9,
        __10,
        __11,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action92<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<crate::parser::Var>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, core::option::Option<crate::ast::Term>, usize),
    __6: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action93<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>, usize),
    __1: (usize, usize, usize),
) -> crate::ast::Term
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action54(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        errors,
        filename,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action94<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, crate::ast::BinaryOp, usize),
    __2: (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    let __start0 = __2.2;
    let __end0 = __2.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action95<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action96<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action97<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action98<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<crate::ast::Term>, usize),
    __3: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action99<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::parser::Var, usize),
) -> crate::ast::Term
{
    let __start0 = __2.2;
    let __end0 = __2.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action100<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, crate::ast::BinaryOp, usize),
    __2: (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    let __start0 = __2.2;
    let __end0 = __2.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action101<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
) -> crate::ast::File
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action102<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, crate::ast::BinaryOp, usize),
    __2: (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    let __start0 = __2.2;
    let __end0 = __2.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action103<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action104<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action105<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, std::string::String, usize),
) -> crate::ast::Term
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action106<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, i32, usize),
) -> crate::ast::Term
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action107<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, std::string::String, usize),
) -> crate::parser::Var
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action88(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action108<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, crate::ast::Term, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, crate::ast::Term, usize),
    __4: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action109<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, crate::parser::Var, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, crate::ast::Term, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    let __start0 = __5.2;
    let __end0 = __5.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action110<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, crate::ast::Term, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, &'input str, usize),
    __8: (usize, &'input str, usize),
    __9: (usize, crate::ast::Term, usize),
    __10: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __10.2;
    let __end0 = __10.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
        __9,
        __10,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action111<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<crate::parser::Var>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, core::option::Option<crate::ast::Term>, usize),
) -> crate::ast::Term
{
    let __start0 = __5.2;
    let __end0 = __5.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action92(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action112<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>, usize),
) -> crate::ast::Term
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action53(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action93(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action113<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::parser::Var, usize),
) -> Vec<crate::parser::Var>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action58(
        errors,
        filename,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        errors,
        filename,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action114<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<crate::parser::Var>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action59(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        errors,
        filename,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action115<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    __1: (usize, crate::parser::Var, usize),
) -> Vec<crate::parser::Var>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action58(
        errors,
        filename,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action116<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
) -> Vec<crate::parser::Var>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action59(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action117<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::parser::Var, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, core::option::Option<crate::ast::Term>, usize),
) -> crate::ast::Term
{
    let __start0 = __2.0;
    let __end0 = __2.2;
    let __temp0 = __action113(
        errors,
        filename,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        errors,
        filename,
        input,
        __0,
        __1,
        __temp0,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action118<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, core::option::Option<crate::ast::Term>, usize),
) -> crate::ast::Term
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action114(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        errors,
        filename,
        input,
        __0,
        __1,
        __temp0,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action119<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    __3: (usize, crate::parser::Var, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, core::option::Option<crate::ast::Term>, usize),
) -> crate::ast::Term
{
    let __start0 = __2.0;
    let __end0 = __3.2;
    let __temp0 = __action115(
        errors,
        filename,
        input,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        errors,
        filename,
        input,
        __0,
        __1,
        __temp0,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action120<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, core::option::Option<crate::ast::Term>, usize),
) -> crate::ast::Term
{
    let __start0 = __2.0;
    let __end0 = __2.2;
    let __temp0 = __action116(
        errors,
        filename,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        errors,
        filename,
        input,
        __0,
        __1,
        __temp0,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action121<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
) -> Vec<crate::ast::Term>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action49(
        errors,
        filename,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        errors,
        filename,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action122<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<crate::ast::Term>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action50(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        errors,
        filename,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action123<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<crate::ast::Term>, usize),
    __1: (usize, crate::ast::Term, usize),
) -> Vec<crate::ast::Term>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action49(
        errors,
        filename,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action124<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<crate::ast::Term>, usize),
) -> Vec<crate::ast::Term>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action50(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        errors,
        filename,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action125<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::parser::Var, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    let __start0 = __5.0;
    let __end0 = __5.2;
    let __temp0 = __action49(
        errors,
        filename,
        input,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action117(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action126<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::parser::Var, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action50(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action117(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action127<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    let __start0 = __4.0;
    let __end0 = __4.2;
    let __temp0 = __action49(
        errors,
        filename,
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action118(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action128<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action50(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action118(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action129<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    __3: (usize, crate::parser::Var, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    let __start0 = __6.0;
    let __end0 = __6.2;
    let __temp0 = __action49(
        errors,
        filename,
        input,
        __6,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action119(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action130<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    __3: (usize, crate::parser::Var, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __5.2;
    let __end0 = __5.2;
    let __temp0 = __action50(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action119(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action131<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, crate::ast::Term, usize),
) -> crate::ast::Term
{
    let __start0 = __5.0;
    let __end0 = __5.2;
    let __temp0 = __action49(
        errors,
        filename,
        input,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action120(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action132<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, alloc::vec::Vec<crate::parser::Var>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action50(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action120(
        errors,
        filename,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action133<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, crate::ast::Term, usize),
    __3: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __2.0;
    let __end0 = __2.2;
    let __temp0 = __action121(
        errors,
        filename,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action98(
        errors,
        filename,
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action134<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action122(
        errors,
        filename,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action98(
        errors,
        filename,
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action135<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, alloc::vec::Vec<crate::ast::Term>, usize),
    __3: (usize, crate::ast::Term, usize),
    __4: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __2.0;
    let __end0 = __3.2;
    let __temp0 = __action123(
        errors,
        filename,
        input,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action98(
        errors,
        filename,
        input,
        __0,
        __1,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action136<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, crate::parser::InnerError>>,
    filename: &str,
    input: &'input str,
    __0: (usize, crate::ast::Term, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, alloc::vec::Vec<crate::ast::Term>, usize),
    __3: (usize, &'input str, usize),
) -> crate::ast::Term
{
    let __start0 = __2.0;
    let __end0 = __2.2;
    let __temp0 = __action124(
        errors,
        filename,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action98(
        errors,
        filename,
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}
#[allow(clippy::type_complexity, dead_code)]

pub  trait __ToTriple<'input, 'err, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, crate::parser::InnerError>>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, crate::parser::InnerError>> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, Token<'input>, usize), crate::parser::InnerError>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, crate::parser::InnerError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
