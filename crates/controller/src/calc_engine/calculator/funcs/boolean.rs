use crate::calc_engine::calculator::calc_vertex::CalcVertex;
use logisheets_parser::ast;

pub fn calc_true(args: Vec<CalcVertex>) -> CalcVertex {
    if args.len() > 0 {
        return CalcVertex::from_error(ast::Error::Unspecified);
    }
    CalcVertex::from_bool(true)
}

pub fn calc_false(args: Vec<CalcVertex>) -> CalcVertex {
    if args.len() > 0 {
        return CalcVertex::from_error(ast::Error::Unspecified);
    }
    CalcVertex::from_bool(false)
}
