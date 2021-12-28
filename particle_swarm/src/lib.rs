use std::error::Error;

pub fn run(config: Box<dyn MeritFunction>) -> Result<f64, Box<dyn Error>>
{
    let value = config.calculate();
    Ok(value)
}

pub trait MeritFunction {
    fn calculate(&self) -> f64;
}

#[cfg(test)]
mod tests {
    #[test]
    fn initialise() {
        assert_eq!(1, 1);
    }
}
