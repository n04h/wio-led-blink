#![no_std]
// no_std ... WindowsやmacOSなどの普通のOS上でないことを明示させるアトリビュート
//            Rustの標準ライブラリであるcore, alloc, stdのstdからcoreを使ってビルドを行うようになる
//            stdはOSの機能を使うため、使用するにはOSが必要

#![no_main]
// no_main ... 標準のmain関数のインターフェースを使わないことを明示させるアトリビュート
//             Rustが関数を呼び出すために前処理をしていて、その処理内にOSの機能を使ってしまうためそれをさせないようにしている

use panic_halt as _;
// Rustでプログラムを異常終了させるためにつかうpanicハンドラを定義するクレート
// panic時は上記ハンドラが必ず定義されている必要があるが、stdを使わない場合定義されていない状態になるためここで定義している

use support::*;

// ↓ no_mainアトリビュートを使わなくなるのでエントリポイントを指定するアトリビュート
#[entry]
fn main() -> ! { // ! ... 発散する関数(diverging function)と呼ばれるもので関数からはなにも戻らないことを示す
    let (mut user_led, mut delay) = support::init();

    loop {
        delay.delay_ms(1_000u16);
        user_led.toggle();
    }
}
