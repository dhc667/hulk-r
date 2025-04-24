use std::collections::HashSet;


pub struct DefContext {
    pub current_frame: Link,
}

type Link = Option<Box<Frame>>;

pub struct Frame {
    pub variables: HashSet<String>,
    pub parent: Link,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            variables: HashSet::new(),
            parent: None,
        }
    }
}

impl DefContext {
    pub fn push_frame(&mut self) {
        let new_frame = Box::new(Frame {
            variables: HashSet::new(),
            parent: self.current_frame.take(),
        });

        self.current_frame = Some(new_frame)
    }

    pub fn pop_frame(&mut self) {
        self.current_frame.take().map(|frame| {
            self.current_frame = frame.parent;
        });
    }

    pub fn is_defined(&self, var_name: &str) -> bool {
        let mut current_frame = &self.current_frame;
        while let Some(frame) = current_frame {
            if frame.variables.contains(var_name) {
                return true;
            }
            current_frame = &frame.parent;
        }
        false
    }

    pub fn define(&mut self, var_name: &str) -> bool {
        if let Some(frame) = &mut self.current_frame {
            return frame.variables.insert(var_name.to_string());
        }
        false
    }
}
