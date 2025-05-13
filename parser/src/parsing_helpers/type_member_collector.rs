use ast::{DataMemberDef, FunctionDef};

pub struct TypeMemberCollector {
    pub data_member_defs: Vec<DataMemberDef>,
    pub function_member_defs: Vec<FunctionDef>,
}

impl TypeMemberCollector {
    pub fn new() -> Self {
        Self {
            data_member_defs: Vec::new(),
            function_member_defs: Vec::new(),
        }
    }

    pub fn add_data_member_def(&mut self, d: DataMemberDef) {
        self.data_member_defs.push(d);
    }

    pub fn add_function_member_def(&mut self, f: FunctionDef) {
        self.function_member_defs.push(f);
    }

    pub fn add_member_def(&mut self, d: MemberDef) {
        match d {
            MemberDef::DataMemberDef(d) => {
                self.add_data_member_def(d);
            }
            MemberDef::FunctionDef(f) => {
                self.add_function_member_def(f);
            }
        }
    }
}

pub enum MemberDef {
    FunctionDef(FunctionDef),
    DataMemberDef(DataMemberDef),
}

impl From<DataMemberDef> for MemberDef {
    fn from(v: DataMemberDef) -> Self {
        Self::DataMemberDef(v)
    }
}

impl From<FunctionDef> for MemberDef {
    fn from(v: FunctionDef) -> Self {
        Self::FunctionDef(v)
    }
}
