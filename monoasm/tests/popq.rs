  extern crate monoasm;
  extern crate monoasm_macro;
  use std::io::Write;

  use monoasm::*;
  use monoasm_macro::monoasm;

  #[test]
  fn popq() {
      let mut jit: JitMemory = JitMemory::new();
      monoasm!(
          jit,
	popq rax;
	popq rcx;
	popq rdx;
	popq rbx;
	popq rsp;
	popq rbp;
	popq rsi;
	popq rdi;
	popq r8;
	popq r9;
	popq r10;
	popq r11;
	popq r12;
	popq r13;
	popq r14;
	popq r15;
	popq [rax];
	popq [rax + 16];
	popq [rax + 512];
	popq [rcx];
	popq [rcx + 16];
	popq [rcx + 512];
	popq [rdx];
	popq [rdx + 16];
	popq [rdx + 512];
	popq [rbx];
	popq [rbx + 16];
	popq [rbx + 512];
	popq [rsp];
	popq [rsp + 16];
	popq [rsp + 512];
	popq [rbp];
	popq [rbp + 16];
	popq [rbp + 512];
	popq [rsi];
	popq [rsi + 16];
	popq [rsi + 512];
	popq [rdi];
	popq [rdi + 16];
	popq [rdi + 512];
	popq [r8];
	popq [r8 + 16];
	popq [r8 + 512];
	popq [r9];
	popq [r9 + 16];
	popq [r9 + 512];
	popq [r10];
	popq [r10 + 16];
	popq [r10 + 512];
	popq [r11];
	popq [r11 + 16];
	popq [r11 + 512];
	popq [r12];
	popq [r12 + 16];
	popq [r12 + 512];
	popq [r13];
	popq [r13 + 16];
	popq [r13 + 512];
	popq [r14];
	popq [r14 + 16];
	popq [r14 + 512];
	popq [r15];
	popq [r15 + 16];
	popq [r15 + 512];
	popq [rip];
	popq [rip + 16];
	popq [rip + 512];
	popq [rax + rax * 1];
	popq [rax + rax * 2];
	popq [rax + rax * 4];
	popq [rax + rax * 8];
	popq [rax + r15 * 1];
	popq [rax + r15 * 2];
	popq [rax + r15 * 4];
	popq [rax + r15 * 8];
	popq [rcx + rax * 1];
	popq [rcx + rax * 2];
	popq [rcx + rax * 4];
	popq [rcx + rax * 8];
	popq [rcx + r15 * 1];
	popq [rcx + r15 * 2];
	popq [rcx + r15 * 4];
	popq [rcx + r15 * 8];
	popq [rdx + rax * 1];
	popq [rdx + rax * 2];
	popq [rdx + rax * 4];
	popq [rdx + rax * 8];
	popq [rdx + r15 * 1];
	popq [rdx + r15 * 2];
	popq [rdx + r15 * 4];
	popq [rdx + r15 * 8];
	popq [rbx + rax * 1];
	popq [rbx + rax * 2];
	popq [rbx + rax * 4];
	popq [rbx + rax * 8];
	popq [rbx + r15 * 1];
	popq [rbx + r15 * 2];
	popq [rbx + r15 * 4];
	popq [rbx + r15 * 8];
	popq [rsp + rax * 1];
	popq [rsp + rax * 2];
	popq [rsp + rax * 4];
	popq [rsp + rax * 8];
	popq [rsp + r15 * 1];
	popq [rsp + r15 * 2];
	popq [rsp + r15 * 4];
	popq [rsp + r15 * 8];
	popq [rbp + rax * 1];
	popq [rbp + rax * 2];
	popq [rbp + rax * 4];
	popq [rbp + rax * 8];
	popq [rbp + r15 * 1];
	popq [rbp + r15 * 2];
	popq [rbp + r15 * 4];
	popq [rbp + r15 * 8];
	popq [rsi + rax * 1];
	popq [rsi + rax * 2];
	popq [rsi + rax * 4];
	popq [rsi + rax * 8];
	popq [rsi + r15 * 1];
	popq [rsi + r15 * 2];
	popq [rsi + r15 * 4];
	popq [rsi + r15 * 8];
	popq [rdi + rax * 1];
	popq [rdi + rax * 2];
	popq [rdi + rax * 4];
	popq [rdi + rax * 8];
	popq [rdi + r15 * 1];
	popq [rdi + r15 * 2];
	popq [rdi + r15 * 4];
	popq [rdi + r15 * 8];
	popq [r8 + rax * 1];
	popq [r8 + rax * 2];
	popq [r8 + rax * 4];
	popq [r8 + rax * 8];
	popq [r8 + r15 * 1];
	popq [r8 + r15 * 2];
	popq [r8 + r15 * 4];
	popq [r8 + r15 * 8];
	popq [r9 + rax * 1];
	popq [r9 + rax * 2];
	popq [r9 + rax * 4];
	popq [r9 + rax * 8];
	popq [r9 + r15 * 1];
	popq [r9 + r15 * 2];
	popq [r9 + r15 * 4];
	popq [r9 + r15 * 8];
	popq [r10 + rax * 1];
	popq [r10 + rax * 2];
	popq [r10 + rax * 4];
	popq [r10 + rax * 8];
	popq [r10 + r15 * 1];
	popq [r10 + r15 * 2];
	popq [r10 + r15 * 4];
	popq [r10 + r15 * 8];
	popq [r11 + rax * 1];
	popq [r11 + rax * 2];
	popq [r11 + rax * 4];
	popq [r11 + rax * 8];
	popq [r11 + r15 * 1];
	popq [r11 + r15 * 2];
	popq [r11 + r15 * 4];
	popq [r11 + r15 * 8];
	popq [r12 + rax * 1];
	popq [r12 + rax * 2];
	popq [r12 + rax * 4];
	popq [r12 + rax * 8];
	popq [r12 + r15 * 1];
	popq [r12 + r15 * 2];
	popq [r12 + r15 * 4];
	popq [r12 + r15 * 8];
	popq [r13 + rax * 1];
	popq [r13 + rax * 2];
	popq [r13 + rax * 4];
	popq [r13 + rax * 8];
	popq [r13 + r15 * 1];
	popq [r13 + r15 * 2];
	popq [r13 + r15 * 4];
	popq [r13 + r15 * 8];
	popq [r14 + rax * 1];
	popq [r14 + rax * 2];
	popq [r14 + rax * 4];
	popq [r14 + rax * 8];
	popq [r14 + r15 * 1];
	popq [r14 + r15 * 2];
	popq [r14 + r15 * 4];
	popq [r14 + r15 * 8];
	popq [r15 + rax * 1];
	popq [r15 + rax * 2];
	popq [r15 + rax * 4];
	popq [r15 + rax * 8];
	popq [r15 + r15 * 1];
	popq [r15 + r15 * 2];
	popq [r15 + r15 * 4];
	popq [r15 + r15 * 8];
      );
      jit.finalize();
      let mut buf = std::fs::File::create("tests/popq_monoasm.bin").unwrap();
      buf.write_all(jit.as_slice()).unwrap();
  }
