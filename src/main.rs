#![no_std]
// no_std ... WindowsやmacOSなどの普通のOS上でないことを明示させるアトリビュート
//            Rustの標準ライブラリであるcore, alloc, stdのstdからcoreを使ってビルドを行うようになる
//            stdはOSの機能を使うため、使用するにはOSが必要

#![no_main]

use panic_halt as _;
use support::*;

#[entry]
fn main() -> ! {
    let (mut user_led, mut delay) = support::init();

    loop {
        delay.delay_ms(1_000u16);
        user_led.toggle();
    }
}
