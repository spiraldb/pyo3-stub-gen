use crate::stub_type::*;
use ::pyo3::{
    basic::CompareOp,
    pybacked::{PyBackedBytes, PyBackedStr},
    pyclass::boolean_struct::False,
    types::*,
    Bound, Py, PyClass, PyRef, PyRefMut,
};
use maplit::hashset;

impl PyStubType for PyAny {
    fn type_output() -> TypeInfo {
        TypeInfo {
            name: "typing.Any".to_string(),
            import: hashset! { "typing".into() },
        }
    }
}

impl<T: PyStubType> PyStubType for Py<T> {
    fn type_input() -> TypeInfo {
        T::type_input()
    }
    fn type_output() -> TypeInfo {
        T::type_output()
    }
}

impl<T: PyStubType + PyClass> PyStubType for PyRef<'_, T> {
    fn type_input() -> TypeInfo {
        T::type_input()
    }
    fn type_output() -> TypeInfo {
        T::type_output()
    }
}

impl<T: PyStubType + PyClass<Frozen = False>> PyStubType for PyRefMut<'_, T> {
    fn type_input() -> TypeInfo {
        T::type_input()
    }
    fn type_output() -> TypeInfo {
        T::type_output()
    }
}

impl<T: PyStubType> PyStubType for Bound<'_, T> {
    fn type_input() -> TypeInfo {
        T::type_input()
    }
    fn type_output() -> TypeInfo {
        T::type_output()
    }
}

macro_rules! impl_builtin {
    ($ty:ty, $pytype:expr) => {
        impl PyStubType for $ty {
            fn type_output() -> TypeInfo {
                TypeInfo {
                    name: $pytype.to_string(),
                    import: HashSet::new(),
                }
            }
        }
    };
}

impl_builtin!(PyInt, "int");
impl_builtin!(PyFloat, "float");
impl_builtin!(PyList, "list");
impl_builtin!(PyTuple, "tuple");
impl_builtin!(PySlice, "slice");
impl_builtin!(PyDict, "dict");
impl_builtin!(PySet, "set");
impl_builtin!(PyString, "str");
impl_builtin!(PyBackedStr, "str");
impl_builtin!(PyByteArray, "bytearray");
impl_builtin!(PyBytes, "bytes");
impl_builtin!(PyBackedBytes, "bytes");
impl_builtin!(PyType, "type");
impl_builtin!(CompareOp, "int");

impl PyStubType for PyIterator {
    fn type_output() -> TypeInfo {
        TypeInfo {
            name: "collections.abc.Iterator".to_string(),
            import: hashset! { "collections.abc".into() },
        }
    }
}

#[cfg_attr(all(not(pyo3_0_25), Py_LIMITED_API), expect(unused_macros))]
macro_rules! impl_simple {
    ($ty:ty, $mod:expr, $pytype:expr) => {
        impl PyStubType for $ty {
            fn type_output() -> TypeInfo {
                TypeInfo {
                    name: concat!($mod, ".", $pytype).to_string(),
                    import: hashset! { $mod.into() },
                }
            }
        }
    };
}

#[cfg(any(pyo3_0_25, not(Py_LIMITED_API)))]
impl_simple!(PyDate, "datetime", "date");
#[cfg(any(pyo3_0_25, not(Py_LIMITED_API)))]
impl_simple!(PyDateTime, "datetime", "datetime");
#[cfg(any(pyo3_0_25, not(Py_LIMITED_API)))]
impl_simple!(PyDelta, "datetime", "timedelta");
#[cfg(any(pyo3_0_25, not(Py_LIMITED_API)))]
impl_simple!(PyTime, "datetime", "time");
#[cfg(any(pyo3_0_25, not(Py_LIMITED_API)))]
impl_simple!(PyTzInfo, "datetime", "tzinfo");
