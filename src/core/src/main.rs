// Mainly for debug purposes

use comemo::Track;
use typst::utils::LazyHash;
use core::convert;
use typst::World;

struct FakeWorld {
    library: LazyHash<typst::Library>,
}

impl FakeWorld {
    fn new() -> Self {
        FakeWorld {
            library: LazyHash::new(typst::Library::builder().build()),
        }
    }
}

impl World for FakeWorld {
    fn library(&self) -> &LazyHash<typst::Library> {
        &self.library
    }
    fn book(&self) -> &LazyHash<typst::text::FontBook> {
        unimplemented!();
    }
    fn file(&self, id: typst_syntax::FileId) -> typst::diag::FileResult<typst::foundations::Bytes> {
        unimplemented!();
    }
    fn font(&self, index: usize) -> Option<typst::text::Font> {
        unimplemented!();
    }
    fn main(&self) -> typst_syntax::FileId {
        unimplemented!();
    }
    fn source(&self, id: typst_syntax::FileId) -> typst::diag::FileResult<typst_syntax::Source> {
        unimplemented!();
    }
    fn today(&self, offset: Option<i64>) -> Option<typst::foundations::Datetime> {
        unimplemented!();
    }
}

pub fn eval(world: &dyn typst::World, string: &str) -> Result<typst::foundations::Content, String> {
    let result = typst_eval::eval_string(
        &typst::ROUTINES,
        world.track(),
        string,
        typst::syntax::Span::detached(),
        typst_eval::EvalMode::Math,
        world.library().math.scope().clone(),
    );

    match result {
        Ok(value) => match value {
            typst::foundations::Value::Content(content) => Ok(content),
            _ => Err("Expected content result.".to_string()),
        },
        Err(err) => Err(err[0].message.to_string()),
    }
}

// Equations tested:
// A = pi r^2
// "area" = pi dot "radius"^2
// cal(A) := { x in RR | x "is natural" }
// x < y => x gt.eq.not y
// sum_(k=0)^n k &= 1 + ... + n \ &= (n(n+1)) / 2
// frac(a^2, 2)
// vec(1, 2, delim: "[")
// mat(1, 2; 3, 4)
// lim_x = op("lim", limits: #true)_x
// (3x + y) / 7 &= 9 && "given" \ 3x + y &= 63 & "multiply by 7" \ 3x &= 63 - y && "subtract y" \ x &= 21 - y/3 & "divide by 3"
// sum_(i=0)^n a_i = 2^(1+i)
// 1/2 < (x+1)/2
// ((x+1)) / 2 = frac(a, b)
// tan x = (sin x)/(cos x)
// op("custom", limits: #true)_(n->oo) n
// bb(b)
// bb(N) = NN
// f: NN -> RR
// vec(a, b, c) dot vec(1, 2, 3) = a + 2b + 3c

// Works partially
// attach(Pi, t: alpha, b: beta, tl: 1, tr: 2+3, bl: 4+5, br: 6)
// lr(]sum_(x=1)^n] x, size: #50%)
// mat(1, 2, ..., 10; 2, 2, ..., 10; dots.v, dots.v, dots.down, dots.v; 10, 10, ..., 10)
// upright(A) != A

// Does not work
// grave(a) = accent(a, `)
// arrow(a) = accent(a, arrow)
// tilde(a) = accent(a, \u{0303})
// scripts(sum)_1^2 != sum_1^2
// limits(A)_1^2 != A_1^2
// (a dot b dot cancel(x)) / cancel(x)
// f(x, y) := cases(1 "if" (x dot y)/2 <= 0, 2 "if" x "is even", 3 "if" x in NN, 4 "else")
// Class: https://typst.app/docs/reference/math/class/
// abs((x + y) / 2)
// { x mid(|) sum_(i=1)^n w_i|f_i (x)| < 1 }
// norm(x/2)
// abs(x/2)
// floor(x/2)
// ceil(x/2)
// round(x/2)
// sqrt(3 - 2 sqrt(2)) = sqrt(2) - 1
// root(3, x)
// sum_i x_i/2 = inline(sum_i x_i/2)
// sum_i x_i/2 = display(sum_i x_i/2)
// sum_i x_i/2 = script(sum_i x_i/2)
// sum_i x_i/2 = sscript(sum_i x_i/2)
// upright(A) != A
// underline(1 + 2 + ... + 5)
// overline(1 + 2 + ... + 5)
// underbrace(1 + 2 + ... + 5, "numbers")
// overbrace(1 + 2 + ... + 5, "numbers")
// underbracket(1 + 2 + ... + 5, "numbers")
// overbracket(1 + 2 + ... + 5, "numbers")
// sans(A B C)
// frak(P)
// mono(x + y = z)

pub fn main() {
    // Try to construct a MathContext object.
    let world = FakeWorld::new();
    let content = eval(&world, "bb(italic(upright(a))/bold(x))").unwrap();
    let math: &typst::math::EquationElem = content.to_packed::<typst::math::EquationElem>().unwrap();
    println!("{:#?}", math);
    println!("{:#?}", convert(&content));
}
