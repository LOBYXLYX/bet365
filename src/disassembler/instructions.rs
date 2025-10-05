use std::collections::HashMap;
use std::path::MAIN_SEPARATOR;
use crate::disassembler::disassembler::Disassembler;
use crate::disassembler::opcodes::OpCodes;

type InstructionType = fn(&mut Disassembler);
#[derive(Debug)]
pub struct Instructions;

impl Instructions {
    pub fn get_instructions() -> HashMap<u8, InstructionType> {
        let mut instructions: HashMap<u8, InstructionType> = HashMap::new();

        instructions.insert(124, Instructions::init_memory);
        instructions.insert(23, Instructions::new_value);
        instructions.insert(251, Instructions::get_property);
        instructions.insert(215, Instructions::call_function);
        instructions.insert(6, Instructions::mul_op);
        instructions.insert(241, Instructions::mov_imm24);
        instructions.insert(90, Instructions::call_apply);
        instructions.insert(55, Instructions::div_op);
        instructions.insert(65, Instructions::or_op);
        instructions.insert(230, Instructions::sub_op);
        instructions.insert(88, Instructions::push_args);
        instructions.insert(181, Instructions::load_imm24);
        instructions.insert(49, Instructions::jump_frame);
        instructions.insert(171, Instructions::new_function);
        instructions.insert(20, Instructions::less_than);
        instructions.insert(39, Instructions::jump_if_false);
        instructions.insert(112, Instructions::less_than);
        instructions.insert(99, Instructions::set_property);
        instructions.insert(243, Instructions::add_op);
        instructions.insert(93, Instructions::jump);
        instructions.insert(166, Instructions::halt);
        instructions.insert(53, Instructions::shl_op);
        instructions.insert(17, Instructions::function_ret);
        instructions.insert(78, Instructions::equal_op);
        instructions.insert(117, Instructions::xor_op);
        instructions.insert(51, Instructions::load_double);
        instructions.insert(40, Instructions::ushr_op);
        instructions.insert(149, Instructions::shr_op);
        instructions.insert(37, Instructions::and_op);
        instructions.insert(156, Instructions::mod_op);
        instructions.insert(247, Instructions::lte_op);
        instructions.insert(214, Instructions::lte_op);
        instructions.insert(22, Instructions::notequal_op);
        instructions.insert(83, Instructions::jump_if_true);
        instructions.insert(115, Instructions::try_catch);
        instructions.insert(161, Instructions::strict_equal_op);
        instructions.insert(220, Instructions::strict_notequal_op);
        instructions.insert(5, Instructions::throw_op);

        instructions
    }

    fn init_memory(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let value: u8 = disasm.get_byte();
        disasm.trace.push(format!("{} {value} -> reg{reg}", OpCodes::InitMemory.as_str()));
    }

    fn new_value(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let value: String = disasm.decode_value();

        disasm.trace.push(format!("{} '{value}' -> reg{reg}", OpCodes::NewValue.as_str()));
        disasm.registers[reg as usize] = value;
    }

    fn get_property(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let obj_reg = disasm.get_byte();
        let prop_reg = disasm.get_byte();

        let val: String = if &disasm.registers[prop_reg as usize] != "_free_reg_" {
            disasm.registers[prop_reg as usize].clone()
        } else {
            format!("reg{}", prop_reg)
        };

        disasm.trace.push(format!("{} reg{obj_reg}[{val}] -> reg{reg}", OpCodes::GetProperty.as_str()));   
    }

    fn call_function(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let func_reg: u8 = disasm.get_byte();
        let func: String = if &disasm.registers[func_reg as usize] != "_free_reg_" {
            disasm.registers[func_reg as usize].clone()
        } else {
            format!("reg{}", func_reg)
        };

        let arg_len: u8 = disasm.get_byte();
        let mut args: Vec<String> = Vec::new();

        for _ in 0..arg_len {
            let arg_reg = disasm.get_byte();
            args.push(format!("reg{}", arg_reg))
        };

        let args: String = args.join(",");

        disasm.trace.push(format!("{} {func}({args}) -> reg{reg}", OpCodes::CallFunction.as_str()));
    }

    fn mul_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} * reg{right_reg} -> reg{reg}", OpCodes::Mul.as_str()));
    }

    fn mov_imm24(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let val_24: u32 = disasm.get_int24();

        disasm.trace.push(format!("{} {val_24} -> reg{reg}", OpCodes::MovImm24.as_str()));
    }

    fn call_apply(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let func_reg: u8 = disasm.get_byte();

        let func: String = if &disasm.registers[func_reg as usize] != "_free_reg_" {
            disasm.registers[func_reg as usize].clone()
        } else {
            format!("reg{}", func_reg)
        };
        let this_reg: u8 = disasm.get_byte();
        let arg_len: u8 = disasm.get_byte();
        let mut args: Vec<String> = Vec::new();

        for _ in 0..arg_len {
            let arg_reg = disasm.get_byte();
            args.push(format!("reg{}", arg_reg))
        };

        let args: String = args.join(",");
        disasm.trace.push(format!("{} {func}.apply(reg{this_reg}, [{args}]) -> reg{reg}", OpCodes::CallApply.as_str()));
    }

    fn div_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} / reg{right_reg} -> reg{reg}", OpCodes::Div.as_str()));
    }

    fn or_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} | reg{right_reg} -> reg{reg}", OpCodes::Or.as_str()));
    }

    fn sub_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} - reg{right_reg} -> reg{reg}", OpCodes::Sub.as_str()));
    }

    fn push_args(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let arg_len: u8 = disasm.get_byte();
        let mut args: Vec<String> = Vec::new();

        for _ in 0..arg_len {
            let arg_reg = disasm.get_byte();
            args.push(format!("reg{}", arg_reg))
        };

        let args: String = args.join(",");
        disasm.trace.push(format!("{} [{args}] -> reg{reg}", OpCodes::PushArgs.as_str()));
    }

    fn load_imm24(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let val_24: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} {val_24} -> reg{reg}", OpCodes::LoadImm24.as_str()));
    }

    fn jump_frame(disasm: &mut Disassembler) {
        let ptr: u32 = disasm.get_int24();
        let context: u8 = disasm.get_byte();
        let params_count: u8 = disasm.get_byte();
        let mut params: Vec<String> = Vec::new();
        
        for _ in 0..params_count {
            let param_reg = disasm.get_byte();
            params.push(format!("reg{}", param_reg))
        };

        let params: String = params.join(",");
        disasm.trace.push(format!("{} entry({ptr}), {context}, params({params})", OpCodes::JumpFrame.as_str()));
    }

    fn new_function(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let func_entry: u32 = disasm.get_int24();
        let args_len: u8 = disasm.get_byte();

        let mut args: Vec<String> = Vec::new();

        for _ in 0..args_len {
            let arg_reg = disasm.get_byte();
            args.push(format!("reg{}", arg_reg))
        };

        let args: String = args.join(",");
        disasm.trace.push(format!("{} entry({func_entry}), args({args})", OpCodes::NewFunction.as_str()));
    }

    fn less_than(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} < reg{right_reg} -> reg{reg}", OpCodes::LessThan.as_str()));
    }

    fn jump_if_false(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let ptr: u32 = disasm.get_int24();

        disasm.trace.push(format!("{} reg{reg}, entry({ptr})", OpCodes::JumpIfFalse.as_str()));        
    }

    fn set_property(disasm: &mut Disassembler) {
        let obj_reg = disasm.get_byte();
        let prop_reg = disasm.get_byte();
        let val_reg = disasm.get_byte();

        let val: String = if &disasm.registers[val_reg as usize] != "_free_reg_" {
            disasm.registers[val_reg as usize].clone()
        } else {
            format!("reg{}", val_reg)
        };

        let prop: String = if &disasm.registers[prop_reg as usize] != "_free_reg_" {
            disasm.registers[prop_reg as usize].clone()
        } else {
            format!("reg{}", prop_reg)
        };

        disasm.trace.push(format!("{} reg{obj_reg}[{prop}] = {val}", OpCodes::SetProperty.as_str()));        
    }

    fn add_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} + reg{right_reg} -> reg{reg}", OpCodes::Add.as_str()));
    }

    fn jump(disasm: &mut Disassembler) {
        let ptr: u32 = disasm.get_int24();
        disasm.trace.push(format!("{} {ptr}", OpCodes::Jump.as_str()));
    }

    fn halt(disasm: &mut Disassembler) {
        disasm.trace.push(format!("{}", OpCodes::Halt.as_str()));
    }

    fn shl_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} << reg{right_reg} -> reg{reg}", OpCodes::Shl.as_str()));
    }

    fn function_ret(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let count = disasm.get_byte();
        let mut list: Vec<String> = Vec::new();

        for _ in 0..count {
            let ret_reg = disasm.get_byte();
            list.push(format!("reg{}", ret_reg))
        };

        let list: String = list.join(",");
        disasm.trace.push(format!("{} {reg} [{list}]", OpCodes::Ret.as_str()));
    }

    fn equal_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} == reg{right_reg} -> reg{reg}", OpCodes::Equal.as_str()));
    }

    fn xor_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} ^ reg{right_reg} -> reg{reg}", OpCodes::Xor.as_str()));
    }

    fn load_double(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let val: f64 = disasm.read_double();

        disasm.trace.push(format!("{} {val} -> reg{reg}", OpCodes::LoadDouble.as_str()));
    }

    fn ushr_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} >>> reg{right_reg} -> reg{reg}", OpCodes::Ushr.as_str()));
    }

    fn shr_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} >> reg{right_reg} -> reg{reg}", OpCodes::Shr.as_str()));
    }

    fn and_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} & reg{right_reg} -> reg{reg}", OpCodes::And.as_str()));
    }

    fn mod_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} % reg{right_reg} -> reg{reg}", OpCodes::Mod.as_str()));
    }

    fn lte_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} <= reg{right_reg} -> reg{reg}", OpCodes::Lte.as_str()));
    }

    fn notequal_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} != reg{right_reg} -> reg{reg}", OpCodes::NotEqual.as_str()));
    }

    fn jump_if_true(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let ptr: u32 = disasm.get_int24();

        disasm.trace.push(format!("{} reg{reg}, entry({ptr})", OpCodes::JumpIfTrue.as_str()));        
    }

    fn try_catch(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let catch_offset = disasm.get_int24();
        let finally_offset = disasm.get_int24();
        let continue_offset = disasm.get_int24();

        disasm.trace.push(format!("{} [{catch_offset}, {finally_offset}, {continue_offset}] -> reg{reg}", OpCodes::TryCatch.as_str()));   
    }

    fn strict_equal_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} === reg{right_reg} -> reg{reg}", OpCodes::StrictEqual.as_str()));
    }

    fn strict_notequal_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
        let left_reg: u8 = disasm.get_byte();
        let right_reg: u8 = disasm.get_byte();

        disasm.trace.push(format!("{} reg{left_reg} !== reg{right_reg} -> reg{reg}", OpCodes::StrictNotEqual.as_str()));
    }

    fn throw_op(disasm: &mut Disassembler) {
        let reg: u8 = disasm.get_byte();
    
        disasm.trace.push(format!("{} {reg}", OpCodes::Throw.as_str()));
    }
}