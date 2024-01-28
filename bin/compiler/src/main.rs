use ir::{
    context::Context,
    ty::{Float, Integer, Type, TypeKind},
};

fn main() {
    let mut context = Context::new();

    let i32_type =
        context.create_type(Type::new(TypeKind::Integer { kind: Integer::I32 }, None));
    let f32_type =
        context.create_type(Type::new(TypeKind::Float { kind: Float::F32 }, None));

    {
        let function =
            context.create_function("test", Some(&[i32_type]), &[i32_type, i32_type]);

        context.with_function(function, |mut ctx| {
            let param0 = ctx.parameter(0);
            let param1 = ctx.parameter(1);

            let entry = ctx.create_label("entry");
            let on_true = ctx.create_label("on_true");
            let on_false = ctx.create_label("on_false");
            ctx.with_label(entry, |mut ctx| {
                let cond = ctx.compare_gt(param0, param1);
                ctx.branch_conditional(cond, on_true, on_false);
            });

            ctx.with_label(on_true, |mut ctx| {
                let result = ctx.add(param0, param1);
                ctx.ret(Some(&[result]));
            });

            ctx.with_label(on_false, |mut ctx| {
                let result = ctx.mul(param0, param1);
                ctx.ret(Some(&[result]));
            });
        });
    }

    {
        let function = context.create_function(
            "lerp",
            Some(&[f32_type]),
            &[f32_type, f32_type, f32_type],
        );

        context.with_function(function, |mut ctx| {
            let param0 = ctx.parameter(0);
            let param1 = ctx.parameter(1);
            let param2 = ctx.parameter(1);

            let entry = ctx.create_label("entry");
            ctx.with_label(entry, |mut ctx| {
                let res = ctx.sub(param1, param0);
                let res = ctx.mul(param2, res);
                let res = ctx.add(param0, res);
                ctx.ret(Some(&[res]));
            });
        });
    }

    let path = std::path::Path::new("test.ir");
    context.dump_ir(path);
}
