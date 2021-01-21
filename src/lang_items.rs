use core::panic::PanicInfo;
use super::hlt_loop;

// 这个函数将在panic时被调用
#[panic_handler]
fn painc(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
