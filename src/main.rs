mod memblock;
mod stack;
use stack::Stack;
use std::env;

fn main() {
    let mut stack = Stack::new();
    let fname = env::args().nth(1);
    match &fname {
        None => {
            println!("Incorrect amount of args.");
            std::process::exit(1)
        }
        _ => (),
    }
    let content = std::fs::read_to_string(fname.unwrap());

    match &content {
        Err(i) => {
            println!("{:?}", i);
            std::process::exit(1)
        }

        _ => (),
    }
    let mut linum = 0;
    let content = content.unwrap();
    let content = content.split("\n").collect::<Vec<_>>();
    for (i, c) in content.iter().enumerate() {
        let mut badvariablename = c.trim_start().trim_end().split(" ");
        let e = badvariablename.nth(0).unwrap();
        let second = badvariablename.nth(0);
        if e == "checkpoint" {
            stack.checkpoint(String::from(second.unwrap()), i)
        }
    }
    while linum < content.len() {
        let code = content[linum].trim_start().trim_end();
        if code == "" {
            linum += 1;
            continue;
        }
        if &code[0..=1] == ";;" {
            linum += 1;
            continue;
        }
        let mut instructions = code.split(" ");
        let first = instructions.nth(0);
        if first.is_none() {
            linum += 1;
            continue;
        }
        let second = instructions.nth(0);
        let third = instructions.nth(0);
        match first.unwrap() {

            "linum" => stack.i64_const(linum as i64),

            "int.const" => stack.i64_const(second.unwrap().parse::<i64>().unwrap()),

            "int.add" => stack.i64_add(),

            "int.sub" => stack.i64_sub(),

            "int.mul" => stack.i64_mul(),

            "int.div" => stack.i64_div(),

            "int.copy" => stack.i64_copy(),

            "int.gt" => stack.i64_gt(),

            "int.lt" => stack.i64_lt(),

            "int.eq" => stack.i64_eq(),

            "int.rot" => stack.i64_rot(),

            "checkpoint" => (),

            "goto" => linum = stack.goto(String::from(second.unwrap())),

            "if" => {
                linum = stack.if_smt(
                    linum,
                    String::from(second.unwrap()),
                    String::from(third.unwrap()),
                )
            }

            "mem.malloc" => stack.malloc(),

            "mem.free" => stack.free(),

            "mem.copy_block" => stack.copyblock(),

            "mem.write" => stack.mem_write(),

            "mem.append" => stack.mem_append(),

            "stack.init" => stack.frame_init(second.unwrap().parse::<i32>().unwrap()),

            "stack.pop" => stack.frame_pop(second.unwrap().parse::<i32>().unwrap()),

            "out.int" => stack.putint(),

            "jmp" => linum = stack.jump(),

            "debug.dump" => stack.dump_stack(),

            "var.store" => stack.store(String::from(second.unwrap())),

            "var.load" => stack.load(String::from(second.unwrap())),

            other => {
                println!("Unknown command: {}", other);
                std::process::exit(1)
            }
        }
        linum += 1;
    }
}
