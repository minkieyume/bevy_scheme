pub trait SchemeRuntime {
    fn eval(&mut self, code: &str) -> Result<SchemeValue, SchemeError>;
    fn eval_file(&mut self, path: &str) -> Result<SchemeValue, SchemeError>;
    
    // === 变量绑定 ===
    fn define(&mut self, name: &str, value: SchemeValue) -> Result<(), SchemeError>;
    fn get(&self, name: &str) -> Result<SchemeValue, SchemeError>;
    
    // === Rust 互操作（关键部分）===
    // 注册 Rust 函数给 Scheme 调用
    fn register_function<F>(&mut self, name: &str, func: F) -> Result<(), SchemeError>
    where
        F: Fn(&[SchemeValue]) -> Result<SchemeValue, SchemeError> + 'static;
    
    // 调用 Scheme 函数
    fn call_scheme_function(&mut self, name: &str, args: &[SchemeValue]) 
        -> Result<SchemeValue, SchemeError>;
}

#[derive(Clone)]
pub enum SchemeValue {
    Nil,
    Unknown,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Symbol(String),
}

pub struct SchemeError {
    pub message: String,
}
