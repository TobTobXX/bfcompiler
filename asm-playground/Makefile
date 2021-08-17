# Thanks HubDanDevMan

AS=nasm
ASMFLAGS=-f elf64
IF=io.asm
OB=ir.o
OF=a.out

build: $(IF)
	$(AS) $(ASMFLAGS) $(IF) -o $(OB)
	ld -o $(OF) $(OB)
	chmod +x $(OF)

run: make
	./$(OF)

clean: $(OF)
	rm $(OB) $(OF)

