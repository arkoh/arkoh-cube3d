bin_path=bin
glfw_path=src/support/glfw-rs
glfw_lib_path=$(glfw_path)/lib
all:
	mkdir -p $(bin_path)
	rustc -L $(glfw_lib_path) --out-dir $(bin_path) src/main/arkoh.rc
examples:
	rustc -L $(glfw_lib_path) -L $(bin_path) --out-dir $(bin_path) src/example/window.rs

libs:
	cd $(glfw_path); cmake .; make lib
