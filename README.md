# The Lepton Virtual Machine (LeViM)

## Introduction
The Lepton Virtual Machine is a modular and interpreted stack machine written in the Rust Programming Language. The goal of Lepton is to create a simple yet fast virtual machine that is extendable, uses low memory, and has fast start-up times.

## Licensing
LeViM is free software, a phrase which here means free as in "freedom", but also free as in "free pistacios for everyone". Modification and distribution of LeViM is governed by the GNU Public License v2. 

## Naming
LeViM is named after the Tau Lepton, an elementry particle which is similar to the electron. LeViM is light and fast, just like a small elementary particle. The acronym LeViM instead of LVM is to prevent confusion with the LLVM project, which is in no way affiliated with LeViM. LeViM has no affiliation with the VIM project or Lepton CMS.

## Design Decisions
The Lepton Virtual Machine is written in Rust because of its focus on memory safety without sacrificing speed. Rust allows LeViM to be fast and safe without being too verbose. The zero-cost abstractions in Rust improve development speed and readability.

LeViM is interpreted and does no runtime optimizations on purpose. Although JIT compilation is very fast, it has a high startup cost, consumes large amounts of memory, and complexifies the code base. The goal of LeViM is to be simple yet universal, so interpretation is a must. Runtime optimizations increase startup time and memory usage. LeViM Assembly should not be computer optimized, as line number discrepancies will change the function of line-based dynamic `jmp` commands.

LeViM Assembly does not have any bytecode format; all assembly commands are interpreted directly. This is to prevent conflicts in opcodes when HashMap operation lookup and assembly extensions are implemented.

## Extensibility
HashMap-based operation lookup is not yet implemented in LeViM.
