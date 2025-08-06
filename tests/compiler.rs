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
            v.x = 204.31;
            v.x += 87;
            v.x -= 48.933;
            v.x *= 3233.23;
            v.x /= 1290;
            v.x **= 32.2;
            v.x %= 32;
            v.x &&= v.y;
            v.x ||= v.y;
        ",
    );
    assert_snapshot!(
        out,
        @r"
            variable.x = 204.31;
            variable.x = variable.x ?? 0 + 87;
            variable.x = variable.x ?? 0 - 48.933;
            variable.x = variable.x ?? 0 * 3233.23;
            variable.x = variable.x ?? 0 / 1290;
            variable.x = math.pow(variable.x ?? 0, 32.2);
            variable.x = math.mod(variable.x ?? 0, 32);
            variable.x ? {
                variable.x = variable.y;
            };
            !variable.x ? {
                variable.x = variable.y;
            };
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
