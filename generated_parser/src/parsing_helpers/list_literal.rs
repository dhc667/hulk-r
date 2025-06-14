use ast::ListLiteral;

use crate::types::ReturnType;

// ListLiteral -> OpenBracket ArgumentList CloseBracket
pub(crate) fn list_literal_non_empty(mut v: Vec<ReturnType>) -> ReturnType {
    let right_brack = v.pop().unwrap().try_into_grouping_operator().unwrap();
    let exp_list = v.pop().unwrap().try_into_argument_list().unwrap();
    let left_brack = v.pop().unwrap().try_into_grouping_operator().unwrap();

    ReturnType::ListLiteral(ListLiteral::new(left_brack, right_brack, exp_list))
}

// ListLiteral -> OpenBracket CloseBracket
pub(crate) fn list_literal_empty(mut v: Vec<ReturnType>) -> ReturnType {

    let right_brack = v.pop().unwrap().try_into_grouping_operator().unwrap();
    let exp_list = Vec::new();
    let left_brack = v.pop().unwrap().try_into_grouping_operator().unwrap();

    ReturnType::ListLiteral(ListLiteral::new(left_brack, right_brack, exp_list))
} 
