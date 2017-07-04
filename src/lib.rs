use std::error::Error;


pub struct Saga<T> {
    adventures: Vec<Adventure<T>>,
}

impl<T> Saga<T> {
    pub fn new(adventures: Vec<Adventure<T>>) -> Self {
        Saga { adventures: adventures }
    }
    /// Tells a saga, reverts on failure and returns either the result or error
    pub fn tell(self: &Self, acc: T) -> Result<T, Failure<T>> {
        tell_(&self.adventures, 0, acc)
    }
}
pub struct Adventure<T> {
    forward: fn(T) -> Result<T, Failure<T>>,
    backward: fn(T) -> T,
}

impl<T> Adventure<T> {
    // add code here
    pub fn forward(self: &Adventure<T>, acc: T) -> Result<T, Failure<T>> {
        let f = self.forward;
        f(acc)
    }
    pub fn backward(self: &Adventure<T>, acc: T) -> T {
        let f = self.backward;
        f(acc)
    }
}

pub struct Failure<T> {
    error: Box<Error>,
    acc: T,
}


fn tell_<T>(saga: &Vec<Adventure<T>>, i: usize, acc: T) -> Result<T, Failure<T>> {
    if i >= saga.len() {
        Ok(acc)
    } else {
        match saga[i].forward(acc) {
            Ok(acc1) => tell_(saga, i + 1, acc1),
            Err(Failure { acc: acc1, error }) => Err(revert(saga, error, i, acc1)),
        }
    }
}

fn revert<T>(saga: &Vec<Adventure<T>>, error: Box<Error>, i: usize, acc: T) -> Failure<T> {
    let acc1 = saga[i].backward(acc);
    if i == 0 {
        Failure { error, acc: acc1 }
    } else {
        revert(saga, error, i - 1, acc1)
    }
}

#[cfg(test)]
mod tests {
    use Adventure;
    use Failure;
    use Saga;
    use std::error::Error;
    use std::fmt;


    #[derive(Debug)]
    pub struct StupidError {
        stupid: bool,
    }
    impl Error for StupidError {
        fn description(&self) -> &str {
            "stupid error"
        }
    }
    impl fmt::Display for StupidError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "is stupid: {}", self.stupid)
        }
    }
    fn inc2(i: i32) -> Result<i32, Failure<i32>> {
        if i >= 2 {
            Err(Failure {
                error: Box::new(StupidError { stupid: true }),
                acc: i + 1,
            })
        } else {
            Ok(i + 1)
        }
    }
    fn dec(i: i32) -> i32 {
        i - 1
    }
    #[test]
    fn good_sage() {
        let saga = Saga::new(vec![
            Adventure {
                forward: inc2,
                backward: dec,
            },
            Adventure {
                forward: inc2,
                backward: dec,
            },
        ]);
        match saga.tell(0) {
            Ok(res) => assert!(res == 2),
            Err(_) => unimplemented!(),
        }
    }
    #[test]
    fn bad_sage() {
        let saga = Saga::new(vec![
            Adventure {
                forward: inc2,
                backward: dec,
            },
            Adventure {
                forward: inc2,
                backward: dec,
            },
            Adventure {
                forward: inc2,
                backward: dec,
            },
        ]);
        match saga.tell(0) {
            Ok(_) => unimplemented!(),
            Err(Failure { acc: res, .. }) => assert_eq!(res, 0),
        }

    }

}
