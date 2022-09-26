pub enum Value {
    Int(i64),
    Float(f64),
}

pub trait HostMachine {
}

pub struct Runtime<T> {
    #[allow(dead_code)]
    host: T,
}

impl <T> Runtime<T> {
    pub fn new(host: T) -> Runtime<T> {
        Runtime {
            host,
        }
    }

    pub fn print(&self, v: &Value) {
        match v {
            Value::Int(n) => println!("{}", n),
            Value::Float(n) => println!("{}", n),
        }
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
