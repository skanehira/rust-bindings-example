C++ = g++
SRC = main.cpp
TARGET = main

INCLUDE_DIR = -I./

RUST_LIB = ../lib/target/release/libcore.a

all: $(TARGET)

$(TARGET):clean $(RUST_LIB) $(SRC)
	$(C++) -o $(TARGET) $(SRC) $(RUST_LIB) $(INCLUDE_DIR) -lpthread -lstdc++

clean:
	@rm -f $(TARGET)
