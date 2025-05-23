pub enum LlvmType {
    F64,
    // this is expected to grow
}


pub enum HandleType {
    Literal(LlvmType),
    Register(LlvmType),
}

impl HandleType {
    pub fn literal_f64() -> HandleType {
        HandleType::Literal(LlvmType::F64)
    }
    pub fn register_f64() -> HandleType {
        HandleType::Register(LlvmType::F64)
    }
}

/// # Description
///
/// LlvmHandle encapsulates the information needed to use temporary variables
/// and literals in llvm:
///
/// - The `handle` member will contain a name such as %1 or 2.0000, which are
/// the results of operations, or literals put directly into source code
/// - The `handle_type` member tells if this is a literal, or a register
pub struct LlvmHandle {
    pub handle_type: HandleType,
    pub llvm_name: String,
}

impl LlvmHandle {
    pub fn new(handle_type: HandleType, handle: String) -> LlvmHandle {
        LlvmHandle {
            handle_type,
            llvm_name: handle,
        }
    }

    pub fn new_f64_literal(value: f64) -> LlvmHandle {
        let mut s = format!("{:.}", value);
        if !s.contains('.') {
            s.push_str(".0");
        }
        LlvmHandle::new(HandleType::literal_f64(), s)
    }

    pub fn new_tmp_register(name: String) -> LlvmHandle {
        LlvmHandle::new(HandleType::register_f64(), name)
    }
}
