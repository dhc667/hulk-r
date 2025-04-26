use std::collections::HashSet;

pub struct DefContext {
    pub frames: Vec<Box<Frame>>,
    pub current: usize,
}

pub struct Frame {
    pub variables: HashSet<String>,
    pub parent: Option<usize>,
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
    pub fn new() -> Self {
        DefContext {
            frames: vec![Box::new(Frame::new())],
            current: 0,
        }
    }

    fn current_frame(&self) -> &Frame {
        &self.frames[self.current]
    }

    fn current_frame_mut(&mut self) -> &mut Frame {
        &mut self.frames[self.current]
    }

    pub fn push_frame(&mut self) {
        let new_frame = Box::new(Frame {
            variables: HashSet::new(),
            parent: Some(self.current),
        });

        self.current = self.frames.len();
        self.frames.push(new_frame);
    }

    pub fn pop_frame(&mut self) {
        self.current = match self.current_frame().parent {
            Some(parent) => parent,
            None => panic!("Fatal Error: No parent frame to pop to. This should not happened."),
        };
    }

    pub fn get_context(&self, var_name: &str) -> Option<usize> {
        let mut current_frame = self.current_frame();
        let mut index = self.current;
        while let Some(parent_index) = current_frame.parent {
            if current_frame.variables.contains(var_name) {
                return Some(index);
            }
            current_frame = &self.frames[parent_index];
            index = parent_index;
        }
        None
    }

    pub fn define(&mut self, var_name: &str) -> Option<usize> {
        if self
            .current_frame_mut()
            .variables
            .insert(var_name.to_string())
        {
            Some(self.current)
        } else {
            None
        }
    }
}
