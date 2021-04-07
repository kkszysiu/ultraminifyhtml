#![allow(clippy::into_iter_on_ref)]
extern crate cpython;

use cpython::{PyDict, PyString, PyList, PyResult, PyErr, Python};

use cpython::PythonObject;
use cpython::ToPyObject;

use cpython::*;
use minify_html::{Cfg, Error, in_place as minify_html_native};
use std::str::from_utf8_unchecked;

py_exception!(libultraminifyhtml, MinifyError);

fn minify(py: Python, code: PyString, minify_js: PyBool, minify_css: PyBool) -> PyResult<String> {
    let mut code = code.to_string(py)?.to_string().into_bytes();
    let minify_js = minify_js.is_true();
    let minify_css = minify_css.is_true();

    match minify_html_native(&mut code, &Cfg {
        minify_js,
        minify_css,
    }) {
        Ok(out_len) => Ok(unsafe { from_utf8_unchecked(&code[0..out_len]).to_string() }),
        Err(Error { error_type, position }) => Err(PyErr::new::<MinifyError, _>(py, format!("{} [Character {}]", error_type.message(), position))),
    }
}

// #[pyfunction(py_args="*", minify_js="false", minify_css="false")]
// fn minify(code: String, minify_js: bool, minify_css: bool) -> PyResult<String> {
//     let mut code = code.into_bytes();
//     match minify_html_native(&mut code, &Cfg {
//         minify_js,
//         minify_css,
//     }) {
//         Ok(out_len) => Ok(unsafe { from_utf8_unchecked(&code[0..out_len]).to_string() }),
//         Err(Error { error_type, position }) => Err(PySyntaxError::new_err(format!("{} [Character {}]", error_type.message(), position))),
//     }
// }

py_module_initializer!(
    libultraminifyhtml,
    initlibultraminifyhtml,
    PyInit_libultraminifyhtml,
    |py, m| {
        m.add(py, "__doc__", "An HTML minifier meticulously optimised for both speed and effectiveness written in Rust then ported to Python")?;
        m.add(py, "minify", py_fn!(py, minify(code: PyString, minify_js: PyBool, minify_css: PyBool)))?;
        m.add(py, "MinifyError", py.get_type::<MinifyError>())?;
        Ok(())
    }
);
