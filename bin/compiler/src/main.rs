mod ast;
mod emitter;
mod lexer;
mod parser;

use ir::{
    context::Context,
    ty::{Type, TypeKind},
};

use crate::{lexer::Lexer, parser::Parser};

fn main() {
    let source = "
        fn test(a: u32, b: u32) -> u32 {
            let temp: u32 = a / 2;

            if temp > 10 {
                for i in 0..10 {
                    if i > 5 {
                        a += i * b;
                    } else {
                        a += i * b * b;
                    }
                }
            }

            return a;
        }
    ";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();
    let mut parser = Parser::new(&lexer, tokens);
    let module = parser.parse();
    println!("{module:#?}");
    let mut emitter = emitter::Emitter::new();
    emitter.emit(module);

    // for token in tokens {
    //     let kind = lexer.get_token_kind(token);
    //     println!(
    //         "{token:?}: {:?} {:?}",
    //         kind.to_string(),
    //         lexer.get_identifier(token)
    //     );
    // }

    // let mut context = Context::new();

    // let i32_type = context.create_type(TypeKind::Integer {
    //     num_bits: 32,
    //     is_signed: true,
    // });
    // let f32_type = context.create_type(TypeKind::Float { num_bits: 32 });

    // {
    //     let function =
    //         context.create_function("test", Some(&[i32_type]), &[i32_type, i32_type]);

    //     context.with_function(function, |mut ctx| {
    //         let param0 = ctx.parameter(0);
    //         let param1 = ctx.parameter(1);

    //         let entry = ctx.create_label("entry");
    //         let on_true = ctx.create_label("on_true");
    //         let on_false = ctx.create_label("on_false");
    //         ctx.with_label(entry, |mut ctx| {
    //             let cond = ctx.compare_gt(param0, param1);
    //             ctx.branch_conditional(cond, on_true, on_false);
    //         });

    //         ctx.with_label(on_true, |mut ctx| {
    //             let result = ctx.add(param0, param1);
    //             ctx.ret(Some(&[result]));
    //         });

    //         ctx.with_label(on_false, |mut ctx| {
    //             let result = ctx.mul(param0, param1);
    //             ctx.ret(Some(&[result]));
    //         });
    //     });
    // }

    // {
    //     let function = context.create_function(
    //         "lerp",
    //         Some(&[f32_type]),
    //         &[f32_type, f32_type, f32_type],
    //     );

    //     context.with_function(function, |mut ctx| {
    //         let param0 = ctx.parameter(0);
    //         let param1 = ctx.parameter(1);
    //         let param2 = ctx.parameter(1);

    //         let entry = ctx.create_label("entry");
    //         ctx.with_label(entry, |mut ctx| {
    //             let res = ctx.sub(param1, param0);
    //             let res = ctx.mul(param2, res);
    //             let res = ctx.add(param0, res);
    //             ctx.ret(Some(&[res]));
    //         });
    //     });
    // }

    // let path = std::path::Path::new("test.ir");
    // context.dump_ir(path);
}
