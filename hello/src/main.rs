fn main() {
    // デフォルトでイミュータブル
    // 代入ではなく「束縛」と呼ぶn
    let x = 43;

    // 再代入はできない
    // x = 52;

    println!("x = {}", x);

    // mut をつけることでミュータブルになる
    let mut y = 42;
    println!("y = {}", y);

    // 再代入可能
    y = 53;
    
    println!("y = {}", y);

    // 型推論が強力だから基本的に型を書く必要がない
    // 明示的に書きたい場合は、後置
    let x0: u32 = 32;
    let x1: u8  = 8;
    // 異なる型の演算は不可能。暗黙的な型変換は行わない
    // let x2: u32 = x0 + x1;
    println!("{} + {} ", x0, x1);

    // タプル
    // 複数の型の値を一つにまとめることができる
    let t: (i32, bool, u16) = (42, true, 16);

    // t.0 t.1 でそれぞれの要素が取り出せる
    println!("t.0 = {}, t.1 = {}", t.0, t.1);

    // タプルから各要素を取り出すことができる
    // パターンマッチと呼ぶ
    // 関数の返り値とかに使えそう…？
    let (t0, t1, t2) = t;
    println!("t0 = {}, t1 = {}, t2 = {}", t0, t1, t2);

    // 配列
    // 要素数が違えば違う型
    // ポインタになることはない
    let a = [0, 1, 2];
    println!("a = {}", a.len());
    
    // 引数なし、返り値なし関数(subroutine)
    sub();

    // 引数あり、返り値なし関数
    func1(32);

    // 引数あり、返り値なし関数
    println!("double -> {}", my_double(3));

    let two = add_one(1);
    println!("two = {}", two);

    // for 文はイテレータを使う
    // C ライクな for(;;) は存在しない
    for value in a.iter() {
        println!("valude: {}", value);
    }

    println!("Hello, world!");
}

fn sub() {
    println!("sub");
}

fn func1(x: i32) {
    println!("x = {}", x);
}

fn my_double(x: i32) -> i32 {
    return x * 2;
}

// 最後は return を書かない事が多い
// その場合、セミコロンも不要
// return は早期リターンで使う事が多い
fn add_one(x: i32) -> i32 {
    println!("{}", x);
    x + 1
}