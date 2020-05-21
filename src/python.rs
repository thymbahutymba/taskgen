use crate::opt::TaskgenOpt;
use cpython::{PyModule, PyResult, Python};
use ndarray::Array1;

const TASKGEN_STR: &'static str = include_str!("../taskgen.py");

fn module_from_str(py: Python<'_>, name: &str, source: &str) -> PyResult<PyModule> {
    // Create a new empty module named
    let module = PyModule::new(py, name).unwrap();

    module.add(py, "__builtins__", py.eval("__builtins__", None, None)?)?;

    // Run the contents of `taskgen_str` in the context of `module` to populate the module
    py.run(source, Some(&module.dict(py)), None).unwrap();

    Ok(module)
}

pub fn get_with_opt(opt: &TaskgenOpt) -> PyResult<(Vec<Array1<f32>>, Vec<Array1<f32>>)> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let taskgen = module_from_str(py, "taskgen", TASKGEN_STR)?;

    let options = (opt.n, opt.util, opt.nsets);
    let x: Vec<Array1<f32>> = taskgen
        .call(py, "StaffordRandFixedSum", options, None)?
        .extract::<Vec<Vec<f32>>>(py)?
        .iter()
        .map(|e| ndarray::arr1(e))
        .collect();

    let options = (
        opt.n,
        opt.nsets,
        opt.permin,
        opt.permax.unwrap_or(opt.permin),
        opt.pergran.unwrap_or(opt.permin),
        &opt.perdist,
    );
    let periods: Vec<Array1<f32>> = taskgen
        .call(py, "gen_periods", options, None)?
        .extract::<Vec<Vec<f32>>>(py)?
        .iter()
        .map(|e| ndarray::arr1(e))
        .collect();

    Ok((x, periods))
}
