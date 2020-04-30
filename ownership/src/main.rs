use std::ops::Drop;

struct Resource(usize);

impl Drop for Resource {
    // デストラクタの実装
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    {
        // ヒープから初期値 4 のデータを確保する
        let x = Box::new(4);

        println!("x = {}", x);
    } // ここで x のスコープが切れる
    
    {
        let x = Resource(4);

        println!("x.0 = {}", x.0);
    } // ここでスコープが切れるので、デストラクタが呼ばれる

    {
        let x = 4;
        let y = x; // 値をコピーして新しい所有権を作る
        // i32 や bool などのスカラ値は Copy トレイトが実装されているからコピーになる

        // 両方アクセス可能
        println!("x = {}, y = {}", x, y);
        // 実態は別物
        println!("&x = {:p}, &y = {:p}", &x, &y);
    }

    {
        let x = Resource(4);
        let y = x; // 所有権を移している
        // Resource は Copy トレイトが実装されていないからムーブになる

        // 所有権が y に移されてる(ムーブ)から x にはアクセスできない
        // コンパイルエラー
        //println!("x = {}, y = {}", x.0, y.0);
        // y だけならアクセス可能
        println!("y = {}", y.0);
    }

    let mut x = String::from("a");
    add_one(&mut x);
    add_one(&mut x);
    
    print!("x = {}", x);
}

fn add_one(x: &mut String) {
    x.push_str("s\n");
}