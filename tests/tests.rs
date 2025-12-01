use bevy_scheme::*;

#[cfg(feature = "guile")]
mod guile_tests {
    use super::*;

    #[test]
    fn can_init() {
        GuileRuntime::init(|vm| {           
            println!("Hello Guile");
        });
    }
}
