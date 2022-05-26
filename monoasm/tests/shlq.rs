  extern crate monoasm;
  extern crate monoasm_macro;
  use std::io::Write;

  use monoasm::*;
  use monoasm_macro::monoasm;

  #[test]
  fn shlq() {
      let mut jit: JitMemory = JitMemory::new();
      monoasm!(
          jit,
	shlq rax, 1;
	shlq rax, 18;
	shlq rcx, 1;
	shlq rcx, 18;
	shlq rdx, 1;
	shlq rdx, 18;
	shlq rbx, 1;
	shlq rbx, 18;
	shlq rsp, 1;
	shlq rsp, 18;
	shlq rbp, 1;
	shlq rbp, 18;
	shlq rsi, 1;
	shlq rsi, 18;
	shlq rdi, 1;
	shlq rdi, 18;
	shlq r8, 1;
	shlq r8, 18;
	shlq r9, 1;
	shlq r9, 18;
	shlq r10, 1;
	shlq r10, 18;
	shlq r11, 1;
	shlq r11, 18;
	shlq r12, 1;
	shlq r12, 18;
	shlq r13, 1;
	shlq r13, 18;
	shlq r14, 1;
	shlq r14, 18;
	shlq r15, 1;
	shlq r15, 18;
	shlq [rax], 1;
	shlq [rax], 18;
	shlq [rax + 16], 1;
	shlq [rax + 16], 18;
	shlq [rax + 512], 1;
	shlq [rax + 512], 18;
	shlq [rcx], 1;
	shlq [rcx], 18;
	shlq [rcx + 16], 1;
	shlq [rcx + 16], 18;
	shlq [rcx + 512], 1;
	shlq [rcx + 512], 18;
	shlq [rdx], 1;
	shlq [rdx], 18;
	shlq [rdx + 16], 1;
	shlq [rdx + 16], 18;
	shlq [rdx + 512], 1;
	shlq [rdx + 512], 18;
	shlq [rbx], 1;
	shlq [rbx], 18;
	shlq [rbx + 16], 1;
	shlq [rbx + 16], 18;
	shlq [rbx + 512], 1;
	shlq [rbx + 512], 18;
	shlq [rsp], 1;
	shlq [rsp], 18;
	shlq [rsp + 16], 1;
	shlq [rsp + 16], 18;
	shlq [rsp + 512], 1;
	shlq [rsp + 512], 18;
	shlq [rbp], 1;
	shlq [rbp], 18;
	shlq [rbp + 16], 1;
	shlq [rbp + 16], 18;
	shlq [rbp + 512], 1;
	shlq [rbp + 512], 18;
	shlq [rsi], 1;
	shlq [rsi], 18;
	shlq [rsi + 16], 1;
	shlq [rsi + 16], 18;
	shlq [rsi + 512], 1;
	shlq [rsi + 512], 18;
	shlq [rdi], 1;
	shlq [rdi], 18;
	shlq [rdi + 16], 1;
	shlq [rdi + 16], 18;
	shlq [rdi + 512], 1;
	shlq [rdi + 512], 18;
	shlq [r8], 1;
	shlq [r8], 18;
	shlq [r8 + 16], 1;
	shlq [r8 + 16], 18;
	shlq [r8 + 512], 1;
	shlq [r8 + 512], 18;
	shlq [r9], 1;
	shlq [r9], 18;
	shlq [r9 + 16], 1;
	shlq [r9 + 16], 18;
	shlq [r9 + 512], 1;
	shlq [r9 + 512], 18;
	shlq [r10], 1;
	shlq [r10], 18;
	shlq [r10 + 16], 1;
	shlq [r10 + 16], 18;
	shlq [r10 + 512], 1;
	shlq [r10 + 512], 18;
	shlq [r11], 1;
	shlq [r11], 18;
	shlq [r11 + 16], 1;
	shlq [r11 + 16], 18;
	shlq [r11 + 512], 1;
	shlq [r11 + 512], 18;
	shlq [r12], 1;
	shlq [r12], 18;
	shlq [r12 + 16], 1;
	shlq [r12 + 16], 18;
	shlq [r12 + 512], 1;
	shlq [r12 + 512], 18;
	shlq [r13], 1;
	shlq [r13], 18;
	shlq [r13 + 16], 1;
	shlq [r13 + 16], 18;
	shlq [r13 + 512], 1;
	shlq [r13 + 512], 18;
	shlq [r14], 1;
	shlq [r14], 18;
	shlq [r14 + 16], 1;
	shlq [r14 + 16], 18;
	shlq [r14 + 512], 1;
	shlq [r14 + 512], 18;
	shlq [r15], 1;
	shlq [r15], 18;
	shlq [r15 + 16], 1;
	shlq [r15 + 16], 18;
	shlq [r15 + 512], 1;
	shlq [r15 + 512], 18;
	shlq [rip], 1;
	shlq [rip], 18;
	shlq [rip + 16], 1;
	shlq [rip + 16], 18;
	shlq [rip + 512], 1;
	shlq [rip + 512], 18;
	shlq [rax + rax * 1], 1;
	shlq [rax + rax * 1], 18;
	shlq [rax + rax * 2], 1;
	shlq [rax + rax * 2], 18;
	shlq [rax + rax * 4], 1;
	shlq [rax + rax * 4], 18;
	shlq [rax + rax * 8], 1;
	shlq [rax + rax * 8], 18;
	shlq [rax + r15 * 1], 1;
	shlq [rax + r15 * 1], 18;
	shlq [rax + r15 * 2], 1;
	shlq [rax + r15 * 2], 18;
	shlq [rax + r15 * 4], 1;
	shlq [rax + r15 * 4], 18;
	shlq [rax + r15 * 8], 1;
	shlq [rax + r15 * 8], 18;
	shlq [rcx + rax * 1], 1;
	shlq [rcx + rax * 1], 18;
	shlq [rcx + rax * 2], 1;
	shlq [rcx + rax * 2], 18;
	shlq [rcx + rax * 4], 1;
	shlq [rcx + rax * 4], 18;
	shlq [rcx + rax * 8], 1;
	shlq [rcx + rax * 8], 18;
	shlq [rcx + r15 * 1], 1;
	shlq [rcx + r15 * 1], 18;
	shlq [rcx + r15 * 2], 1;
	shlq [rcx + r15 * 2], 18;
	shlq [rcx + r15 * 4], 1;
	shlq [rcx + r15 * 4], 18;
	shlq [rcx + r15 * 8], 1;
	shlq [rcx + r15 * 8], 18;
	shlq [rdx + rax * 1], 1;
	shlq [rdx + rax * 1], 18;
	shlq [rdx + rax * 2], 1;
	shlq [rdx + rax * 2], 18;
	shlq [rdx + rax * 4], 1;
	shlq [rdx + rax * 4], 18;
	shlq [rdx + rax * 8], 1;
	shlq [rdx + rax * 8], 18;
	shlq [rdx + r15 * 1], 1;
	shlq [rdx + r15 * 1], 18;
	shlq [rdx + r15 * 2], 1;
	shlq [rdx + r15 * 2], 18;
	shlq [rdx + r15 * 4], 1;
	shlq [rdx + r15 * 4], 18;
	shlq [rdx + r15 * 8], 1;
	shlq [rdx + r15 * 8], 18;
	shlq [rbx + rax * 1], 1;
	shlq [rbx + rax * 1], 18;
	shlq [rbx + rax * 2], 1;
	shlq [rbx + rax * 2], 18;
	shlq [rbx + rax * 4], 1;
	shlq [rbx + rax * 4], 18;
	shlq [rbx + rax * 8], 1;
	shlq [rbx + rax * 8], 18;
	shlq [rbx + r15 * 1], 1;
	shlq [rbx + r15 * 1], 18;
	shlq [rbx + r15 * 2], 1;
	shlq [rbx + r15 * 2], 18;
	shlq [rbx + r15 * 4], 1;
	shlq [rbx + r15 * 4], 18;
	shlq [rbx + r15 * 8], 1;
	shlq [rbx + r15 * 8], 18;
	shlq [rsp + rax * 1], 1;
	shlq [rsp + rax * 1], 18;
	shlq [rsp + rax * 2], 1;
	shlq [rsp + rax * 2], 18;
	shlq [rsp + rax * 4], 1;
	shlq [rsp + rax * 4], 18;
	shlq [rsp + rax * 8], 1;
	shlq [rsp + rax * 8], 18;
	shlq [rsp + r15 * 1], 1;
	shlq [rsp + r15 * 1], 18;
	shlq [rsp + r15 * 2], 1;
	shlq [rsp + r15 * 2], 18;
	shlq [rsp + r15 * 4], 1;
	shlq [rsp + r15 * 4], 18;
	shlq [rsp + r15 * 8], 1;
	shlq [rsp + r15 * 8], 18;
	shlq [rbp + rax * 1], 1;
	shlq [rbp + rax * 1], 18;
	shlq [rbp + rax * 2], 1;
	shlq [rbp + rax * 2], 18;
	shlq [rbp + rax * 4], 1;
	shlq [rbp + rax * 4], 18;
	shlq [rbp + rax * 8], 1;
	shlq [rbp + rax * 8], 18;
	shlq [rbp + r15 * 1], 1;
	shlq [rbp + r15 * 1], 18;
	shlq [rbp + r15 * 2], 1;
	shlq [rbp + r15 * 2], 18;
	shlq [rbp + r15 * 4], 1;
	shlq [rbp + r15 * 4], 18;
	shlq [rbp + r15 * 8], 1;
	shlq [rbp + r15 * 8], 18;
	shlq [rsi + rax * 1], 1;
	shlq [rsi + rax * 1], 18;
	shlq [rsi + rax * 2], 1;
	shlq [rsi + rax * 2], 18;
	shlq [rsi + rax * 4], 1;
	shlq [rsi + rax * 4], 18;
	shlq [rsi + rax * 8], 1;
	shlq [rsi + rax * 8], 18;
	shlq [rsi + r15 * 1], 1;
	shlq [rsi + r15 * 1], 18;
	shlq [rsi + r15 * 2], 1;
	shlq [rsi + r15 * 2], 18;
	shlq [rsi + r15 * 4], 1;
	shlq [rsi + r15 * 4], 18;
	shlq [rsi + r15 * 8], 1;
	shlq [rsi + r15 * 8], 18;
	shlq [rdi + rax * 1], 1;
	shlq [rdi + rax * 1], 18;
	shlq [rdi + rax * 2], 1;
	shlq [rdi + rax * 2], 18;
	shlq [rdi + rax * 4], 1;
	shlq [rdi + rax * 4], 18;
	shlq [rdi + rax * 8], 1;
	shlq [rdi + rax * 8], 18;
	shlq [rdi + r15 * 1], 1;
	shlq [rdi + r15 * 1], 18;
	shlq [rdi + r15 * 2], 1;
	shlq [rdi + r15 * 2], 18;
	shlq [rdi + r15 * 4], 1;
	shlq [rdi + r15 * 4], 18;
	shlq [rdi + r15 * 8], 1;
	shlq [rdi + r15 * 8], 18;
	shlq [r8 + rax * 1], 1;
	shlq [r8 + rax * 1], 18;
	shlq [r8 + rax * 2], 1;
	shlq [r8 + rax * 2], 18;
	shlq [r8 + rax * 4], 1;
	shlq [r8 + rax * 4], 18;
	shlq [r8 + rax * 8], 1;
	shlq [r8 + rax * 8], 18;
	shlq [r8 + r15 * 1], 1;
	shlq [r8 + r15 * 1], 18;
	shlq [r8 + r15 * 2], 1;
	shlq [r8 + r15 * 2], 18;
	shlq [r8 + r15 * 4], 1;
	shlq [r8 + r15 * 4], 18;
	shlq [r8 + r15 * 8], 1;
	shlq [r8 + r15 * 8], 18;
	shlq [r9 + rax * 1], 1;
	shlq [r9 + rax * 1], 18;
	shlq [r9 + rax * 2], 1;
	shlq [r9 + rax * 2], 18;
	shlq [r9 + rax * 4], 1;
	shlq [r9 + rax * 4], 18;
	shlq [r9 + rax * 8], 1;
	shlq [r9 + rax * 8], 18;
	shlq [r9 + r15 * 1], 1;
	shlq [r9 + r15 * 1], 18;
	shlq [r9 + r15 * 2], 1;
	shlq [r9 + r15 * 2], 18;
	shlq [r9 + r15 * 4], 1;
	shlq [r9 + r15 * 4], 18;
	shlq [r9 + r15 * 8], 1;
	shlq [r9 + r15 * 8], 18;
	shlq [r10 + rax * 1], 1;
	shlq [r10 + rax * 1], 18;
	shlq [r10 + rax * 2], 1;
	shlq [r10 + rax * 2], 18;
	shlq [r10 + rax * 4], 1;
	shlq [r10 + rax * 4], 18;
	shlq [r10 + rax * 8], 1;
	shlq [r10 + rax * 8], 18;
	shlq [r10 + r15 * 1], 1;
	shlq [r10 + r15 * 1], 18;
	shlq [r10 + r15 * 2], 1;
	shlq [r10 + r15 * 2], 18;
	shlq [r10 + r15 * 4], 1;
	shlq [r10 + r15 * 4], 18;
	shlq [r10 + r15 * 8], 1;
	shlq [r10 + r15 * 8], 18;
	shlq [r11 + rax * 1], 1;
	shlq [r11 + rax * 1], 18;
	shlq [r11 + rax * 2], 1;
	shlq [r11 + rax * 2], 18;
	shlq [r11 + rax * 4], 1;
	shlq [r11 + rax * 4], 18;
	shlq [r11 + rax * 8], 1;
	shlq [r11 + rax * 8], 18;
	shlq [r11 + r15 * 1], 1;
	shlq [r11 + r15 * 1], 18;
	shlq [r11 + r15 * 2], 1;
	shlq [r11 + r15 * 2], 18;
	shlq [r11 + r15 * 4], 1;
	shlq [r11 + r15 * 4], 18;
	shlq [r11 + r15 * 8], 1;
	shlq [r11 + r15 * 8], 18;
	shlq [r12 + rax * 1], 1;
	shlq [r12 + rax * 1], 18;
	shlq [r12 + rax * 2], 1;
	shlq [r12 + rax * 2], 18;
	shlq [r12 + rax * 4], 1;
	shlq [r12 + rax * 4], 18;
	shlq [r12 + rax * 8], 1;
	shlq [r12 + rax * 8], 18;
	shlq [r12 + r15 * 1], 1;
	shlq [r12 + r15 * 1], 18;
	shlq [r12 + r15 * 2], 1;
	shlq [r12 + r15 * 2], 18;
	shlq [r12 + r15 * 4], 1;
	shlq [r12 + r15 * 4], 18;
	shlq [r12 + r15 * 8], 1;
	shlq [r12 + r15 * 8], 18;
	shlq [r13 + rax * 1], 1;
	shlq [r13 + rax * 1], 18;
	shlq [r13 + rax * 2], 1;
	shlq [r13 + rax * 2], 18;
	shlq [r13 + rax * 4], 1;
	shlq [r13 + rax * 4], 18;
	shlq [r13 + rax * 8], 1;
	shlq [r13 + rax * 8], 18;
	shlq [r13 + r15 * 1], 1;
	shlq [r13 + r15 * 1], 18;
	shlq [r13 + r15 * 2], 1;
	shlq [r13 + r15 * 2], 18;
	shlq [r13 + r15 * 4], 1;
	shlq [r13 + r15 * 4], 18;
	shlq [r13 + r15 * 8], 1;
	shlq [r13 + r15 * 8], 18;
	shlq [r14 + rax * 1], 1;
	shlq [r14 + rax * 1], 18;
	shlq [r14 + rax * 2], 1;
	shlq [r14 + rax * 2], 18;
	shlq [r14 + rax * 4], 1;
	shlq [r14 + rax * 4], 18;
	shlq [r14 + rax * 8], 1;
	shlq [r14 + rax * 8], 18;
	shlq [r14 + r15 * 1], 1;
	shlq [r14 + r15 * 1], 18;
	shlq [r14 + r15 * 2], 1;
	shlq [r14 + r15 * 2], 18;
	shlq [r14 + r15 * 4], 1;
	shlq [r14 + r15 * 4], 18;
	shlq [r14 + r15 * 8], 1;
	shlq [r14 + r15 * 8], 18;
	shlq [r15 + rax * 1], 1;
	shlq [r15 + rax * 1], 18;
	shlq [r15 + rax * 2], 1;
	shlq [r15 + rax * 2], 18;
	shlq [r15 + rax * 4], 1;
	shlq [r15 + rax * 4], 18;
	shlq [r15 + rax * 8], 1;
	shlq [r15 + rax * 8], 18;
	shlq [r15 + r15 * 1], 1;
	shlq [r15 + r15 * 1], 18;
	shlq [r15 + r15 * 2], 1;
	shlq [r15 + r15 * 2], 18;
	shlq [r15 + r15 * 4], 1;
	shlq [r15 + r15 * 4], 18;
	shlq [r15 + r15 * 8], 1;
	shlq [r15 + r15 * 8], 18;
	shlq rax, 1;
	shlq rax, 18;
	shlq rcx, 1;
	shlq rcx, 18;
	shlq rdx, 1;
	shlq rdx, 18;
	shlq rbx, 1;
	shlq rbx, 18;
	shlq rsp, 1;
	shlq rsp, 18;
	shlq rbp, 1;
	shlq rbp, 18;
	shlq rsi, 1;
	shlq rsi, 18;
	shlq rdi, 1;
	shlq rdi, 18;
	shlq r8, 1;
	shlq r8, 18;
	shlq r9, 1;
	shlq r9, 18;
	shlq r10, 1;
	shlq r10, 18;
	shlq r11, 1;
	shlq r11, 18;
	shlq r12, 1;
	shlq r12, 18;
	shlq r13, 1;
	shlq r13, 18;
	shlq r14, 1;
	shlq r14, 18;
	shlq r15, 1;
	shlq r15, 18;
	shlq [rax], 1;
	shlq [rax], 18;
	shlq [rax + 16], 1;
	shlq [rax + 16], 18;
	shlq [rax + 512], 1;
	shlq [rax + 512], 18;
	shlq [rcx], 1;
	shlq [rcx], 18;
	shlq [rcx + 16], 1;
	shlq [rcx + 16], 18;
	shlq [rcx + 512], 1;
	shlq [rcx + 512], 18;
	shlq [rdx], 1;
	shlq [rdx], 18;
	shlq [rdx + 16], 1;
	shlq [rdx + 16], 18;
	shlq [rdx + 512], 1;
	shlq [rdx + 512], 18;
	shlq [rbx], 1;
	shlq [rbx], 18;
	shlq [rbx + 16], 1;
	shlq [rbx + 16], 18;
	shlq [rbx + 512], 1;
	shlq [rbx + 512], 18;
	shlq [rsp], 1;
	shlq [rsp], 18;
	shlq [rsp + 16], 1;
	shlq [rsp + 16], 18;
	shlq [rsp + 512], 1;
	shlq [rsp + 512], 18;
	shlq [rbp], 1;
	shlq [rbp], 18;
	shlq [rbp + 16], 1;
	shlq [rbp + 16], 18;
	shlq [rbp + 512], 1;
	shlq [rbp + 512], 18;
	shlq [rsi], 1;
	shlq [rsi], 18;
	shlq [rsi + 16], 1;
	shlq [rsi + 16], 18;
	shlq [rsi + 512], 1;
	shlq [rsi + 512], 18;
	shlq [rdi], 1;
	shlq [rdi], 18;
	shlq [rdi + 16], 1;
	shlq [rdi + 16], 18;
	shlq [rdi + 512], 1;
	shlq [rdi + 512], 18;
	shlq [r8], 1;
	shlq [r8], 18;
	shlq [r8 + 16], 1;
	shlq [r8 + 16], 18;
	shlq [r8 + 512], 1;
	shlq [r8 + 512], 18;
	shlq [r9], 1;
	shlq [r9], 18;
	shlq [r9 + 16], 1;
	shlq [r9 + 16], 18;
	shlq [r9 + 512], 1;
	shlq [r9 + 512], 18;
	shlq [r10], 1;
	shlq [r10], 18;
	shlq [r10 + 16], 1;
	shlq [r10 + 16], 18;
	shlq [r10 + 512], 1;
	shlq [r10 + 512], 18;
	shlq [r11], 1;
	shlq [r11], 18;
	shlq [r11 + 16], 1;
	shlq [r11 + 16], 18;
	shlq [r11 + 512], 1;
	shlq [r11 + 512], 18;
	shlq [r12], 1;
	shlq [r12], 18;
	shlq [r12 + 16], 1;
	shlq [r12 + 16], 18;
	shlq [r12 + 512], 1;
	shlq [r12 + 512], 18;
	shlq [r13], 1;
	shlq [r13], 18;
	shlq [r13 + 16], 1;
	shlq [r13 + 16], 18;
	shlq [r13 + 512], 1;
	shlq [r13 + 512], 18;
	shlq [r14], 1;
	shlq [r14], 18;
	shlq [r14 + 16], 1;
	shlq [r14 + 16], 18;
	shlq [r14 + 512], 1;
	shlq [r14 + 512], 18;
	shlq [r15], 1;
	shlq [r15], 18;
	shlq [r15 + 16], 1;
	shlq [r15 + 16], 18;
	shlq [r15 + 512], 1;
	shlq [r15 + 512], 18;
	shlq [rip], 1;
	shlq [rip], 18;
	shlq [rip + 16], 1;
	shlq [rip + 16], 18;
	shlq [rip + 512], 1;
	shlq [rip + 512], 18;
	shlq [rax + rax * 1], 1;
	shlq [rax + rax * 1], 18;
	shlq [rax + rax * 2], 1;
	shlq [rax + rax * 2], 18;
	shlq [rax + rax * 4], 1;
	shlq [rax + rax * 4], 18;
	shlq [rax + rax * 8], 1;
	shlq [rax + rax * 8], 18;
	shlq [rax + r15 * 1], 1;
	shlq [rax + r15 * 1], 18;
	shlq [rax + r15 * 2], 1;
	shlq [rax + r15 * 2], 18;
	shlq [rax + r15 * 4], 1;
	shlq [rax + r15 * 4], 18;
	shlq [rax + r15 * 8], 1;
	shlq [rax + r15 * 8], 18;
	shlq [rcx + rax * 1], 1;
	shlq [rcx + rax * 1], 18;
	shlq [rcx + rax * 2], 1;
	shlq [rcx + rax * 2], 18;
	shlq [rcx + rax * 4], 1;
	shlq [rcx + rax * 4], 18;
	shlq [rcx + rax * 8], 1;
	shlq [rcx + rax * 8], 18;
	shlq [rcx + r15 * 1], 1;
	shlq [rcx + r15 * 1], 18;
	shlq [rcx + r15 * 2], 1;
	shlq [rcx + r15 * 2], 18;
	shlq [rcx + r15 * 4], 1;
	shlq [rcx + r15 * 4], 18;
	shlq [rcx + r15 * 8], 1;
	shlq [rcx + r15 * 8], 18;
	shlq [rdx + rax * 1], 1;
	shlq [rdx + rax * 1], 18;
	shlq [rdx + rax * 2], 1;
	shlq [rdx + rax * 2], 18;
	shlq [rdx + rax * 4], 1;
	shlq [rdx + rax * 4], 18;
	shlq [rdx + rax * 8], 1;
	shlq [rdx + rax * 8], 18;
	shlq [rdx + r15 * 1], 1;
	shlq [rdx + r15 * 1], 18;
	shlq [rdx + r15 * 2], 1;
	shlq [rdx + r15 * 2], 18;
	shlq [rdx + r15 * 4], 1;
	shlq [rdx + r15 * 4], 18;
	shlq [rdx + r15 * 8], 1;
	shlq [rdx + r15 * 8], 18;
	shlq [rbx + rax * 1], 1;
	shlq [rbx + rax * 1], 18;
	shlq [rbx + rax * 2], 1;
	shlq [rbx + rax * 2], 18;
	shlq [rbx + rax * 4], 1;
	shlq [rbx + rax * 4], 18;
	shlq [rbx + rax * 8], 1;
	shlq [rbx + rax * 8], 18;
	shlq [rbx + r15 * 1], 1;
	shlq [rbx + r15 * 1], 18;
	shlq [rbx + r15 * 2], 1;
	shlq [rbx + r15 * 2], 18;
	shlq [rbx + r15 * 4], 1;
	shlq [rbx + r15 * 4], 18;
	shlq [rbx + r15 * 8], 1;
	shlq [rbx + r15 * 8], 18;
	shlq [rsp + rax * 1], 1;
	shlq [rsp + rax * 1], 18;
	shlq [rsp + rax * 2], 1;
	shlq [rsp + rax * 2], 18;
	shlq [rsp + rax * 4], 1;
	shlq [rsp + rax * 4], 18;
	shlq [rsp + rax * 8], 1;
	shlq [rsp + rax * 8], 18;
	shlq [rsp + r15 * 1], 1;
	shlq [rsp + r15 * 1], 18;
	shlq [rsp + r15 * 2], 1;
	shlq [rsp + r15 * 2], 18;
	shlq [rsp + r15 * 4], 1;
	shlq [rsp + r15 * 4], 18;
	shlq [rsp + r15 * 8], 1;
	shlq [rsp + r15 * 8], 18;
	shlq [rbp + rax * 1], 1;
	shlq [rbp + rax * 1], 18;
	shlq [rbp + rax * 2], 1;
	shlq [rbp + rax * 2], 18;
	shlq [rbp + rax * 4], 1;
	shlq [rbp + rax * 4], 18;
	shlq [rbp + rax * 8], 1;
	shlq [rbp + rax * 8], 18;
	shlq [rbp + r15 * 1], 1;
	shlq [rbp + r15 * 1], 18;
	shlq [rbp + r15 * 2], 1;
	shlq [rbp + r15 * 2], 18;
	shlq [rbp + r15 * 4], 1;
	shlq [rbp + r15 * 4], 18;
	shlq [rbp + r15 * 8], 1;
	shlq [rbp + r15 * 8], 18;
	shlq [rsi + rax * 1], 1;
	shlq [rsi + rax * 1], 18;
	shlq [rsi + rax * 2], 1;
	shlq [rsi + rax * 2], 18;
	shlq [rsi + rax * 4], 1;
	shlq [rsi + rax * 4], 18;
	shlq [rsi + rax * 8], 1;
	shlq [rsi + rax * 8], 18;
	shlq [rsi + r15 * 1], 1;
	shlq [rsi + r15 * 1], 18;
	shlq [rsi + r15 * 2], 1;
	shlq [rsi + r15 * 2], 18;
	shlq [rsi + r15 * 4], 1;
	shlq [rsi + r15 * 4], 18;
	shlq [rsi + r15 * 8], 1;
	shlq [rsi + r15 * 8], 18;
	shlq [rdi + rax * 1], 1;
	shlq [rdi + rax * 1], 18;
	shlq [rdi + rax * 2], 1;
	shlq [rdi + rax * 2], 18;
	shlq [rdi + rax * 4], 1;
	shlq [rdi + rax * 4], 18;
	shlq [rdi + rax * 8], 1;
	shlq [rdi + rax * 8], 18;
	shlq [rdi + r15 * 1], 1;
	shlq [rdi + r15 * 1], 18;
	shlq [rdi + r15 * 2], 1;
	shlq [rdi + r15 * 2], 18;
	shlq [rdi + r15 * 4], 1;
	shlq [rdi + r15 * 4], 18;
	shlq [rdi + r15 * 8], 1;
	shlq [rdi + r15 * 8], 18;
	shlq [r8 + rax * 1], 1;
	shlq [r8 + rax * 1], 18;
	shlq [r8 + rax * 2], 1;
	shlq [r8 + rax * 2], 18;
	shlq [r8 + rax * 4], 1;
	shlq [r8 + rax * 4], 18;
	shlq [r8 + rax * 8], 1;
	shlq [r8 + rax * 8], 18;
	shlq [r8 + r15 * 1], 1;
	shlq [r8 + r15 * 1], 18;
	shlq [r8 + r15 * 2], 1;
	shlq [r8 + r15 * 2], 18;
	shlq [r8 + r15 * 4], 1;
	shlq [r8 + r15 * 4], 18;
	shlq [r8 + r15 * 8], 1;
	shlq [r8 + r15 * 8], 18;
	shlq [r9 + rax * 1], 1;
	shlq [r9 + rax * 1], 18;
	shlq [r9 + rax * 2], 1;
	shlq [r9 + rax * 2], 18;
	shlq [r9 + rax * 4], 1;
	shlq [r9 + rax * 4], 18;
	shlq [r9 + rax * 8], 1;
	shlq [r9 + rax * 8], 18;
	shlq [r9 + r15 * 1], 1;
	shlq [r9 + r15 * 1], 18;
	shlq [r9 + r15 * 2], 1;
	shlq [r9 + r15 * 2], 18;
	shlq [r9 + r15 * 4], 1;
	shlq [r9 + r15 * 4], 18;
	shlq [r9 + r15 * 8], 1;
	shlq [r9 + r15 * 8], 18;
	shlq [r10 + rax * 1], 1;
	shlq [r10 + rax * 1], 18;
	shlq [r10 + rax * 2], 1;
	shlq [r10 + rax * 2], 18;
	shlq [r10 + rax * 4], 1;
	shlq [r10 + rax * 4], 18;
	shlq [r10 + rax * 8], 1;
	shlq [r10 + rax * 8], 18;
	shlq [r10 + r15 * 1], 1;
	shlq [r10 + r15 * 1], 18;
	shlq [r10 + r15 * 2], 1;
	shlq [r10 + r15 * 2], 18;
	shlq [r10 + r15 * 4], 1;
	shlq [r10 + r15 * 4], 18;
	shlq [r10 + r15 * 8], 1;
	shlq [r10 + r15 * 8], 18;
	shlq [r11 + rax * 1], 1;
	shlq [r11 + rax * 1], 18;
	shlq [r11 + rax * 2], 1;
	shlq [r11 + rax * 2], 18;
	shlq [r11 + rax * 4], 1;
	shlq [r11 + rax * 4], 18;
	shlq [r11 + rax * 8], 1;
	shlq [r11 + rax * 8], 18;
	shlq [r11 + r15 * 1], 1;
	shlq [r11 + r15 * 1], 18;
	shlq [r11 + r15 * 2], 1;
	shlq [r11 + r15 * 2], 18;
	shlq [r11 + r15 * 4], 1;
	shlq [r11 + r15 * 4], 18;
	shlq [r11 + r15 * 8], 1;
	shlq [r11 + r15 * 8], 18;
	shlq [r12 + rax * 1], 1;
	shlq [r12 + rax * 1], 18;
	shlq [r12 + rax * 2], 1;
	shlq [r12 + rax * 2], 18;
	shlq [r12 + rax * 4], 1;
	shlq [r12 + rax * 4], 18;
	shlq [r12 + rax * 8], 1;
	shlq [r12 + rax * 8], 18;
	shlq [r12 + r15 * 1], 1;
	shlq [r12 + r15 * 1], 18;
	shlq [r12 + r15 * 2], 1;
	shlq [r12 + r15 * 2], 18;
	shlq [r12 + r15 * 4], 1;
	shlq [r12 + r15 * 4], 18;
	shlq [r12 + r15 * 8], 1;
	shlq [r12 + r15 * 8], 18;
	shlq [r13 + rax * 1], 1;
	shlq [r13 + rax * 1], 18;
	shlq [r13 + rax * 2], 1;
	shlq [r13 + rax * 2], 18;
	shlq [r13 + rax * 4], 1;
	shlq [r13 + rax * 4], 18;
	shlq [r13 + rax * 8], 1;
	shlq [r13 + rax * 8], 18;
	shlq [r13 + r15 * 1], 1;
	shlq [r13 + r15 * 1], 18;
	shlq [r13 + r15 * 2], 1;
	shlq [r13 + r15 * 2], 18;
	shlq [r13 + r15 * 4], 1;
	shlq [r13 + r15 * 4], 18;
	shlq [r13 + r15 * 8], 1;
	shlq [r13 + r15 * 8], 18;
	shlq [r14 + rax * 1], 1;
	shlq [r14 + rax * 1], 18;
	shlq [r14 + rax * 2], 1;
	shlq [r14 + rax * 2], 18;
	shlq [r14 + rax * 4], 1;
	shlq [r14 + rax * 4], 18;
	shlq [r14 + rax * 8], 1;
	shlq [r14 + rax * 8], 18;
	shlq [r14 + r15 * 1], 1;
	shlq [r14 + r15 * 1], 18;
	shlq [r14 + r15 * 2], 1;
	shlq [r14 + r15 * 2], 18;
	shlq [r14 + r15 * 4], 1;
	shlq [r14 + r15 * 4], 18;
	shlq [r14 + r15 * 8], 1;
	shlq [r14 + r15 * 8], 18;
	shlq [r15 + rax * 1], 1;
	shlq [r15 + rax * 1], 18;
	shlq [r15 + rax * 2], 1;
	shlq [r15 + rax * 2], 18;
	shlq [r15 + rax * 4], 1;
	shlq [r15 + rax * 4], 18;
	shlq [r15 + rax * 8], 1;
	shlq [r15 + rax * 8], 18;
	shlq [r15 + r15 * 1], 1;
	shlq [r15 + r15 * 1], 18;
	shlq [r15 + r15 * 2], 1;
	shlq [r15 + r15 * 2], 18;
	shlq [r15 + r15 * 4], 1;
	shlq [r15 + r15 * 4], 18;
	shlq [r15 + r15 * 8], 1;
	shlq [r15 + r15 * 8], 18;
      );
      jit.finalize();
      let mut buf = std::fs::File::create("tests/shlq_monoasm.bin").unwrap();
      buf.write_all(jit.as_slice()).unwrap();
  }
