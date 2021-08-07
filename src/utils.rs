use crate::*;
pub type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

pub trait InstantExt {
    fn elapsed_now(&mut self) -> Duration;
}

impl InstantExt for Instant {
    fn elapsed_now(&mut self) -> Duration {
        let prev = *self;
        let now = Instant::now();
        *self = now;
        now - prev
    }
}

pub struct Byte(pub u8);

impl FromResidual<Result<Infallible, !>> for Byte {
    fn from_residual(residual: Result<Infallible, !>) -> Self {
        match residual {
            Ok(s) => match s {},
            Err(s) => s,
        }
    }
}

impl Try for Byte {
    type Output = u8;

    type Residual = Result<Infallible, !>;

    fn from_output(output: u8) -> Self {
        Self(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, u8> {
        ControlFlow::Continue(self.0)
    }
}
