
/// A simple data structure whose job is to keep track of the state of function calls
pub struct CallStack {
    c_stack: Vec<CallFrame>
}

struct CallFrame {
    name: String,
    length: usize,
    capacity: usize, // Only used in `dump()` to calculate which index of instruction is current
}

impl CallStack {
    /// Create a new empty call stack
    pub fn new() -> Self {
        Self { c_stack: vec![] }
    }

    /// Add a new function to the top of the stack
    /// `name`: Name of the new function
    /// `length`: Amount of instructions in the new function
    pub fn push(&mut self, name: String, length: usize) {
        self.c_stack.push(CallFrame { name, length, capacity: length });
    }

    /// To be called before an instruction is run.
    /// `step()` and `propagate()` are separated so that calls to `dump()` are accurate.
    pub fn step(&mut self) {
        self.c_stack.last_mut().unwrap().length -= 1;
    }

    /// This function exists because branching both consumes an instruction and puts one.
    /// TODO: Make it not an edge case
    /// (idea: rework `propagate()` into 'sync(usize)' to accomodate any mid-exec changes)
    pub fn unstep(&mut self) {
        self.c_stack.last_mut().unwrap().length += 1;
    }
    
    /// To be called after an instruction is run.
    /// `step()` and `propagate()` are separated so that calls to `dump()` are accurate.
    pub fn propagate(&mut self) {
        while self.c_stack.len() > 1 && self.c_stack.last().unwrap().length < 1 {
            self.c_stack.pop();
        }
    }

    /// Returns the current state of the stack, using indentation to represent depth into the call stack.
    pub fn dump(&self) -> String {
        let mut result = "".to_string();
        let mut indentation = 0;
        for frame in self.c_stack.iter() {
            indentation += 1;
            let instr_ptr = frame.capacity - frame.length;
            result += &format!(
                "{}in \"${}\", instruction {}\n",
                    "  ".repeat(indentation),
                    frame.name,
                    instr_ptr
            ).to_string();
        }
        result
    }
}

