# Find all .hs files in current directory
HS_FILES := $(wildcard *.hs)
# Generate list of target .comb files
COMB_FILES := $(HS_FILES:.hs=.comb)
RS_ROM_FILES := $(HS_FILES:.hs=.rs)
CH_ROM_FILES := $(HS_FILES:.hs=.scala)
# Set Chisel project path
MUTATOR_PATH := ~/workspace/sc-machine/src/main/scala/benchmarks
HDL_PATH := ~/workspace/ouros-chisel/src/main/scala/benchmarks
SIMULATOR_PATH := ~/workspace/ouros-simulator/src/hardware/ouros/benchmarks

# Rule to process all .hs files
all-rs: $(RS_ROM_FILES)

all-ch: $(CH_ROM_FILES)

# Rule to handle single argument
%:
	#mhs -i../lib $@ -o$@.comb
	mhs -i../lib $@ -o$@.rs

# Pattern rule to convert .hs to .rs ROM
%.rs: %.hs
	mhs -i../lib $* -o$(shell echo $@ | tr '[:upper:]' '[:lower:]')
	
%.scala: %.hs
	mhs -i../lib $* -o$@

# Pattern rule to convert .hs to .comb
%.comb: %.hs
	mhs -i../lib $* -o$@

# Clean rule to remove generated files
clean:
	rm -f *.comb *.rs *scala

# Rule to clean and reprocess all files
rerun: clean all

# Rule for installing binary to interpreter project
install-rs:
	cp *.rs $(SIMULATOR_PATH)/
	
install-ch:
	cp *.scala $(HDL_PATH)/

.PHONY: help clean all-rs all-ch rerun % install-rs install-ch
