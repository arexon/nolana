use insta::assert_snapshot;
use nolana::{Codegen, CodegenOptions, MolangTransformer, Parser};

fn transform(source: &str) -> String {
    let mut result = Parser::new(source).parse();
    MolangTransformer::default().transform(&mut result.program);
    Codegen::default().with_options(CodegenOptions { minify: false }).build(&result.program)
}

#[test]
fn binary() {
    let out = transform(
        "
            v.x % v.y;
            v.x ** v.y;
            v.x >> v.y;
            v.x << v.y;
            q.foo(v.x | v.y);
            q.bar(v.x & v.y);
            q.baz(v.x ^ v.y);
        ",
    );
    assert_snapshot!(
        out,
        @r"
            math.mod(variable.x, variable.y);
            math.pow(variable.x, variable.y);
            math.floor(variable.x / math.pow(2, variable.y));
            variable.x * math.pow(2, variable.y);
            {
                variable.__4_result = 0;
                variable.__4_bit = 0;
                loop(24, {
                    variable.__4_left_bit = math.mod(math.floor(variable.x / math.pow(2, variable.__4_bit)), 2);
                    variable.__4_right_bit = math.mod(math.floor(variable.y / math.pow(2, variable.__4_bit)), 2);
                    variable.__4_or_bit = math.min(1, variable.__4_left_bit + variable.__4_right_bit);
                    variable.__4_result = variable.__4_result + variable.__4_or_bit * math.pow(2, variable.__4_bit);
                    variable.__4_bit = variable.__4_bit + 1;
                });
            };
            query.foo(variable.__4_result);
            {
                variable.__6_result = 0;
                variable.__6_bit = 0;
                loop(24, {
                    variable.__6_left_bit = math.mod(math.floor(variable.x / math.pow(2, variable.__6_bit)), 2);
                    variable.__6_right_bit = math.mod(math.floor(variable.y / math.pow(2, variable.__6_bit)), 2);
                    variable.__6_and_bit = variable.__6_left_bit * variable.__6_right_bit;
                    variable.__6_result = variable.__6_result + variable.__6_and_bit * math.pow(2, variable.__6_bit);
                    variable.__6_bit = variable.__6_bit + 1;
                });
            };
            query.bar(variable.__6_result);
            {
                variable.__8_result = 0;
                variable.__8_bit = 0;
                loop(24, {
                    variable.__8_left_bit = math.mod(math.floor(variable.x / math.pow(2, variable.__8_bit)), 2);
                    variable.__8_right_bit = math.mod(math.floor(variable.y / math.pow(2, variable.__8_bit)), 2);
                    variable.__8_xor_bit = math.mod(variable.__8_left_bit + variable.__8_right_bit, 2);
                    variable.__8_result = variable.__8_result + variable.__8_xor_bit * math.pow(2, variable.__8_bit);
                    variable.__8_bit = variable.__8_bit + 1;
                });
            };
            query.baz(variable.__8_result);
        "
    );
}

#[test]
fn assigments() {
    let out = transform(
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
            v.x |= v.y;
            v.x &= v.y;
            v.x ^= v.y;
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
            {
                variable.__11_result = 0;
                variable.__11_bit = 0;
                loop(24, {
                    variable.__11_left_bit = math.mod(math.floor(variable.x ?? 0 / math.pow(2, variable.__11_bit)), 2);
                    variable.__11_right_bit = math.mod(math.floor(variable.y / math.pow(2, variable.__11_bit)), 2);
                    variable.__11_or_bit = math.min(1, variable.__11_left_bit + variable.__11_right_bit);
                    variable.__11_result = variable.__11_result + variable.__11_or_bit * math.pow(2, variable.__11_bit);
                    variable.__11_bit = variable.__11_bit + 1;
                });
            };
            variable.x = variable.__11_result;
            {
                variable.__13_result = 0;
                variable.__13_bit = 0;
                loop(24, {
                    variable.__13_left_bit = math.mod(math.floor(variable.x ?? 0 / math.pow(2, variable.__13_bit)), 2);
                    variable.__13_right_bit = math.mod(math.floor(variable.y / math.pow(2, variable.__13_bit)), 2);
                    variable.__13_and_bit = variable.__13_left_bit * variable.__13_right_bit;
                    variable.__13_result = variable.__13_result + variable.__13_and_bit * math.pow(2, variable.__13_bit);
                    variable.__13_bit = variable.__13_bit + 1;
                });
            };
            variable.x = variable.__13_result;
            {
                variable.__15_result = 0;
                variable.__15_bit = 0;
                loop(24, {
                    variable.__15_left_bit = math.mod(math.floor(variable.x ?? 0 / math.pow(2, variable.__15_bit)), 2);
                    variable.__15_right_bit = math.mod(math.floor(variable.y / math.pow(2, variable.__15_bit)), 2);
                    variable.__15_xor_bit = math.mod(variable.__15_left_bit + variable.__15_right_bit, 2);
                    variable.__15_result = variable.__15_result + variable.__15_xor_bit * math.pow(2, variable.__15_bit);
                    variable.__15_bit = variable.__15_bit + 1;
                });
            };
            variable.x = variable.__15_result;
        "
    );
}

#[test]
fn updates() {
    let out = transform(
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

#[test]
fn simple_into_complex_with_update() {
    let out = transform("v.x++");
    assert_snapshot!(
        out,
        @r"
            variable.x = variable.x + 1;
            return variable.x;
        "
    );
}

#[test]
fn simple_into_complex_with_bitwise() {
    let out = transform("v.x | v.y");
    assert_snapshot!(
        out,
        @r"
            {
                variable.__0_result = 0;
                variable.__0_bit = 0;
                loop(24, {
                    variable.__0_left_bit = math.mod(math.floor(variable.x / math.pow(2, variable.__0_bit)), 2);
                    variable.__0_right_bit = math.mod(math.floor(variable.y / math.pow(2, variable.__0_bit)), 2);
                    variable.__0_or_bit = math.min(1, variable.__0_left_bit + variable.__0_right_bit);
                    variable.__0_result = variable.__0_result + variable.__0_or_bit * math.pow(2, variable.__0_bit);
                    variable.__0_bit = variable.__0_bit + 1;
                });
            };
            return variable.__0_result;
        "
    );
}
