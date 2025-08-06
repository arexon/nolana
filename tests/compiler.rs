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
            v.x % v.y;
            v.x ** v.y;
            v.x >> v.y;
            v.x << v.y;
        ",
    );
    assert_snapshot!(
        out,
        @r"
            math.mod(variable.x, variable.y);
            math.pow(variable.x, variable.y);
            math.floor(variable.x / math.pow(2, variable.y));
            variable.x * math.pow(2, variable.y);
        "
    );
}

#[test]
fn assigments() {
    let out = compile(
        "
            v.x = v.y;
            v.x += v.y;
            v.x -= v.y;
            v.x *= v.y;
            v.x /= v.y;
            v.x **= v.y;
            v.x %= v.y;
            v.x &&= v.y;
            v.x ||= v.y;
            v.x >>= v.y;
            v.x <<= v.y;
        ",
    );
    assert_snapshot!(
        out,
        @r"
            variable.x = variable.y;
            variable.x = variable.x ?? 0 + variable.y;
            variable.x = variable.x ?? 0 - variable.y;
            variable.x = variable.x ?? 0 * variable.y;
            variable.x = variable.x ?? 0 / variable.y;
            variable.x = math.pow(variable.x ?? 0, variable.y);
            variable.x = math.mod(variable.x ?? 0, variable.y);
            variable.x ? {
                variable.x = variable.y;
            };
            !variable.x ? {
                variable.x = variable.y;
            };
            variable.x = math.floor(variable.x ?? 0 / math.pow(2, variable.y));
            variable.x = variable.x ?? 0 * math.pow(2, variable.y);
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
