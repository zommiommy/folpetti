build:
	cross run --target riscv64gc-unknown-linux-gnu
view:
	llvm-objdump -d target/riscv64gc-unknown-linux-gnu/debug/test_fuzz
run:
	qemu-riscv64 -singlestep -g 666 -L /usr/riscv64-linux-gnu/ target/riscv64gc-unknown-linux-gnu/debug/test_fuzz