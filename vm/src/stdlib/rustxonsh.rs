pub(crate) use decl::make_module;

#[pymodule(name = "rustxonsh")]
mod decl {
    use crate::{builtins::PyStrRef, pyobject::PySequence, vm::VirtualMachine};

    #[pyfunction]
    fn subproc_uncaptured(cmd: PyStrRef, args: PySequence<PyStrRef>, _vm: &VirtualMachine) -> () {
        println!("cmd: {}", cmd.as_str());
        for (arg, i) in args.into_vec().iter().zip(1..) {
            println!("arg {}: {}", i, arg.as_str());
        }
    }
}
