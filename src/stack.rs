use crate::memblock::{MemBlock, FreeBlock};
use std::collections::HashMap;

const STACK_SIZE: usize = 65_535;
const HEAP_SIZE: usize = 65_535;

pub struct Stack {
    stack: [u8; STACK_SIZE],
    pointer: usize,
    checkpoints: HashMap<String, usize>,
    heap: [u8; HEAP_SIZE],
    freeblocks: Vec<FreeBlock>,
    memblocks: HashMap<i32, MemBlock>,
    frames: Vec<usize>,
    nextblock: i32
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; 65_535],
            pointer: 0,
            checkpoints: HashMap::new(),
            heap: [0; 65_535],
            freeblocks: vec![FreeBlock {
                start: 0,
                end: 65_534,
            }],
            memblocks: HashMap::new(),
            frames: Vec::new(),
            nextblock: 1
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
            return linum 
          }
          return self.goto(t);
        } else {
          if f == "NULL" {
            return linum
          }
          return self.goto(f);
        }
    }

    pub fn jump(&mut self) -> usize {
      let ret = i64::from_be_bytes(self.get_top_i64()) as usize;
      ret
    }

    pub fn malloc(&mut self) {
        let size = i64::from_be_bytes(self.get_top_i64()) as usize;
        let mut ret: i64 = -1;
        for block in &mut self.freeblocks {
            if block.end - block.start >= size {
                ret = block.start as i64;
                block.start = block.start + size;
                self.memblocks.insert(self.nextblock, MemBlock {
                    start: ret as usize,
                    end: ret as usize + size - 1,
                    size,
                    password: None
                });
                ret = self.nextblock as i64;
                self.nextblock += 1;
                break;
            }
        }
        if ret == -1 {
            println!(
                "Heap Overflow: Tried to allocate {} bytes, could not find free block",
                size
            );
            std::process::exit(1);
        } else {
            self.i64_const(ret)
        }
    }

    pub fn free(&mut self) {
        let blocknum = i64::from_be_bytes(self.get_top_i64()) as usize;
        let block = self.memblocks.get(&(blocknum as i32)).unwrap();
        self.freeblocks.push(FreeBlock {
            start: block.start,
            end: block.start + (block.size - 1),
        });
        self.memblocks.remove(&(blocknum as i32));
    }

    pub fn i64_store(&mut self) {
        let mut address = i64::from_be_bytes(self.get_top_i64());
        let val = self.get_top_i64();
        for byte in val {
            self.heap[address as usize] = byte;
            address += 1
        }
    }

    pub fn i64_load(&mut self) {
        let address = i64::from_be_bytes(self.get_top_i64());
        let mut val = [0, 0, 0, 0, 0, 0, 0, 0];
        for i in 0..8 {
            val[i as usize] = self.heap[(address + i) as usize]
        }
        for s in val.iter() {
            self.push(*s)
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
      let mut ret: [u8; 8] = [0,0,0,0,0,0,0,0];
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
        println! {"{:?}", &self.stack[0..=self.pointer]}
    }
}
