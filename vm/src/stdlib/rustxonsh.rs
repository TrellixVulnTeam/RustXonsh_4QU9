pub(crate) use decl::make_module;

#[pymodule(name = "rustxonsh")]
mod decl {
    use crate::{
        builtins::PyStrRef,
        pyobject::{PyResult, PySequence},
        vm::VirtualMachine,
    };
    use std::process::Command;

    #[pyfunction]
    fn subproc_uncaptured(
        cmd: PyStrRef,
        args: PySequence<PyStrRef>,
        vm: &VirtualMachine,
    ) -> PyResult {
        match Command::new(cmd.as_str())
            .args(args.into_vec().iter().map(|s| s.as_str()))
            .status()
        {
            Ok(_) => Ok(vm.ctx.none()),
            Err(e) => Err(vm.new_os_error(e.to_string())),
        }
    }
}
