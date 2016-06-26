
//! This file has been generated by rustypy and contains bindings for Python.
//! rustypy version: alpha
//! Python implementation build version: cpython-35

// IMPORTANT: This file will be replaced on new rustypy calls.
// Don't write on it directly, rather call functions from an other file.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    unused_variables)]

use libc::{c_long, c_double};
use cpython::{Python, ToPyObject, FromPyObject, ObjectProtocol,
              PythonObjectWithCheckedDowncast, PyObject, PyModule, PyErr,
              PyDict, NoArgs};
use std::collections::HashMap;

fn handle_import_error(err: PyErr) {
    println!("failed to load Python module, reason:
             {}", err.pvalue.unwrap());
}

/// Binds for Python module `primitives`
pub struct PmVMSV {
    module: PyModule
}

impl PmVMSV {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<PmVMSV> {
        let module = py.import("test_package.basics.primitives");
        match module {
            Ok(m) => Some(
                PmVMSV {
                    module: m,
                }),
            Err(exception) => {
                handle_import_error(exception);
                None
            }
        }
    }

    pub fn rust_bind_float_func(&self, py: Python,
        float_arg: c_double
    ) -> c_double {

        let func = self.module.get(py, "rust_bind_float_func").unwrap();
        let float_arg = float_arg.to_py_object(py);
        let result = func.call(py, (float_arg, ), None).unwrap();
        let KmORSQ = result.extract::<c_double>(py).unwrap();

        return KmORSQ

    }

    pub fn rust_bind_bool_func(&self, py: Python,
        bool_arg: bool
    ) -> bool {

        let func = self.module.get(py, "rust_bind_bool_func").unwrap();
        let bool_arg = bool_arg.to_py_object(py);
        let result = func.call(py, (bool_arg, ), None).unwrap();
        let jGWSYD = result.extract::<bool>(py).unwrap();

        return jGWSYD

    }

    pub fn rust_bind_tuple3(&self, py: Python,
        tup_arg1: c_double,
        tup_arg2: bool
    ) -> (c_double, bool) {

        let func = self.module.get(py, "rust_bind_tuple3").unwrap();
        let tup_arg1 = tup_arg1.to_py_object(py);
        let tup_arg2 = tup_arg2.to_py_object(py);
        let result = func.call(py, (tup_arg1, tup_arg2, ), None).unwrap();
            let UEYnjk = result.get_item(py,0).unwrap().extract::<c_double>(py).unwrap();
            let jmbiUp = result.get_item(py,1).unwrap().extract::<bool>(py).unwrap();

        return (UEYnjk, jmbiUp)

    }

    pub fn rust_bind_str_func(&self, py: Python,
        str_arg: String
    ) -> String {

        let func = self.module.get(py, "rust_bind_str_func").unwrap();
        let str_arg = str_arg.to_py_object(py);
        let result = func.call(py, (str_arg, ), None).unwrap();
        let QIhjUI = result.extract::<String>(py).unwrap();

        return QIhjUI

    }

    pub fn rust_bind_tuple1(&self, py: Python,
        tup_arg: (String, c_long)
    ) -> (String, c_long) {

        let func = self.module.get(py, "rust_bind_tuple1").unwrap();
        let tup_arg = tup_arg.to_py_object(py);
        let result = func.call(py, (tup_arg, ), None).unwrap();
            let EtQeCW = result.get_item(py,0).unwrap().extract::<String>(py).unwrap();
            let ybnHxa = result.get_item(py,1).unwrap().extract::<c_long>(py).unwrap();

        return (EtQeCW, ybnHxa)

    }

    pub fn rust_bind_tuple2(&self, py: Python,
        tup_arg: (String, bool)
    ) -> (String, bool) {

        let func = self.module.get(py, "rust_bind_tuple2").unwrap();
        let tup_arg = tup_arg.to_py_object(py);
        let result = func.call(py, (tup_arg, ), None).unwrap();
            let LVkFAO = result.get_item(py,0).unwrap().extract::<String>(py).unwrap();
            let NCFihb = result.get_item(py,1).unwrap().extract::<bool>(py).unwrap();

        return (LVkFAO, NCFihb)

    }

    pub fn rust_bind_int_func(&self, py: Python,
        int_arg: c_long
    ) -> c_long {

        let func = self.module.get(py, "rust_bind_int_func").unwrap();
        let int_arg = int_arg.to_py_object(py);
        let result = func.call(py, (int_arg, ), None).unwrap();
        let zEPdvw = result.extract::<c_long>(py).unwrap();

        return zEPdvw

    }

}

/// Binds for Python module `nested_types`
pub struct sMAKMj {
    module: PyModule
}

impl sMAKMj {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<sMAKMj> {
        let module = py.import("test_package.basics.nested_types");
        match module {
            Ok(m) => Some(
                sMAKMj {
                    module: m,
                }),
            Err(exception) => {
                handle_import_error(exception);
                None
            }
        }
    }

    pub fn cmpd_dict_and_ls(&self, py: Python,

    ) -> HashMap<c_long, Vec<c_double>> {

        let func = self.module.get(py, "cmpd_dict_and_ls").unwrap();

        let result = func.call(py, NoArgs, None).unwrap();
        let mut QbUwtr = HashMap::new();
        let QbUwtr_d: PyDict = PyDict::downcast_from(py, result).unwrap();
        for (key, value) in QbUwtr_d.items(py) {
            let dict_key = key.extract::<c_long>(py).unwrap();
            let mut KCMRZr = vec![];
            for e in value.iter(py).unwrap() {
                KCMRZr.push(e.unwrap().extract::<c_double>(py).unwrap());
            };
            let dict_value = KCMRZr;
            QbUwtr.insert(dict_key, dict_value);
        };

        return QbUwtr

    }

    pub fn cmpd_dict(&self, py: Python,

    ) -> HashMap<String, HashMap<c_long, (String, bool)>> {

        let func = self.module.get(py, "cmpd_dict").unwrap();

        let result = func.call(py, NoArgs, None).unwrap();
        let mut LqGMfy = HashMap::new();
        let LqGMfy_d: PyDict = PyDict::downcast_from(py, result).unwrap();
        for (key, value) in LqGMfy_d.items(py) {
            let dict_key = key.extract::<String>(py).unwrap();
            let mut YjPCxU = HashMap::new();
            let YjPCxU_d: PyDict = PyDict::downcast_from(py, value).unwrap();
            for (key, value) in YjPCxU_d.items(py) {
                let dict_key = key.extract::<c_long>(py).unwrap();
                let kNEvGV = value;
                    let pwbOKx = kNEvGV.get_item(py,0).unwrap().extract::<String>(py).unwrap();
                    let uWJNCL = kNEvGV.get_item(py,1).unwrap().extract::<bool>(py).unwrap();
                let dict_value = (pwbOKx, uWJNCL);
                YjPCxU.insert(dict_key, dict_value);
            };
            let dict_value = YjPCxU;
            LqGMfy.insert(dict_key, dict_value);
        };

        return LqGMfy

    }

    pub fn cmpd_tuple(&self, py: Python,
        tup_arg1: (c_long, (c_double, bool))
    ) -> (c_long, (String, bool), c_double) {

        let func = self.module.get(py, "cmpd_tuple").unwrap();
        let tup_arg1 = tup_arg1.to_py_object(py);
        let result = func.call(py, (tup_arg1, ), None).unwrap();
            let wZdppw = result.get_item(py,0).unwrap().extract::<c_long>(py).unwrap();
            let nmsXZG = result.get_item(py, 1).unwrap();
                let Zeyvaa = nmsXZG.get_item(py,0).unwrap().extract::<String>(py).unwrap();
                let NTUeEZ = nmsXZG.get_item(py,1).unwrap().extract::<bool>(py).unwrap();
            let kZZNVs = result.get_item(py,2).unwrap().extract::<c_double>(py).unwrap();

        return (wZdppw, (Zeyvaa, NTUeEZ), kZZNVs)

    }

    pub fn dict2(&self, py: Python,
        dict_arg: HashMap<String, (String, bool)>
    ) -> HashMap<String, (String, bool)> {

        let func = self.module.get(py, "dict2").unwrap();
        let dict_arg = dict_arg.to_py_object(py);
        let result = func.call(py, (dict_arg, ), None).unwrap();
        let mut eKgdiR = HashMap::new();
        let eKgdiR_d: PyDict = PyDict::downcast_from(py, result).unwrap();
        for (key, value) in eKgdiR_d.items(py) {
            let dict_key = key.extract::<String>(py).unwrap();
            let FgInEo = value;
                let rgKPgS = FgInEo.get_item(py,0).unwrap().extract::<String>(py).unwrap();
                let mGGapG = FgInEo.get_item(py,1).unwrap().extract::<bool>(py).unwrap();
            let dict_value = (rgKPgS, mGGapG);
            eKgdiR.insert(dict_key, dict_value);
        };

        return eKgdiR

    }

    pub fn dict1(&self, py: Python,
        dict_arg: HashMap<String, c_long>
    ) -> HashMap<String, c_long> {

        let func = self.module.get(py, "dict1").unwrap();
        let dict_arg = dict_arg.to_py_object(py);
        let result = func.call(py, (dict_arg, ), None).unwrap();
        let mut LQBmEq = HashMap::new();
        let LQBmEq_d: PyDict = PyDict::downcast_from(py, result).unwrap();
        for (key, value) in LQBmEq_d.items(py) {
            let dict_key = key.extract::<String>(py).unwrap();
            let dict_value = value.extract::<c_long>(py).unwrap();
            LQBmEq.insert(dict_key, dict_value);
        };

        return LQBmEq

    }

    pub fn cmpd_list(&self, py: Python,
        arg1: Vec<(c_long, bool)>,
        arg2: Vec<c_long>
    ) -> Vec<(Vec<c_long>, c_double)> {

        let func = self.module.get(py, "cmpd_list").unwrap();
        let arg1 = arg1.to_py_object(py);
        let arg2 = arg2.to_py_object(py);
        let result = func.call(py, (arg1, arg2, ), None).unwrap();
        let mut bizvCz = vec![];
        for e in result.iter(py).unwrap() {
            let zllstd = e.unwrap();
                let gLfodb = zllstd.get_item(py, 0).unwrap();
                let mut VeakXe = vec![];
                for e in gLfodb.iter(py).unwrap() {
                    VeakXe.push(e.unwrap().extract::<c_long>(py).unwrap());
                };
                let oELWBf = zllstd.get_item(py,1).unwrap().extract::<c_double>(py).unwrap();
            let zllstd = (VeakXe, oELWBf);
            bizvCz.push(zllstd);
        };

        return bizvCz

    }

    pub fn generic2(&self, py: Python,
        g_arg: Vec<PyObject>
    ) -> Vec<PyObject> {

        let func = self.module.get(py, "generic2").unwrap();
        let g_arg = g_arg.to_py_object(py);
        let result = func.call(py, (g_arg, ), None).unwrap();
        let mut AkoRmd = vec![];
        for e in result.iter(py).unwrap() {
            AkoRmd.push(e.unwrap().extract::<PyObject>(py).unwrap());
        };

        return AkoRmd

    }

    pub fn cmpd_list_and_tuple(&self, py: Python,
        ls_arg: Vec<((String, bool), PyObject)>
    ) -> Vec<(c_long, bool)> {

        let func = self.module.get(py, "cmpd_list_and_tuple").unwrap();
        let ls_arg = ls_arg.to_py_object(py);
        let result = func.call(py, (ls_arg, ), None).unwrap();
        let mut OIZjKV = vec![];
        for e in result.iter(py).unwrap() {
            let IkAzBA = e.unwrap();
                let NPrwOi = IkAzBA.get_item(py,0).unwrap().extract::<c_long>(py).unwrap();
                let sFmKdx = IkAzBA.get_item(py,1).unwrap().extract::<bool>(py).unwrap();
            let IkAzBA = (NPrwOi, sFmKdx);
            OIZjKV.push(IkAzBA);
        };

        return OIZjKV

    }

    pub fn cmpd_list_and_dict(&self, py: Python,

    ) -> Vec<HashMap<c_long, (String, bool)>> {

        let func = self.module.get(py, "cmpd_list_and_dict").unwrap();

        let result = func.call(py, NoArgs, None).unwrap();
        let mut maJMrD = vec![];
        for e in result.iter(py).unwrap() {
            let qwBhpy = e.unwrap();
            let mut XZuIGT = HashMap::new();
            let XZuIGT_d: PyDict = PyDict::downcast_from(py, qwBhpy).unwrap();
            for (key, value) in XZuIGT_d.items(py) {
                let dict_key = key.extract::<c_long>(py).unwrap();
                let xSnAWN = value;
                    let oXdmtX = xSnAWN.get_item(py,0).unwrap().extract::<String>(py).unwrap();
                    let WpdMre = xSnAWN.get_item(py,1).unwrap().extract::<bool>(py).unwrap();
                let dict_value = (oXdmtX, WpdMre);
                XZuIGT.insert(dict_key, dict_value);
            };
            maJMrD.push(XZuIGT);
        };

        return maJMrD

    }

    pub fn generic1(&self, py: Python,
        g_arg: PyObject
    ) -> PyObject {

        let func = self.module.get(py, "generic1").unwrap();
        let g_arg = g_arg.to_py_object(py);
        let result = func.call(py, (g_arg, ), None).unwrap();
        let jdEqwI = result.extract::<PyObject>(py).unwrap();

        return jdEqwI

    }

    pub fn list1(&self, py: Python,
        ls_arg: Vec<(c_double, bool)>
    ) -> Vec<String> {

        let func = self.module.get(py, "list1").unwrap();
        let ls_arg = ls_arg.to_py_object(py);
        let result = func.call(py, (ls_arg, ), None).unwrap();
        let mut zTkjYc = vec![];
        for e in result.iter(py).unwrap() {
            zTkjYc.push(e.unwrap().extract::<String>(py).unwrap());
        };

        return zTkjYc

    }

}

/// Struct for folder `basics`
pub struct KmSAXH {
    pub primitives: PmVMSV,
    pub nested_types: sMAKMj,

}

impl KmSAXH {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<KmSAXH> {
        Some(KmSAXH {
            primitives: PmVMSV::new(py).unwrap(),
            nested_types: sMAKMj::new(py).unwrap(),

        })
    }
}

/// Binds for Python module `call_from_subfirst`
pub struct AiVurl {
    module: PyModule
}

impl AiVurl {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<AiVurl> {
        let module = py.import("test_package.firstdir.subfirstdir.call_from_subfirst");
        match module {
            Ok(m) => Some(
                AiVurl {
                    module: m,
                }),
            Err(exception) => {
                handle_import_error(exception);
                None
            }
        }
    }

    pub fn subfirst_module(&self, py: Python,

    )  {

        let func = self.module.get(py, "subfirst_module").unwrap();

        let result = func.call(py, NoArgs, None).unwrap();
        let jRYxBZ = result.extract::<PyObject>(py).unwrap();

        return 

    }

}

/// Struct for folder `subfirstdir`
pub struct rroiXx {
    pub call_from_subfirst: AiVurl,

}

impl rroiXx {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<rroiXx> {
        Some(rroiXx {
            call_from_subfirst: AiVurl::new(py).unwrap(),

        })
    }
}

/// Binds for Python module `call_from_first`
pub struct TYcxxv {
    module: PyModule
}

impl TYcxxv {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<TYcxxv> {
        let module = py.import("test_package.firstdir.call_from_first");
        match module {
            Ok(m) => Some(
                TYcxxv {
                    module: m,
                }),
            Err(exception) => {
                handle_import_error(exception);
                None
            }
        }
    }

    pub fn first_module(&self, py: Python,

    )  {

        let func = self.module.get(py, "first_module").unwrap();

        let result = func.call(py, NoArgs, None).unwrap();
        let MXIaVi = result.extract::<PyObject>(py).unwrap();

        return 

    }

}

/// Struct for folder `firstdir`
pub struct OVyoxS {
    pub subfirstdir: rroiXx,
    pub call_from_first: TYcxxv,

}

impl OVyoxS {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<OVyoxS> {
        Some(OVyoxS {
            subfirstdir: rroiXx::new(py).unwrap(),
            call_from_first: TYcxxv::new(py).unwrap(),

        })
    }
}

/// Binds for Python module `root_module_1`
pub struct OZmceF {
    module: PyModule
}

impl OZmceF {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<OZmceF> {
        let module = py.import("test_package.root_module_1");
        match module {
            Ok(m) => Some(
                OZmceF {
                    module: m,
                }),
            Err(exception) => {
                handle_import_error(exception);
                None
            }
        }
    }

    pub fn root_module_1(&self, py: Python,

    )  {

        let func = self.module.get(py, "root_module_1").unwrap();

        let result = func.call(py, NoArgs, None).unwrap();
        let hgrmJl = result.extract::<PyObject>(py).unwrap();

        return 

    }

}

/// Binds for Python module `root_module_2`
pub struct JbZokt {
    module: PyModule
}

impl JbZokt {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<JbZokt> {
        let module = py.import("test_package.root_module_2");
        match module {
            Ok(m) => Some(
                JbZokt {
                    module: m,
                }),
            Err(exception) => {
                handle_import_error(exception);
                None
            }
        }
    }

    pub fn root_module_2(&self, py: Python,

    )  {

        let func = self.module.get(py, "root_module_2").unwrap();

        let result = func.call(py, NoArgs, None).unwrap();
        let SZBIhd = result.extract::<PyObject>(py).unwrap();

        return 

    }

}

/// Struct for folder `test_package`
pub struct CKonls {
    pub basics: KmSAXH,
    pub firstdir: OVyoxS,
    pub root_module_1: OZmceF,
    pub root_module_2: JbZokt,

}

impl CKonls {
    /// Loads the module instance to the interpreter
    pub fn new(py: Python) -> Option<CKonls> {
        Some(CKonls {
            basics: KmSAXH::new(py).unwrap(),
            firstdir: OVyoxS::new(py).unwrap(),
            root_module_1: OZmceF::new(py).unwrap(),
            root_module_2: JbZokt::new(py).unwrap(),

        })
    }
}

/// Python module manager
pub struct PyModules {
    pub basics: KmSAXH,
    pub firstdir: OVyoxS,
    pub root_module_1: OZmceF,
    pub root_module_2: JbZokt,

}

impl PyModules {
    pub fn new(py: Python) -> PyModules {
        PyModules {
            basics: KmSAXH::new(py).unwrap(),
            firstdir: OVyoxS::new(py).unwrap(),
            root_module_1: OZmceF::new(py).unwrap(),
            root_module_2: JbZokt::new(py).unwrap(),

        }
    }
}
