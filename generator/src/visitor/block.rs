use ast::{BlockBodyItem, VisitableExpression};

use super::{GeneratorVisitor, VisitorResult};

impl GeneratorVisitor {
    pub(crate) fn handle_block_items(&mut self, body_items: &mut Vec<BlockBodyItem>, multiple_semicolon_terminated: bool) -> VisitorResult {
        let mut preamble = "".to_string();
        let mut result_handle = None;

        for exp in body_items {
            let result = exp.accept(self);
            preamble = preamble + "\n" + &result.preamble;

            result_handle = result.result_handle;
        }

        VisitorResult {
            preamble,
            result_handle: if multiple_semicolon_terminated {
                None
            } else {
                result_handle
            },
        }
    }
}
