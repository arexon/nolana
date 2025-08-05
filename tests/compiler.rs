use insta::assert_snapshot;
use nolana::{Codegen, CodegenOptions, Compiler, Parser};

fn compile(source: &str) -> String {
    let mut result = Parser::new(source).parse();
    Compiler::default().compile(&mut result.program);
    Codegen::default().with_options(CodegenOptions { minify: false }).build(&result.program)
}

#[test]
fn binary() {
    let out = compile(
        "
            v.a % v.x;
            v.a ** v.x;
            v.a >> v.x;
            v.a << v.x;
        ",
    );
    assert_snapshot!(
        out,
        @r"
            math.mod(variable.a, variable.x);
            math.pow(variable.a, variable.x);
            math.floor(variable.a / math.pow(2, variable.x));
            variable.a * math.pow(2, variable.x);
        "
    );
}

#[test]
fn assigments() {
    let out = compile(
        "
            v.a = 204.31;
            v.b += 87;
            v.c -= 48.933;
            v.d *= 3233.23;
            v.e /= 1290;
            v.f **= 32.2;
            v.g %= 32;
        ",
    );
    assert_snapshot!(
        out,
        @r"
            variable.a = 204.31;
            variable.b = variable.b ?? 0 + 87;
            variable.c = variable.c ?? 0 - 48.933;
            variable.d = variable.d ?? 0 * 3233.23;
            variable.e = variable.e ?? 0 / 1290;
            variable.f = math.pow(variable.f ?? 0, 32.2);
            variable.g = math.mod(variable.g ?? 0, 32);
        "
    );
}

#[test]
fn updates() {
    let out = compile(
        "
            v.a++;
            {
                v.other = math.random();
                v.result = v.b--;
                v.do ? {
                    v.other = math.random();
                    q.foo(v.c++);
                    v.other = math.random();
                };
            };
        ",
    );
    assert_snapshot!(
        out,
        @r"
            variable.a = variable.a + 1;
            {
                variable.other = math.random();
                variable.b = variable.b - 1;
                variable.result = variable.b;
                variable.do ? {
                    variable.other = math.random();
                    variable.c = variable.c + 1;
                    query.foo(variable.c);
                    variable.other = math.random();
                };
            };
        "
    );
}
