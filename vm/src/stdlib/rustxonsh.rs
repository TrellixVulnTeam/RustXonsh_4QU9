pub(crate) use decl::make_module;

#[pymodule(name = "rustxonsh")]
mod decl {
    use crate::{
        builtins::PyStrRef,
        pyobject::{PyResult, PySequence},
        vm::VirtualMachine,
    };
    use std::process::{Command, Stdio};

    fn new_command(cmd: PyStrRef, args: PySequence<PyStrRef>) -> Command {
        let mut command = Command::new(cmd.as_str());
        command.args(args.into_vec().iter().map(|s| s.as_str()));
        command
    }

    #[pyfunction]
    fn subproc_uncaptured(
        cmd: PyStrRef,
        args: PySequence<PyStrRef>,
        vm: &VirtualMachine,
    ) -> PyResult {
        match new_command(cmd, args).status() {
            Ok(_) => Ok(vm.ctx.none()),
            Err(e) => Err(vm.new_os_error(e.to_string())),
        }
    }

    #[pyfunction]
    fn subproc_captured_stdout(
        cmd: PyStrRef,
        args: PySequence<PyStrRef>,
        vm: &VirtualMachine,
    ) -> PyResult {
        match new_command(cmd, args).stderr(Stdio::inherit()).output() {
            Ok(output) => Ok(vm.ctx.new_bytes(output.stdout)),
            Err(e) => Err(vm.new_os_error(e.to_string())),
        }
    }
}
