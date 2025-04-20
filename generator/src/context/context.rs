use std::collections::HashMap;


struct Frame<T> {
    can_access_parents: bool,
    values: HashMap<String, T>,
}

impl<T> Frame<T> {
    pub fn new(can_access_parents: bool) -> Frame<T> {
        Frame {
            can_access_parents,
            values: HashMap::new(),
        }
    }
}

pub struct Context<R> {
    scope_stack: Vec<Frame<R>>,
}

impl<R> Context<R> {
    pub fn new_empty() -> Context<R> {
        Context {
            scope_stack: Vec::new(),
        }
    }

    pub fn new_one_frame() -> Context<R> {
        let mut ctx = Context::new_empty();
        ctx.push_frame(false);

        ctx
    }

    pub fn push_frame(&mut self, can_access_parents: bool) {
        let mut can_access_parents = can_access_parents;

        if self.scope_stack.len() == 0 {
            can_access_parents = false;
        }

        self.scope_stack.push(Frame::new(can_access_parents));
    }

    pub fn pop_frame(&mut self) {
        self.scope_stack.pop().expect("Popping empty context");
    }

    pub fn define(&mut self, id: String, value: R) {
        self.scope_stack
            .last_mut()
            .expect("Attempting to define on empty context")
            .values
            .insert(id, value);
    }

    pub fn is_defined(&self, id: &str) -> bool {
        self.get_value(id).is_some()
    }

    pub fn get_value(&self, id: &str) -> Option<&R> {
        let mut stack_iter = self.scope_stack.iter().rev();
        let mut current = stack_iter
            .next()
            .expect("Attempting to get value on empty context");

        let mut last_value = current.values.get(id);

        while matches!(last_value, None) && current.can_access_parents {
            current = stack_iter.next()
                .expect("Attempting to access the parent of global scope (can_access_parents should be false)");

            last_value = current.values.get(id);
        }

        last_value
    }

    pub fn get_value_mut(&mut self, id: &str) -> Option<&mut R> {
        let mut stack_iter = self.scope_stack.iter_mut().rev();
        let mut current = stack_iter
            .next()
            .expect("Attempting to get value on empty context");

        let mut last_value = current.values.get_mut(id);

        while matches!(last_value, None) && current.can_access_parents {
            current = stack_iter.next()
                .expect("Attempting to access the parent of global scope (can_access_parents should be false)");

            last_value = current.values.get_mut(id);
        }

        last_value
    }
}
