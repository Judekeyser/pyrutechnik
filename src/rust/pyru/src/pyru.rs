#[derive(PartialEq,Debug)]
pub struct Example {
  pub s: String,
  pub x: usize
}

pub fn add(left: Example, right: Example) -> Example {
    let x: usize = left.x + right.x;
    let s: String = left.s + &right.s;

    return Example {
        x: x,
        s: s.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let left = Example {
            x: 2023,
            s: "Hello".to_owned()
        };
        let right = Example {
            x: 1,
            s: "World".to_owned()
        };

        let expected = Example {
            x: 2024,
            s: "HelloWorld".to_owned()
        };
        let actual = add(left, right);
        assert_eq!(actual, expected);
    }
}
