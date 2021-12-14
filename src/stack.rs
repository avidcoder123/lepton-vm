use crate::memblock::{MemBlock};
use std::collections::HashMap;

const STACK_SIZE: usize = 65_535;
const HEAP_SIZE: usize = 65_535;

pub struct Stack {
    stack: [u8; STACK_SIZE],
    pointer: usize,
    checkpoints: HashMap<String, usize>,
    memblocks: HashMap<usize, MemBlock>,
    frames: Vec<usize>,
    nextblock: i32,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; 65_535],
            pointer: 0,
            checkpoints: HashMap::new(),
            memblocks: HashMap::new(),
            frames: Vec::new(),
            nextblock: 1,
        }
    }

    pub fn push(&mut self, val: u8) {
        self.stack[self.pointer] = val;
        self.pointer += 1;
    }

    pub fn pop(&mut self) -> () {
        self.pointer -= 1;
        self.stack[self.pointer] = 0;
    }

    //Destructive top function
    pub fn top(&mut self) -> u8 {
        self.pointer -= 1;
        let ret = self.stack[self.pointer];
        self.pointer += 1;
        self.pop();
        ret
    }

    fn get_top_i64(&mut self) -> [u8; 8] {
        let mut i: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        for j in (0..=7).rev() {
            i[j as usize] = self.top()
        }
        i
    }

    pub fn i64_const(&mut self, n: i64) {
        let bytes = n.to_le_bytes();
        for i in bytes.iter().rev() {
            self.push(*i);
        }
    }

    pub fn i64_add(&mut self) {
        let int2 = self.get_top_i64();
        let int1 = self.get_top_i64();

        let int1 = i64::from_be_bytes(int1);
        let int2 = i64::from_be_bytes(int2);

        let sum = (int1 + int2).to_be_bytes();

        for s in sum.iter() {
            self.push(*s)
        }
    }

    pub fn i64_sub(&mut self) {
        let int2 = self.get_top_i64();
        let int1 = self.get_top_i64();

        let int1 = i64::from_le_bytes(int1);
        let int2 = i64::from_le_bytes(int2);

        let sum = (int1 - int2).to_le_bytes();

        for s in sum.iter() {
            self.push(*s)
        }
    }

    pub fn i64_mul(&mut self) {
        let int2 = self.get_top_i64();
        let int1 = self.get_top_i64();

        let int1 = i64::from_be_bytes(int1);
        let int2 = i64::from_be_bytes(int2);

        let sum = (int1 * int2).to_be_bytes();

        for s in sum.iter() {
            self.push(*s)
        }
    }

    pub fn i64_div(&mut self) {
        let int2 = self.get_top_i64();
        let int1 = self.get_top_i64();

        let int1 = i64::from_le_bytes(int1);
        let int2 = i64::from_le_bytes(int2);
        let sum = (int1 / int2).to_be_bytes();

        for s in sum.iter() {
            self.push(*s)
        }
    }

    pub fn i64_copy(&mut self) {
        let i = self.get_top_i64();
        for j in i {
            self.push(j)
        }
        for j in i {
            self.push(j)
        }
    }

    pub fn i64_gt(&mut self) {
        let int2 = self.get_top_i64();
        let int1 = self.get_top_i64();
        let condition = int1 > int2;
        if condition {
            self.push(1)
        } else {
            self.push(0)
        }
    }

    pub fn i64_lt(&mut self) {
        let int2 = self.get_top_i64();
        let int1 = self.get_top_i64();
        let condition = int1 < int2;
        if condition {
            self.push(1)
        } else {
            self.push(0)
        }
    }

    pub fn i64_eq(&mut self) {
        let int2 = self.get_top_i64();
        let int1 = self.get_top_i64();
        let condition = int1 == int2;
        if condition {
            self.push(1)
        } else {
            self.push(0)
        }
    }

    pub fn i64_rot(&mut self) {
        let int1 = self.get_top_i64();
        let int2 = self.get_top_i64();
        for s in int1.iter() {
            self.push(*s)
        }
        for s in int2.iter() {
            self.push(*s)
        }
    }

    pub fn checkpoint(&mut self, name: String, ins: usize) {
        self.checkpoints.insert(name, ins);
        return;
    }

    pub fn goto(&self, point: String) -> usize {
        *self.checkpoints.get(&point).unwrap()
    }

    pub fn if_smt(&mut self, linum: usize, t: String, f: String) -> usize {
        let boolean = self.top();
        if boolean == 1 {
            if t == "NULL" {
                return linum;
            }
            return self.goto(t);
        } else {
            if f == "NULL" {
                return linum;
            }
            return self.goto(f);
        }
    }

    pub fn jump(&mut self) -> usize {
        let ret = i64::from_be_bytes(self.get_top_i64()) as usize;
        ret
    }

    pub fn malloc(&mut self) {
        let blockid = self.nextblock;
        self.memblocks.insert(
            blockid as usize,
            MemBlock {
                content: Vec::new()
            }
        );
    }

    pub fn free(&mut self) {
        let blocknum = i64::from_be_bytes(self.get_top_i64()) as usize;
        self.memblocks.remove(&(blocknum));
    }

    pub fn copyblock(&mut self) {
        let blocknum = i64::from_be_bytes(self.get_top_i64()) as usize;
        let block = self.memblocks.get(&blocknum).unwrap();
        for i in &block.content {
            self.push(*i)
        }
    }

    pub fn mem_write(&mut self) {
        let blocknum = i64::from_be_bytes(self.get_top_i64()) as usize;
        let byteamount = i64::from_be_bytes(self.get_top_i64()) as usize;

        
        let block = self.memblocks.get(&blocknum).unwrap();
        
        let mut to_write: Vec<u8> = Vec::new();
        for _i in 0..byteamount {
            to_write.push(self.top())
        }
        to_write.reverse();
        block.content = to_write;
    }

    pub fn mem_append(&mut self) {
        let blocknum = i64::from_be_bytes(self.get_top_i64()) as usize;
        let byteamount = i64::from_be_bytes(self.get_top_i64()) as usize;
        
        let block = self.memblocks.get(&blocknum).unwrap();
        
        let mut to_write: Vec<u8> = Vec::new();
        for _i in 0..byteamount {
            to_write.push(self.top())
        }
        to_write.reverse();
        for val in to_write {
            block.content.push(val);
        }
    }

    pub fn frame_init(&mut self, offset: i32) {
        self.frames.push(self.pointer - offset as usize)
    }

    pub fn frame_pop(&mut self, offset: i32) {
        let mut save: Vec<u8> = Vec::new();
        for _i in 0..offset {
            save.push(self.top())
        }
        let save = save.iter().rev();
        while self.pointer != *self.frames.last().unwrap() {
            self.pop()
        }
        for i in save {
            self.push(*i)
        }
        self.frames.pop();
    }

    pub fn frame_get(&mut self, offset: usize) {
        self.push(self.stack[*self.frames.last().unwrap() + offset])
    }

    pub fn int_local(&mut self, offset: usize) {
        let mut ret: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut set = offset;
        for i in 0..8 {
            ret[i] = self.stack[*self.frames.last().unwrap() + set];
            set += 1;
        }
        for i in ret {
            self.push(i)
        }
    }

    pub fn putint(&mut self) {
        let i = self.get_top_i64();
        println!("{}", i64::from_be_bytes(i));
    }

    pub fn dump_stack(&self) {
        println!("STACK DUMP:");
        println! {"{:?}", &self.stack[0..self.pointer]}
    }
}
