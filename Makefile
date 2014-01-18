bin_path=bin
glfw_path=src/support/glfw-rs
glfw_lib_path=$(glfw_path)/lib
all:
	mkdir -p $(bin_path)
	rustc -L $(glfw_lib_path) --out-dir $(bin_path) src/main/arkoh.rc
libs:
	cd $(glfw_path); cmake .; make lib
