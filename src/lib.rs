pub enum Value {
    Int(i64),
    Float(f64),
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}

pub trait HostMachine {
    type Data;

    fn new(data: Self::Data) -> Self;

    fn destroy(&self);
}

pub struct Runtime<T: HostMachine> {
    #[allow(dead_code)]
    host: T,
}

impl <T: HostMachine> Runtime<T> {
    pub fn new(data: T::Data) -> Runtime<T> {
        Runtime {
            host: T::new(data),
        }
    }

    pub fn print(&self, v: &Value) {
        match v {
            Value::Int(n) => println!("{}", n),
            Value::Float(n) => println!("{}", n),
        }
    }

    pub fn destroy(&self) {
        self.host.destroy();
    }
}

#[cfg(feature = "cgo")]
pub mod cgo {
    use crate::Value;
    use libc::uintptr_t;

    type Runtime = crate::Runtime<HostMachine>;
    pub struct HostMachine {
        _handle: uintptr_t,
    }

    impl crate::HostMachine for HostMachine {
        type Data = uintptr_t;

        fn new(data: Self::Data) -> Self {
            HostMachine { _handle: data }
        }

        fn destroy(&self) {}
    }

    #[no_mangle]
    pub extern "C" fn value_int_new(v: i64) -> Box<Value> {
        Box::new(Value::from(v))
    }

    #[no_mangle]
    pub extern "C" fn value_free(_: Box<Value>) {}

    #[no_mangle]
    pub extern "C" fn runtime_create(h: uintptr_t) -> Box<Runtime> {
        Box::new(Runtime::new(h))
    }

    #[no_mangle]
    pub extern "C" fn runtime_destroy(rt: Box<Runtime>) {
        rt.destroy();
    }

    #[no_mangle]
    pub extern "C" fn runtime_print(rt: &Runtime, v: &Value) {
        rt.print(v)
    }

    extern "C" {
        // fn NewVMContext() -> uintptr_t;
        // fn DestroyVMContext(h: uintptr_t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
