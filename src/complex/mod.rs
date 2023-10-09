pub mod test;

#[derive(Debug, Clone)]
pub struct Complex
{
    real: f64,
    imag: f64,
}

impl Complex
{
    pub fn new(r: f64, i: f64) -> Complex
    {
        return Complex{real: r, imag: i};
    }

    pub fn zero() -> Complex
    {
        return Complex{real: 0.0, imag: 0.0};
    }

    pub fn add(&self, c: Complex) -> Complex
    {
        return Complex{real: (self.real + c.real), imag: (self.imag + c.imag)};
    }

    pub fn mul(&self, c: Complex) -> Complex
    {
        let x = (self.real * c.real) - (self.imag * c.imag);
        let y = (self.real * c.imag) - (self.imag * c.real);
        return Complex{real: x, imag: y};
    }

    pub fn conj(&self) -> Complex
    {
        return Complex{real: self.real, imag: -self.imag};
    }

    pub fn scalar(&self, s: f64) -> Complex
    {
        return Complex{real: self.real * s, imag: self.imag * s};
    }

    pub fn exp(&self) -> Complex
    {
        let f = self.real.exp();

        let x = self.imag.cos();
        let y = self.imag.sin();

        return Complex{real: (f * x), imag: (f * y)};
    }

    pub fn abs_squared(&self) -> f64
    {
        let re = self.real * self.real;
        let im = self.imag * self.imag;
        return re + im;
    }
}