#[derive(Clone, Debug)]
pub enum LlvmType {
    F64,
    I1,
    String, // Now represents %string_type*
    Object,
    List(Box<LlvmType>),
}

impl LlvmType {
    pub fn llvm_type_str(&self) -> String {
        match self {
            LlvmType::F64 => "double".to_string(),
            LlvmType::I1 => "i1".to_string(),
            LlvmType::String => "i8*".to_string(),
            LlvmType::Object => "i8*".to_string(),
            LlvmType::List(inner) => format!("{}*", inner.llvm_type_str()),
        }
    }
}

#[derive(Clone)]
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
    pub fn literal_i1() -> HandleType {
        HandleType::Literal(LlvmType::I1)
    }
    pub fn register_i1() -> HandleType {
        HandleType::Register(LlvmType::I1)
    }
    pub fn literal_string() -> HandleType {
        HandleType::Literal(LlvmType::String)
    }
    pub fn register_string() -> HandleType {
        HandleType::Register(LlvmType::String)
    }
    pub fn literal_object() -> HandleType {
        HandleType::Literal(LlvmType::Object)
    }
    pub fn register_object() -> HandleType {
        HandleType::Register(LlvmType::Object)
    }
    pub fn literal_list(inner: LlvmType) -> HandleType {
        HandleType::Literal(LlvmType::List(Box::new(inner)))
    }
    pub fn register_list(inner: LlvmType) -> HandleType {
        HandleType::Register(LlvmType::List(Box::new(inner)))
    }


    pub fn inner_type(&self) -> LlvmType {
        match self {
            HandleType::Register(t) | HandleType::Literal(t) => t.clone(),
        }
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

pub const TRUE_LITERAL_STR: &str = "true";
pub const FALSE_LITERAL_STR: &str = "false";

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

    pub fn new_i1_literal(value: bool) -> LlvmHandle {
        let llvm_value = if value {
            TRUE_LITERAL_STR.to_string()
        } else {
            FALSE_LITERAL_STR.to_string()
        };
        LlvmHandle::new(HandleType::literal_i1(), llvm_value.to_string())
    }

    pub fn new_string_literal(value: String) -> LlvmHandle {
        LlvmHandle::new(HandleType::literal_string(), value)
    }

    pub fn new_object_literal(value: String) -> LlvmHandle {
        LlvmHandle::new(HandleType::literal_object(), value)
    }

    pub fn new_f64_register(name: String) -> LlvmHandle {
        LlvmHandle::new(HandleType::register_f64(), name)
    }

    pub fn new_i1_register(name: String) -> LlvmHandle {
        LlvmHandle::new(HandleType::register_i1(), name)
    }

    pub fn new_string_register(name: String) -> LlvmHandle {
        LlvmHandle::new(HandleType::register_string(), name)
    }

    pub fn new_object_register(name: String) -> LlvmHandle {
        LlvmHandle::new(HandleType::register_object(), name)
    }

    pub fn new_list_literal(inner: LlvmType, value: String) -> LlvmHandle {
        LlvmHandle::new(HandleType::literal_list(inner), value)
    }

    pub fn new_list_register(inner: LlvmType, name: String) -> LlvmHandle {
        LlvmHandle::new(HandleType::register_list(inner), name)
    }
}
