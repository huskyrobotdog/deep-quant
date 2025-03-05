use anyhow::Result;
use deep_quant_engine::{config::load as load_config, log::init as init_log};
use pyo3::{prelude::*, types::PyTuple};

#[pymodule]
fn deep_quant_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_engine, m)?)?;
    m.add_function(wrap_pyfunction!(show_trace, m)?)?;
    m.add_function(wrap_pyfunction!(show_debug, m)?)?;
    m.add_function(wrap_pyfunction!(show_info, m)?)?;
    m.add_function(wrap_pyfunction!(show_warn, m)?)?;
    m.add_function(wrap_pyfunction!(show_error, m)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(
    name = "init_engine",
    signature = ()
)]
fn init_engine() -> Result<()> {
    let config = load_config()?;
    if let Err(err) = init_log(&config.log) {
        eprintln!("初始化日志失败: {}", err);
    }
    Ok(())
}

#[pyfunction]
#[pyo3(
    name = "show_trace",
    signature = (*args)
)]
fn show_trace(args: &Bound<'_, PyTuple>) {
    tracing::trace!(
        "{}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[pyfunction]
#[pyo3(
    name = "show_debug",
    signature = (*args)
)]
fn show_debug(args: &Bound<'_, PyTuple>) {
    tracing::debug!(
        "{}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[pyfunction]
#[pyo3(
    name = "show_info",
    signature = (*args)
)]
fn show_info(args: &Bound<'_, PyTuple>) {
    tracing::info!(
        "{}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[pyfunction]
#[pyo3(
    name = "show_warn",
    signature = (*args)
)]
fn show_warn(args: &Bound<'_, PyTuple>) {
    tracing::warn!(
        "{}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[pyfunction]
#[pyo3(
    name = "show_error",
    signature = (*args)
)]
fn show_error(args: &Bound<'_, PyTuple>) {
    tracing::error!(
        "{}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
