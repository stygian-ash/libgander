CC = cc
INC = -Iinclude/
CFLAGS = -Wall -Wextra -pedantic -std=c23 -g -Og -MD -MP $(INC)
LDFLAGS = 
LDLIBS = 

SRC = 
OBJ = $(patsubst src/%.c, build/%.o, $(SRC))
DEP = $(OBJ:.o=.d)

all: build build/libgander.so

build:
	mkdir -p build

build/libgander.so: $(OBJ)
	$(CC) $(LDFLAGS) -shared -o $@ $< $(LDLIBS)

build/%.o: src/%.c
	$(CC) $(CFLAGS) -fPIC -c -o $@ $<


EX_SRC = examples/process.c
EX_BIN = $(patsubst examples/%.c, build/examples/%, $(EX_SRC))
EX_DEP = $(patsubst %, %.d, $(EX_BIN))

examples: build/examples $(EX_BIN)

build/examples:
	mkdir -p build/examples

build/examples/%: examples/%.c build/libgander.so
	$(CC) $(CFLAGS) $(LDFLAGS) -Lbuild -o $@ $< $(LDLIBS) -lgander

clean:
	rm -f $(OBJ) $(DEP) $(EX_BIN) $(EX_DEP) build/libgander.so

.PHONY: examples

-include $(DEP)
-include $(EX_DEP)
