pub struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher{
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_value() {
        let mut cacher = super::Cacher::new(|v| v);
        let v1 = cacher.value(1);
        let v2 = cacher.value(2);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }
}
