bin_path=bin
glfw_path=src/support/glfw-rs
glfw_lib_path=$(glfw_path)/lib
gl_path=src/support/gl-rs
gl_lib_path=$(gl_path)/src/gl
nalgebra=src/support/nalgebra
nalgebra_lib_path=$(nalgebra)/lib

all:
	mkdir -p $(bin_path)
	rustc -L $(glfw_lib_path) -L $(gl_lib_path) -L $(nalgebra_lib_path) --out-dir $(bin_path) src/main/arkoh.rc
examples:
	rustc -L $(glfw_lib_path) -L $(gl_lib_path) -L$(nalgebra_lib_path) -L $(bin_path) --out-dir $(bin_path) src/example/window.rs
	rustc -L $(glfw_lib_path) -L $(gl_lib_path) -L$(nalgebra_lib_path) -L $(bin_path) --out-dir $(bin_path) src/example/event_handle.rs
	rustc -L $(glfw_lib_path) -L $(gl_lib_path) -L$(nalgebra_lib_path) -L $(bin_path) --out-dir $(bin_path) src/example/cube.rs
libs:
	cd $(glfw_path); cmake .; make lib
	cd src/support/gl-rs; rustc --opt-level=3 src/gl/lib.rs
	make -C src/support/nalgebra
