use super::context::Context;

#[test]
fn define_get() {
    let mut ctx = Context::new_one_frame();

    ctx.define("a".to_string(), 3);
    ctx.define("b".to_string(), 2);

    assert_eq!(ctx.get_value("a"), Some(&3));
    assert_eq!(ctx.get_value("b"), Some(&2));

}

#[test]
fn define_in_parent_get_in_child() {
    let mut ctx = Context::new_one_frame();

    ctx.define("x".to_string(), 3);
    ctx.define("y".to_string(), 2);

    ctx.push_frame(true);

    ctx.define("y".to_string(), 1);

    assert_eq!(ctx.get_value("x"), Some(&3));
    assert_eq!(ctx.get_value("y"), Some(&1));

    ctx.push_frame(false);

    ctx.define("x".to_string(), 0);

    assert_eq!(ctx.get_value("x"), Some(&0));
    assert_eq!(ctx.get_value("y"), None);
    
    ctx.pop_frame();
    ctx.pop_frame();

    assert_eq!(ctx.get_value("x"), Some(&3));
    assert_eq!(ctx.get_value("y"), Some(&2));
}
