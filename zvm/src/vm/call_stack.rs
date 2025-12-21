use crate::{
    debug_log,
    parser::class_file::ClassFile,
    vm::{runtime::RuntimeDataArea, stack_frame::Frame, value::Value},
};

pub struct CallStack {
    pub frames: Vec<Frame>,
    max_depth: usize,
}

impl CallStack {
    pub fn new(max_depth: Option<usize>) -> Self {
        Self {
            frames: Vec::new(),
            max_depth: max_depth.unwrap_or(1000),
        }
    }

    /// Handle pushing frames
    pub fn push_frame(
        &mut self,
        method_name: String,
        bytecode: Vec<u8>,
        max_locals: usize,
        args: Vec<Value>,
    ) {
        // TODO: Handle StackOverFlowException

        // Create the frame and initialize it
        let mut frame = Frame::new(Some(method_name.clone()), max_locals, bytecode);

        // Get the passed arguments and store them in the current frame's local variables
        let mut current_arg_index = 0;
        let mut i = 0;

        while i < max_locals {
            if i < frame.local_variables.vars.len() && current_arg_index < args.len() {
                let current_arg = args[current_arg_index].clone();

                // NOTE: Double and long values take two places in the local variables array
                // of the current frame meanwhile any other type takes just one place

                if matches!(current_arg, Value::Long(_) | Value::Double(_)) {
                    frame.local_variables.vars[i] = Some(current_arg.clone());
                    frame.local_variables.vars[i + 1] = Some(current_arg);
                    i += 1;
                } else {
                    frame.local_variables.vars[i] = Some(current_arg);
                }

                current_arg_index += 1;
            }

            i += 1;
        }

        self.frames.push(frame);
    }

    /// Handle popping frames
    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }
    /// Handle borrowing current frame mutably
    pub fn current_frame(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut()
    }

    /// Handle borrowing current frame immutably
    pub fn current_frame_ref(&self) -> Option<&Frame> {
        self.frames.last()
    }

    /// Handle checking call stack emptiness
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Handle getting current frame method name
    pub fn current_method_name(&self) -> Option<&str> {
        self.current_frame_ref()?.method_name.as_deref()
    }

    /// Handle executing frames
    pub fn execute_frames(
        &mut self,
        class_file: &ClassFile,
        runtime_data_area: &mut RuntimeDataArea,
    ) {
        while !self.is_empty() {
            //TODO: This is a workaround to fool the borrow checker,
            //TODO: Figure out a safe solution
            // Execute using raw pointer to work around borrow checker
            let execution_result = unsafe {
                // Get a raw pointer to self
                let self_ptr = self as *mut Self;

                // Borrow current frame mutably
                if let Some(current_frame) = (*self_ptr).current_frame() {
                    // Pass self through the raw pointer ( second mutable borrow )
                    current_frame.execute_frame(class_file, runtime_data_area, &mut *self_ptr)
                } else {
                    break;
                }
            };

            match execution_result {
                Ok(returned) => {
                    self.pop_frame();
                    if let Some(value) = returned {
                        if let Some(invoker_frame) = self.current_frame() {
                            invoker_frame.operand_stack.push(value);
                        }
                    }
                }
                Err(msg) => {
                    debug_log!("Error executing frame: {}", msg);
                }
            }
        }
    }

    /// Handle getting current call stack size
    pub fn size(&self) -> usize {
        self.frames.len()
    }

    /// Handle printing current stack frames
    pub fn print_frames(&self) {
        for (i, frame) in self.frames.iter().enumerate() {
            let frame_name = frame
                .method_name
                .as_deref()
                .expect("Failed to get frame name!");
            debug_log!("Frame[{}]: {}", i, frame_name);
        }
    }

    //TODO: Handle frames returning stuff
    //TODO: Handle frames returning void
    // pub fn return_void(&mut self) -> Result<(), String> {
    //     self.return_from_method(None)
    // }
    //TODO: Handle printing stack trace
}
