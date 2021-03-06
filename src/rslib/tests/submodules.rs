#![allow(dead_code)]

extern crate libc;
extern crate cpython;
extern crate rustypy;

mod test_package;
use test_package::rustypy_pybind::PyModules;
use cpython::{Python};

#[test]
fn submodules() {
	let gil = Python::acquire_gil();
	let py = gil.python();
	let test_package: PyModules = PyModules::new(py);
	test_package.root_module_1.root_module_1(py);
	test_package.root_module_2.root_module_2(py);
	test_package.firstdir.call_from_first.first_module(py);
	test_package.firstdir.subfirstdir.call_from_subfirst.subfirst_module(py);
}

/*#[no_mangle]
pub extern fn callback_func(fname: *const c_char) {
	let fname = unsafe {
        assert!(!fname.is_null());
        CStr::from_ptr(fname)
    };
	let fname = fname.to_str();
	match fname {
		Ok(s) => {
			match s {
				"check_recs" => check_recs(),
				_ => {},
			};
		},
		Err(e) => panic!("{}, call from Python is invalid: \
						  not valid string", e),
	};
}*/
