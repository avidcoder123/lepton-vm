mod memblock;
mod stack;
use colored::*;
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
        let errs = match first.unwrap() {
            "linum" => stack.i64_const(linum as i64),

            "int.const" => {
                match second {
                    Some(e) => match e.parse::<i64>() {
                        Ok(i) => stack.i64_const(i),
                        Err(_) => Err(
                            String::from("Expected number, got string")
                        )
                    },
                    None => Err(
                        String::from("Missing Argument")
                    )
                }
            },

            "int.add" => stack.i64_add(),

            "int.sub" => stack.i64_sub(),

            "int.mul" => stack.i64_mul(),

            "int.div" => stack.i64_div(),

            "int.copy" => stack.i64_copy(),

            "int.gt" => stack.i64_gt(),

            "int.lt" => stack.i64_lt(),

            "int.eq" => stack.i64_eq(),

            "int.rot" => stack.i64_rot(),

            "checkpoint" => Ok(()),

            "goto" => {
                let to_jmp = stack.goto(String::from(second.unwrap()));

                match to_jmp {
                    Ok(e) => {
                        linum = e;
                        Ok(())
                    }

                    Err(e) => Err(e),
                }
            }

            "if" => {
                let to_jmp = match second {
                    Some(e) => {
                        match third {
                            Some(i) => stack.if_smt(
                                linum, 
                                String::from(e), 
                                String::from(i)
                            ),
                            None => Err(
                                String::from("Missing Argument")
                            )
                        }
                    },
                    None => Err(
                        String::from("Missing Argument")
                    )
                };

                match to_jmp {
                    Ok(e) => {
                        linum = e;
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }

            "mem.malloc" => stack.malloc(),

            "mem.free" => stack.free(),

            "mem.copy_block" => stack.copyblock(),

            "mem.write" => stack.mem_write(),

            "mem.append" => stack.mem_append(),

            "stack.init" => {
                match second {
                    Some(e) => match e.parse::<i32>() {
                        Ok(i) => stack.frame_init(i),
                        Err(_) => Err(
                            String::from("Expected number, got string")
                        )
                    },
                    None => Err(
                        String::from("Missing Argument")
                    )
                }
            },

            "stack.pop" => {
                match second {
                    Some(e) => match e.parse::<i32>() {
                        Ok(i) => stack.frame_pop(i),
                        Err(_) => Err(
                            String::from("Expected number, got string")
                        )
                    },
                    None => Err(
                        String::from("Missing Argument")
                    )
                }
            },

            "out.int" => stack.putint(),

            "jmp" => {
                linum = stack.jump().unwrap();
                Ok(())
            }

            "debug.dump" => stack.dump_stack(),

            "var.store" => {
                match second {
                    Some(e) => stack.store(
                        String::from(e)
                    ),
                    None => Err(
                        String::from("Missing Argument")
                    )
                }
            },

            "var.load" => {
                match second {
                    Some(e) => stack.load(
                        String::from(e)
                    ),
                    None => Err(
                        String::from("Missing Argument")
                    )
                }
            },

            _other => {
                Err(
                    String::from("Unknown Instruction")
                )
            }
        };
        if errs.is_err() {
            println!(
                "{}\n{}\n{}\n{}{}{} {}\n{}",
                "Fatal Error:".red().bold(),
                errs.err().unwrap().green(),
                "~~~~~~~".red().bold(),
                "Line ".purple(),
                (linum + 1).to_string().purple(),
                ":".purple(),
                first.unwrap().yellow(),
                "        ^ Here".red().bold()
            );
            std::process::exit(1);
        }
        linum += 1;
        stack.linum = linum
    }
}
