CXX = g++
SRC = main.cpp ../lib/target/cxxbridge/rust-bindings-example/src/cpp.rs.cc
TARGET = myapp

CXX_LIB = $(wildcard $(HOME)/.cargo/registry/src/index.crates.io-*/cxx-*/include)
CXX_FLAGS = $(addprefix -I, $(CXX_LIB))
INCLUDE_DIR = -I../lib/target/cxxbridge/rust-bindings-example/src/ $(CXX_FLAGS)

RUST_LIB = ../lib/target/release/libcore.a

all: $(TARGET)

$(TARGET):clean $(RUST_LIB) $(SRC)
	$(CXX) -o $(TARGET) $(SRC) $(RUST_LIB) $(INCLUDE_DIR) -lpthread -lstdc++

clean:
	@rm -f $(TARGET)
