use std::error::Error;

pub fn run<T>(ps_config: PSConfig<T>) -> Result<f64, Box<dyn Error>>
where T: Fn(Vec<f64>) -> f64,
{
    Ok(0.0)
}

pub struct PSConfig<T> 
where T: Fn(Vec<f64>) -> f64,
{
    merit: T,
}

impl<T> PSConfig<T> 
where T: Fn(Vec<f64>) -> f64,
{
    pub fn new(merit: T) -> PSConfig<T> {
        PSConfig {merit}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialise() {
        let merit = |x: Vec<f64>| x.iter().sum();

        let ps_config = PSConfig::new(merit);

        let expected = 0.0;
        let unexpected = 1.0;

        let result = match run(ps_config) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error when running {}", e);
                unexpected
            }
        };

        assert_eq!(expected, result);
    }
}
