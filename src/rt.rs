use core::mem;

#[lang = "start"]
#[allow(unused_variables)]
fn lang_start(main: *const u8, argc: isize, argv: *const *const u8) -> isize {
    unsafe { mem::transmute::<_, fn()>(main)(); }
    0
}
