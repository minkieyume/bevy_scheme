use bevy::prelude::*;
pub use guile::GuileVM;
pub use guile;
pub use crate::runtimes::scheme::{SchemeRuntime, SchemeValue, SchemeError};

#[derive(Resource)]
pub struct GuileRuntime;

pub trait GuileInit {
    fn init<F>(func: F) 
    where
      F: Fn(GuileVM);
}

// Guile VM 实现
impl GuileInit for GuileRuntime {    
    fn init<F>(func: F) 
    where
      F: Fn(GuileVM)
      {
          guile::init(func);
      }
}

impl SchemeRuntime for GuileRuntime {
    fn eval(&mut self, code: &str) -> Result<SchemeValue, SchemeError> {
        // 实现代码
        Ok(SchemeValue::Nil)
    }
    
    fn eval_file(&mut self, path: &str) -> Result<SchemeValue, SchemeError> {
        // 实现代码
        Ok(SchemeValue::Nil)
    }
    
    fn define(&mut self, name: &str, value: SchemeValue) -> Result<(), SchemeError> {
        // 实现代码
        Ok(())
    }
    
    fn get(&self, name: &str) -> Result<SchemeValue, SchemeError> {
        // 实现代码
        Ok(SchemeValue::Nil)
    }
    
    fn register_function<F>(&mut self, name: &str, func: F) -> Result<(), SchemeError>
    where
        F: Fn(&[SchemeValue]) -> Result<SchemeValue, SchemeError> + 'static,
    {
        // 实现代码
        Ok(())
    }
    
    fn call_scheme_function(&mut self, name: &str, args: &[SchemeValue]) 
        -> Result<SchemeValue, SchemeError> 
    {
        // 实现代码
        Ok(SchemeValue::Nil)
    }
}
