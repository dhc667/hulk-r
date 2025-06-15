use std::collections::HashMap;

use crate::SymbolId;

pub(crate) fn get_name_or_default(
    s: &SymbolId,
    symbol_names: &HashMap<SymbolId, Option<String>>,
) -> String {
    symbol_names
        .get(s)
        .unwrap()
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or(s.id_string())
}

pub fn get_production_string(
    production: &(SymbolId, Vec<SymbolId>),
    symbol_names: &HashMap<SymbolId, Option<String>>,
) -> (String, String) {
    let production_rhs: Vec<String> = production
        .1
        .iter()
        .map(|s| get_name_or_default(s, symbol_names))
        .collect();

    let production_rhs = production_rhs.join(" ");

    let production_lhs_symbol = production.0;
    let production_lhs_name = get_name_or_default(&production_lhs_symbol, symbol_names);

    (production_lhs_name, production_rhs)
}
