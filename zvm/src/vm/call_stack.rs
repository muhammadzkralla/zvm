use crate::{
    parser::{class_file::ClassFile, opcode::Opcode},
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

    // FRAME:
    // pub method_name: Option<String>,
    // pub operand_stack: OperandStack,
    // pub local_variables: LocalVariables,
    // pub pc: usize,
    // pub bytecode: Vec<u8>,

    //TODO: Handle pushing frames
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

        // Get the arguments
        for (i, arg) in args.iter().enumerate() {
            if i < frame.local_variables.vars.len() {
                frame.local_variables.vars[i] = Some(arg.clone());
            }
        }

        self.frames.push(frame);
    }
    //TODO: Handle popping frames
    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }
    //TODO: Handle peeking frames
    pub fn current_frame(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut()
    }

    pub fn current_frame_ref(&self) -> Option<&Frame> {
        self.frames.last()
    }
    //TODO: Handle checking call stack emptiness
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
    //TODO: Handle getting current frame method name
    pub fn current_method_name(&self) -> Option<&str> {
        self.current_frame_ref()?.method_name.as_deref()
    }
    //TODO: Handle executing frames
    pub fn execute_frames(
        &mut self,
        class_file: &ClassFile,
        runtime_data_area: &mut RuntimeDataArea,
    ) {
    while let Some(mut frame) = self.pop_frame() {
        frame.execute_frame(class_file, runtime_data_area, self);

        println!(
            "FINISHED EXECUTING FRAME: {}",
            frame.method_name.expect("Failed to get method name")
        );
    }}

    //TODO: Handle getting current call stack size
    pub fn size(&self) -> usize {
        self.frames.len()
    }

    //TODO: Handle frames returning stuff
    //TODO: Handle frames returning void
    // pub fn return_void(&mut self) -> Result<(), String> {
    //     self.return_from_method(None)
    // }
    //TODO: Handle printing stack trace
}
