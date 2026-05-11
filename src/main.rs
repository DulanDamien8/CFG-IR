use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
enum Operand {
    Register(String),
    Immediate(i32),
    Memory(String),
    Label(String),
}

#[derive(Debug, Clone)]
struct Instruction {
    mnemonic: String,
    operands: Vec<Operand>,
    comment: Option<String>,
}

impl Instruction {
    fn new(mnemonic: &str, operands: Vec<Operand>) -> Self {
        Instruction {
            mnemonic: mnemonic.to_string(),
            operands,
            comment: None,
        }
    }

    fn with_comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_string());
        self
    }

    fn to_string(&self) -> String {
        let ops = self
            .operands
            .iter()
            .map(|op| match op {
                Operand::Register(r) => r.clone(),
                Operand::Immediate(i) => {
                    let hex = format!("{:x}", i);
                    if hex.chars().next().unwrap().is_alphabetic() {
                        format!("0{}h", hex)
                    } else {
                        format!("{}h", hex)
                    }
                }
                Operand::Memory(m) => m.clone(),
                Operand::Label(l) => l.clone(),
            })
            .collect::<Vec<_>>()
            .join(", ");

        let base = if self.mnemonic.ends_with(':') {
            format!("{}", self.mnemonic)
        } else if ops.is_empty() {
            format!("    {}", self.mnemonic)
        } else {
            format!("    {} {}", self.mnemonic, ops)
        };

        if let Some(comment) = &self.comment {
            format!("{:<40} ; {}", base, comment)
        } else {
            base
        }
    }

    fn is_branch(&self) -> bool {
        self.mnemonic.starts_with('j') && self.mnemonic != "jmp"
    }

    fn is_unconditional_jump(&self) -> bool {
        self.mnemonic == "jmp"
    }

    fn is_call(&self) -> bool {
        self.mnemonic == "call"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BlockType {
    Entry,
    Basic,
    Loop,
    Exit,
}

#[derive(Debug, Clone)]
struct BasicBlock {
    #[allow(dead_code)]
    id: usize,
    label: Option<String>,
    instructions: Vec<Instruction>,
    block_type: BlockType,
    successors: Vec<usize>,
    predecessors: Vec<usize>,
}

impl BasicBlock {
    fn new(id: usize, block_type: BlockType) -> Self {
        BasicBlock {
            id,
            label: None,
            instructions: Vec::new(),
            block_type,
            successors: Vec::new(),
            predecessors: Vec::new(),
        }
    }

    fn add_instruction(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    fn add_successor(&mut self, successor_id: usize) {
        if !self.successors.contains(&successor_id) {
            self.successors.push(successor_id);
        }
    }

    fn add_predecessor(&mut self, predecessor_id: usize) {
        if !self.predecessors.contains(&predecessor_id) {
            self.predecessors.push(predecessor_id);
        }
    }
}

#[derive(Debug)]
struct ControlFlowGraph {
    blocks: HashMap<usize, BasicBlock>,
    entry_block: usize,
    exit_blocks: Vec<usize>,
    label_to_block: HashMap<String, usize>,
    next_block_id: usize,
}

impl ControlFlowGraph {
    fn new() -> Self {
        ControlFlowGraph {
            blocks: HashMap::new(),
            entry_block: 0,
            exit_blocks: Vec::new(),
            label_to_block: HashMap::new(),
            next_block_id: 0,
        }
    }

    fn create_block(&mut self, block_type: BlockType) -> usize {
        let id = self.next_block_id;
        self.next_block_id += 1;
        self.blocks.insert(id, BasicBlock::new(id, block_type));
        id
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        if let Some(from_block) = self.blocks.get_mut(&from) {
            from_block.add_successor(to);
        }
        if let Some(to_block) = self.blocks.get_mut(&to) {
            to_block.add_predecessor(from);
        }
    }

    fn get_block_mut(&mut self, id: usize) -> Option<&mut BasicBlock> {
        self.blocks.get_mut(&id)
    }

    fn get_block(&self, id: usize) -> Option<&BasicBlock> {
        self.blocks.get(&id)
    }

    fn topological_sort(&self) -> Vec<usize> {
        let mut in_degree: HashMap<usize, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        for &block_id in self.blocks.keys() {
            let block = self.blocks.get(&block_id).unwrap();
            in_degree.insert(block_id, block.predecessors.len());
            if block.predecessors.is_empty() {
                queue.push_back(block_id);
            }
        }

        while let Some(block_id) = queue.pop_front() {
            result.push(block_id);
            if let Some(block) = self.blocks.get(&block_id) {
                for &successor in &block.successors {
                    if let Some(degree) = in_degree.get_mut(&successor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(successor);
                        }
                    }
                }
            }
        }

        if result.len() != self.blocks.len() {
            let mut fallback: Vec<usize> = self.blocks.keys().copied().collect();
            fallback.sort();
            return fallback;
        }

        result
    }
}

struct CFGMetamorphicEngine {
    cfg: ControlFlowGraph,
    rng: rand::rngs::StdRng,
    unused_regs: Vec<String>,
}

impl CFGMetamorphicEngine {
    fn new(seed: u64) -> Self {
        CFGMetamorphicEngine {
            cfg: ControlFlowGraph::new(),
            rng: rand::rngs::StdRng::seed_from_u64(seed),
            unused_regs: vec![
                "r8".to_string(), "r9".to_string(), "r10".to_string(),
                "r11".to_string(), "r12".to_string(), "r13".to_string(),
                "r14".to_string(), "r15".to_string(),
            ],
        }
    }

    fn parse_and_build_cfg(&mut self) {
        let instructions = vec![
            Instruction::new("sub", vec![Operand::Register("rsp".to_string()), Operand::Immediate(0x28)])
                .with_comment("32 bytes shadow space + alignment"),
            Instruction::new("mov", vec![Operand::Register("rcx".to_string()), Operand::Immediate(0x0A)])
                .with_comment("n = 10"),
            Instruction::new("xor", vec![Operand::Register("rax".to_string()), Operand::Register("rax".to_string())])
                .with_comment("f0 = 0"),
            Instruction::new("mov", vec![Operand::Register("rdx".to_string()), Operand::Immediate(0x01)])
                .with_comment("f1 = 1"),
            Instruction::new("fib_loop:", vec![]),
            Instruction::new("add", vec![Operand::Register("rax".to_string()), Operand::Register("rdx".to_string())]),
            Instruction::new("xchg", vec![Operand::Register("rax".to_string()), Operand::Register("rdx".to_string())]),
            Instruction::new("dec", vec![Operand::Register("rcx".to_string())]),
            Instruction::new("jnz", vec![Operand::Label("fib_loop".to_string())]),
            Instruction::new("mov", vec![Operand::Register("rcx".to_string()), Operand::Register("rax".to_string())])
                .with_comment("exit code = fib(10)"),
            Instruction::new("call", vec![Operand::Label("ExitProcess".to_string())]),
            Instruction::new("add", vec![Operand::Register("rsp".to_string()), Operand::Immediate(0x28)])
                .with_comment("(never reached, but ABI-correct)"),
        ];
        self.build_cfg_from_instructions(instructions);
    }

    fn build_cfg_from_instructions(&mut self, instructions: Vec<Instruction>) {
        let entry_id = self.cfg.create_block(BlockType::Entry);
        self.cfg.entry_block = entry_id;
        let mut current_block_id = entry_id;
        let mut pending_branches: Vec<(usize, String)> = Vec::new();

        for inst in instructions.iter() {
            if inst.mnemonic.ends_with(':') {
                let label = inst.mnemonic.trim_end_matches(':').to_string();
                if !self.cfg.get_block(current_block_id).unwrap().instructions.is_empty() {
                    let new_block_id = self.cfg.create_block(BlockType::Basic);
                    self.cfg.add_edge(current_block_id, new_block_id);
                    current_block_id = new_block_id;
                }
                if let Some(block) = self.cfg.get_block_mut(current_block_id) {
                    block.label = Some(label.clone());
                }
                self.cfg.label_to_block.insert(label.clone(), current_block_id);
                continue;
            }
            if inst.is_branch() {
                if let Some(block) = self.cfg.get_block_mut(current_block_id) {
                    block.add_instruction(inst.clone());
                    if let Some(Operand::Label(target)) = inst.operands.first() {
                        pending_branches.push((current_block_id, target.clone()));
                    }
                }
                let fall_through_id = self.cfg.create_block(BlockType::Basic);
                self.cfg.add_edge(current_block_id, fall_through_id);
                current_block_id = fall_through_id;
            } else if inst.is_unconditional_jump() {
                if let Some(block) = self.cfg.get_block_mut(current_block_id) {
                    block.add_instruction(inst.clone());
                    if let Some(Operand::Label(target)) = inst.operands.first() {
                        pending_branches.push((current_block_id, target.clone()));
                    }
                }
                let new_block_id = self.cfg.create_block(BlockType::Basic);
                current_block_id = new_block_id;
            } else if inst.is_call() {
                if let Some(block) = self.cfg.get_block_mut(current_block_id) {
                    block.add_instruction(inst.clone());
                }
                let exit_block_id = self.cfg.create_block(BlockType::Exit);
                self.cfg.add_edge(current_block_id, exit_block_id);
                self.cfg.exit_blocks.push(exit_block_id);
                current_block_id = exit_block_id;
            } else {
                if let Some(block) = self.cfg.get_block_mut(current_block_id) {
                    block.add_instruction(inst.clone());
                }
            }
        }

        for (from_block, target_label) in pending_branches {
            if let Some(&target_block) = self.cfg.label_to_block.get(&target_label) {
                self.cfg.add_edge(from_block, target_block);
                if let Some(block) = self.cfg.get_block_mut(target_block) {
                    if block.block_type == BlockType::Basic && block.predecessors.len() > 1 {
                        block.block_type = BlockType::Loop;
                    }
                }
            }
        }
        self.identify_loop_blocks();
    }

    fn identify_loop_blocks(&mut self) {
        let block_ids: Vec<usize> = self.cfg.blocks.keys().copied().collect();
        for &block_id in &block_ids {
            let has_back_edge = {
                let block = self.cfg.get_block(block_id).unwrap();
                block.successors.iter().any(|&succ| succ <= block_id)
            };
            if has_back_edge {
                if let Some(block) = self.cfg.get_block_mut(block_id) {
                    block.block_type = BlockType::Loop;
                }
            }
        }
    }

    fn transform_block_instructions(&mut self) {
        let block_ids: Vec<usize> = self.cfg.blocks.keys().copied().collect();
        for block_id in block_ids {
            let block = self.cfg.blocks.get(&block_id).unwrap().clone();
            let mut new_instructions = Vec::new();
            for inst in &block.instructions {
                let is_stack_op = matches!(inst.mnemonic.as_str(), "sub" | "add" | "call")
                    && inst.operands.first().map_or(false, |op| matches!(op, Operand::Register(r) if r == "rsp"));
                if is_stack_op || inst.is_call() {
                    new_instructions.push(inst.clone());
                    continue;
                }
                match inst.mnemonic.as_str() {
                    "xor" if inst.operands.len() == 2 && inst.operands[0] == inst.operands[1] => {
                        let choice = self.rng.gen_range(0..3);
                        match choice {
                            0 => new_instructions.push(Instruction::new("sub", vec![inst.operands[0].clone(), inst.operands[1].clone()]).with_comment("CFG: xor -> sub")),
                            1 => new_instructions.push(Instruction::new("and", vec![inst.operands[0].clone(), Operand::Immediate(0)]).with_comment("CFG: xor -> and 0")),
                            _ => new_instructions.push(inst.clone()),
                        }
                    }
                    "mov" if matches!(inst.operands.get(1), Some(Operand::Immediate(1))) => {
                        let choice = self.rng.gen_range(0..3);
                        match choice {
                            0 => {
                                new_instructions.push(Instruction::new("xor", vec![inst.operands[0].clone(), inst.operands[0].clone()]).with_comment("CFG: mov 1 -> xor; inc"));
                                new_instructions.push(Instruction::new("inc", vec![inst.operands[0].clone()]));
                            }
                            1 => {
                                new_instructions.push(Instruction::new("push", vec![Operand::Immediate(1)]).with_comment("CFG: mov 1 -> push; pop"));
                                new_instructions.push(Instruction::new("pop", vec![inst.operands[0].clone()]));
                            }
                            _ => new_instructions.push(inst.clone()),
                        }
                    }
                    "mov" if matches!(inst.operands.get(1), Some(Operand::Immediate(10))) => {
                        let choice = self.rng.gen_range(0..4);
                        match choice {
                            0 => {
                                new_instructions.push(Instruction::new("mov", vec![inst.operands[0].clone(), Operand::Immediate(5)]).with_comment("CFG: mov 10 -> mov 5; add 5"));
                                new_instructions.push(Instruction::new("add", vec![inst.operands[0].clone(), Operand::Immediate(5)]));
                            }
                            1 => {
                                new_instructions.push(Instruction::new("mov", vec![inst.operands[0].clone(), Operand::Immediate(2)]).with_comment("CFG: mov 10 -> mov 2; imul 5"));
                                new_instructions.push(Instruction::new("imul", vec![inst.operands[0].clone(), Operand::Immediate(5)]));
                            }
                            2 => {
                                new_instructions.push(Instruction::new("push", vec![Operand::Immediate(10)]).with_comment("CFG: mov 10 -> push; pop"));
                                new_instructions.push(Instruction::new("pop", vec![inst.operands[0].clone()]));
                            }
                            _ => new_instructions.push(inst.clone()),
                        }
                    }
                    "xchg" if inst.operands.len() == 2 => {
                        if self.rng.gen_bool(0.2) {
                            new_instructions.push(Instruction::new("push", vec![inst.operands[0].clone()]).with_comment("CFG: xchg -> push/mov/pop"));
                            new_instructions.push(Instruction::new("mov", vec![inst.operands[0].clone(), inst.operands[1].clone()]));
                            new_instructions.push(Instruction::new("pop", vec![inst.operands[1].clone()]));
                        } else {
                            new_instructions.push(inst.clone());
                        }
                    }
                    "dec" => {
                        if self.rng.gen_bool(0.3) {
                            new_instructions.push(Instruction::new("sub", vec![inst.operands[0].clone(), Operand::Immediate(1)]).with_comment("CFG: dec -> sub 1"));
                        } else {
                            new_instructions.push(inst.clone());
                        }
                    }
                    _ => new_instructions.push(inst.clone()),
                }
            }
            if let Some(block) = self.cfg.get_block_mut(block_id) {
                block.instructions = new_instructions;
            }
        }
    }

    fn insert_dead_code_in_blocks(&mut self) {
        let block_ids: Vec<usize> = self.cfg.blocks.keys().copied().collect();
        for block_id in block_ids {
            if self.cfg.get_block(block_id).unwrap().block_type == BlockType::Exit { continue; }
            let block = self.cfg.blocks.get(&block_id).unwrap().clone();
            let mut new_instructions = Vec::new();
            for (i, inst) in block.instructions.iter().enumerate() {
                new_instructions.push(inst.clone());
                let is_stack_op = matches!(inst.mnemonic.as_str(), "sub" | "add" | "call")
                    && inst.operands.first().map_or(false, |op| matches!(op, Operand::Register(r) if r == "rsp"));
                if is_stack_op || inst.is_call() { continue; }
                let is_flag_sensitive = matches!(inst.mnemonic.as_str(), "dec" | "add" | "sub" | "cmp");
                let next_is_branch = block.instructions.get(i + 1).map_or(false, |next| next.is_branch());
                if is_flag_sensitive && next_is_branch { continue; }
                if self.rng.gen_bool(0.25) {
                    if let Some(reg) = self.unused_regs.choose(&mut self.rng) {
                        let dead_choice = self.rng.gen_range(0..4);
                        match dead_choice {
                            0 => new_instructions.push(Instruction::new("nop", vec![]).with_comment("CFG dead code")),
                            1 => new_instructions.push(Instruction::new("mov", vec![Operand::Register(reg.clone()), Operand::Register(reg.clone())]).with_comment("CFG dead code: nop equiv")),
                            2 => {
                                new_instructions.push(Instruction::new("push", vec![Operand::Register(reg.clone())]).with_comment("CFG dead code: push/pop"));
                                new_instructions.push(Instruction::new("pop", vec![Operand::Register(reg.clone())]).with_comment("CFG dead code: push/pop"));
                            }
                            _ => new_instructions.push(Instruction::new("lea", vec![Operand::Register(reg.clone()), Operand::Memory(format!("[{}]", reg))]).with_comment("CFG dead code: lea nop")),
                        }
                    }
                }
            }
            if let Some(block) = self.cfg.get_block_mut(block_id) {
                block.instructions = new_instructions;
            }
        }
    }

    fn block_reordering(&mut self) {
        let original_order = self.cfg.topological_sort();
        let mut reorderable_blocks = Vec::new();

        for &block_id in &original_order {
            let block = self.cfg.get_block(block_id).unwrap();
            if block.block_type != BlockType::Entry && block.block_type != BlockType::Exit {
                reorderable_blocks.push(block_id);
            }
        }

        if reorderable_blocks.len() > 1 && self.rng.gen_bool(0.3) {
            let choice = self.rng.gen_range(0..2);
            match choice {
                0 => { reorderable_blocks.shuffle(&mut self.rng); }
                _ => { reorderable_blocks.reverse(); }
            }
            for &block_id in &reorderable_blocks {
                if let Some(block) = self.cfg.get_block_mut(block_id) {
                    for inst in &mut block.instructions {
                        if inst.comment.is_none() && !inst.mnemonic.ends_with(':') {
                            inst.comment = Some("CFG: reordered block".to_string());
                            break;
                        }
                    }
                }
            }
        }
    }

    // Transformation 4: Register Renaming
    // Renames registers used in dead code instructions to diversify the code
    fn register_renaming(&mut self) {
        let block_ids: Vec<usize> = self.cfg.blocks.keys().copied().collect();

        // Build a rename map: pick random unused regs as replacements
        let mut rename_map: HashMap<String, String> = HashMap::new();
        let dead_code_regs = vec!["r10".to_string(), "r11".to_string(), "r8".to_string(), "r9".to_string()];
        for reg in &dead_code_regs {
            if let Some(replacement) = self.unused_regs.choose(&mut self.rng) {
                rename_map.insert(reg.clone(), replacement.clone());
            }
        }

        for block_id in block_ids {
            let block = self.cfg.blocks.get(&block_id).unwrap().clone();
            let mut new_instructions = Vec::new();

            for inst in &block.instructions {
                let is_dead_code = inst.comment.as_ref().map_or(false, |c| c.contains("dead code"));

                if is_dead_code {
                    let mut renamed = inst.clone();
                    for operand in &mut renamed.operands {
                        match operand {
                            Operand::Register(reg) => {
                                if let Some(new_reg) = rename_map.get(reg.as_str()) {
                                    *reg = new_reg.clone();
                                }
                            }
                            Operand::Memory(mem) => {
                                if mem.starts_with('[') && mem.ends_with(']') {
                                    let inner = mem[1..mem.len()-1].to_string();
                                    if let Some(new_reg) = rename_map.get(&inner) {
                                        *mem = format!("[{}]", new_reg);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    // Update comment to reflect renaming
                    if let Some(comment) = &mut renamed.comment {
                        if !comment.contains("renamed") {
                            *comment = format!("{} (renamed)", comment);
                        }
                    }
                    new_instructions.push(renamed);
                } else {
                    new_instructions.push(inst.clone());
                }
            }

            if let Some(block) = self.cfg.get_block_mut(block_id) {
                block.instructions = new_instructions;
            }
        }
    }

    fn apply_all_transformations(&mut self) {
        println!("[CFG] Transforming block instructions...");
        self.transform_block_instructions();
        println!("[CFG] Inserting dead code into blocks...");
        self.insert_dead_code_in_blocks();
        println!("[CFG] Applying block reordering...");
        self.block_reordering();
        println!("[CFG] Performing register renaming...");
        self.register_renaming();
    }

    fn generate_assembly(&self) -> String {
        let mut output = String::new();
        output.push_str("; CFG-based Metamorphic variant generated by Rust CFG Transformation Engine\n");
        output.push_str("; Bachelor's Thesis: Design and Evaluation of a Rust Based Metamorphic Malware Transformation Engine\n\n");
        output.push_str("extern ExitProcess : proc\n\n");
        output.push_str(".code\n");
        output.push_str("main PROC\n");
        let sorted_blocks = self.cfg.topological_sort();
        for &block_id in &sorted_blocks {
            if let Some(block) = self.cfg.get_block(block_id) {
                if let Some(ref label) = block.label {
                    output.push_str(&format!("{}:\n", label));
                }
                for inst in &block.instructions {
                    output.push_str(&format!("{}\n", inst.to_string()));
                }
            }
        }
        output.push_str("main ENDP\n");
        output.push_str("END\n");
        output
    }

    fn extract_opcodes(&self) -> Vec<String> {
        let mut opcodes = Vec::new();
        let sorted_blocks = self.cfg.topological_sort();
        for &block_id in &sorted_blocks {
            if let Some(block) = self.cfg.get_block(block_id) {
                for inst in &block.instructions {
                    opcodes.push(inst.mnemonic.clone());
                }
            }
        }
        opcodes
    }
}

// ---------------------------------------------------------------------------
// Build automation
// ---------------------------------------------------------------------------

const EXPECTED_EXIT_CODE: i32 = 55;

fn assemble_and_link(asm_path: &str) -> Result<String, String> {
    let base = asm_path.trim_end_matches(".asm");
    let obj_path = format!("{}.obj", base);
    let exe_path = format!("{}.exe", base);

    let asm_result = Command::new("ml64")
        .args(&["/c", "/nologo", asm_path, &format!("/Fo{}", obj_path)])
        .output()
        .map_err(|e| format!("ml64 launch failed: {}", e))?;
    if !asm_result.status.success() {
        return Err(format!("ml64 failed for {}: {}", asm_path, String::from_utf8_lossy(&asm_result.stderr)));
    }

    let link_result = Command::new("link")
        .args(&[&obj_path, "/subsystem:console", "/entry:main", "kernel32.lib", "/nologo", &format!("/out:{}", exe_path)])
        .output()
        .map_err(|e| format!("link launch failed: {}", e))?;
    if !link_result.status.success() {
        return Err(format!("link failed for {}: {}", obj_path, String::from_utf8_lossy(&link_result.stderr)));
    }
    Ok(exe_path)
}

fn verify_semantic_preservation(exe_path: &str, label: &str) -> bool {
    match Command::new(exe_path).output() {
        Ok(result) => match result.status.code() {
            Some(code) if code == EXPECTED_EXIT_CODE => {
                println!("  [OK]  {} -> exit code {}", label, code);
                true
            }
            Some(code) => {
                println!("  [FAIL] {} -> exit code {} (expected {})", label, code, EXPECTED_EXIT_CODE);
                false
            }
            None => { println!("  [ERR] {} terminated by signal", label); false }
        },
        Err(e) => { println!("  [ERR] {} -> {}", label, e); false }
    }
}

fn generate_build_script(num_variants: usize) {
    let mut s = String::new();
    s.push_str("@echo off\r\n");
    s.push_str("REM --- CFG Engine build script (auto-loads VS tools) ---\r\n\r\n");

    // vcvars auto-detection
    s.push_str("if exist \"C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Auxiliary\\Build\\vcvars64.bat\" (\r\n");
    s.push_str("    call \"C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Auxiliary\\Build\\vcvars64.bat\" >nul 2>&1\r\n");
    s.push_str(") else if exist \"C:\\Program Files\\Microsoft Visual Studio\\2022\\Professional\\VC\\Auxiliary\\Build\\vcvars64.bat\" (\r\n");
    s.push_str("    call \"C:\\Program Files\\Microsoft Visual Studio\\2022\\Professional\\VC\\Auxiliary\\Build\\vcvars64.bat\" >nul 2>&1\r\n");
    s.push_str(") else if exist \"C:\\Program Files\\Microsoft Visual Studio\\2022\\Enterprise\\VC\\Auxiliary\\Build\\vcvars64.bat\" (\r\n");
    s.push_str("    call \"C:\\Program Files\\Microsoft Visual Studio\\2022\\Enterprise\\VC\\Auxiliary\\Build\\vcvars64.bat\" >nul 2>&1\r\n");
    s.push_str(") else (\r\n");
    s.push_str("    echo [ERROR] Could not find VS 2022 vcvars64.bat\r\n");
    s.push_str("    pause & exit /b 1\r\n");
    s.push_str(")\r\n");
    s.push_str("where ml64 >nul 2>&1 || ( echo [ERROR] ml64 not found & pause & exit /b 1 )\r\n");
    s.push_str("echo [+] VS build tools loaded\r\n\r\n");

    // pushd into output
    s.push_str("pushd output\r\n\r\n");

    // original
    s.push_str("echo [*] Building cfg_original...\r\n");
    s.push_str("ml64 /c /nologo cfg_original.asm\r\n");
    s.push_str("link cfg_original.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_original.exe\r\n");
    s.push_str("cfg_original.exe\r\n");
    s.push_str("echo     cfg_original exit code: %ERRORLEVEL%\r\n\r\n");

    for v in 1..=num_variants {
        s.push_str(&format!("echo [*] Building cfg_variant_{}...\r\n", v));
        s.push_str(&format!("ml64 /c /nologo cfg_variant_{v}.asm\r\n"));
        s.push_str(&format!("link cfg_variant_{v}.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_{v}.exe\r\n"));
        s.push_str(&format!("cfg_variant_{v}.exe\r\n"));
        s.push_str(&format!("echo     cfg_variant_{v} exit code: %ERRORLEVEL%\r\n\r\n"));
    }

    s.push_str("popd\r\n");
    s.push_str("echo [+] Done. All exit codes should be 55 (fib(10)).\r\n");
    s.push_str("pause\r\n");

    fs::write("build_cfg.bat", &s).expect("Unable to write build_cfg.bat");
    println!("[+] Build script saved to: build_cfg.bat");
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== CFG-Based Metamorphic Transformation Engine ===\n");

    let can_build = Command::new("ml64").arg("/?").output().map(|o| o.status.success()).unwrap_or(false);
    if can_build {
        println!("[+] ml64 detected — automated build & verification enabled");
    } else {
        println!("[!] ml64 not found — run build_cfg.bat from any terminal");
    }

    let base_seed = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_nanos() as u64;
    println!("[*] Base seed: {}\n", base_seed);

    let num_variants: usize = 50;
    let mut semantic_results: Vec<(String, bool)> = Vec::new();
    fs::create_dir_all("output").unwrap();

    // Original
    {
        let mut original = CFGMetamorphicEngine::new(0);
        original.parse_and_build_cfg();
        fs::write("output/cfg_original.asm", original.generate_assembly()).unwrap();
        fs::write("output/cfg_original_opcodes.txt", original.extract_opcodes().join("\n")).unwrap();
        println!("[+] cfg_original.asm written");
        if can_build {
            match assemble_and_link("output/cfg_original.asm") {
                Ok(exe) => { let ok = verify_semantic_preservation(&exe, "cfg_original"); semantic_results.push(("cfg_original".into(), ok)); }
                Err(e) => { println!("  [ERR] {}", e); semantic_results.push(("cfg_original".into(), false)); }
            }
        }
    }

    // Variants
    for v in 1..=num_variants {
        println!("\n[*] Generating CFG variant {}...", v);
        let mut engine = CFGMetamorphicEngine::new(base_seed.wrapping_add(v as u64));
        engine.parse_and_build_cfg();
        println!("[*] CFG built with {} blocks", engine.cfg.blocks.len());
        engine.apply_all_transformations();

        let asm_path = format!("output/cfg_variant_{}.asm", v);
        fs::write(&asm_path, engine.generate_assembly()).unwrap();
        fs::write(format!("output/cfg_variant_{}_opcodes.txt", v), engine.extract_opcodes().join("\n")).unwrap();
        println!("[+] CFG variant {} saved", v);

        if can_build {
            let label = format!("cfg_variant_{}", v);
            match assemble_and_link(&asm_path) {
                Ok(exe) => { let ok = verify_semantic_preservation(&exe, &label); semantic_results.push((label, ok)); }
                Err(e) => { println!("  [ERR] {}", e); semantic_results.push((format!("cfg_variant_{}", v), false)); }
            }
        }
    }

    generate_build_script(num_variants);

    // Summary
    println!("\n{}", "=".repeat(60));
    println!("CFG GENERATION SUMMARY");
    println!("{}", "=".repeat(60));
    if can_build && !semantic_results.is_empty() {
        let pass = semantic_results.iter().filter(|(_, ok)| *ok).count();
        println!("{}/{} variants semantically correct (exit code {})", pass, semantic_results.len(), EXPECTED_EXIT_CODE);
        let csv: Vec<String> = semantic_results.iter()
            .map(|(l, ok)| format!("{},{},{}", l, if *ok {"PASS"} else {"FAIL"}, EXPECTED_EXIT_CODE)).collect();
        fs::write("output/cfg_semantic_verification.csv", format!("variant,status,expected\n{}", csv.join("\n"))).unwrap();
    }
    println!("\n[*] Run Evaluate_cfg.py or: docker compose -f docker-compose-cfg.yml up");
}