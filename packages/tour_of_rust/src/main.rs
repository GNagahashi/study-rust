fn main() {
    //
    // 変数
    //
    // ref: https://tourofrust.com/03_ja.html

    // 変数宣言の方法 その1(型を推論させるタイプ)
    let x = 123;  // i32になる(要確認)
    println!("x: {}", x);

    // 変数宣言の方法 その2(型を指定するタイプ)
    let y: i32 = 456;  // i32(32ビット整数を指定)
    println!("y: {}", y);

    // 変数宣言の方法 その3(変数を宣言したあと初期化するタイプ)
    let z;
    z = 789;
    println!("z: {}", z);

    println!("-----");


    //
    // 変数の変更
    //
    // ref: https://tourofrust.com/04_ja.html

    let immutable = "Hello";
    let mut mutable = "Hello";

    println!("immutable: {}", immutable);
    println!("mutable: {}", mutable);

    // immutable = "Hello, Ferris!";
    // 変数immutableは不変なのでコンパイルエラー
    mutable = "Hello, Ferris!";
    
    println!("immutable: {}", immutable);
    println!("mutable: {}", mutable);


    //
    // 変数と定数
    //
    // ref: https://tourofrust.com/07_ja.html

    let this_is_variable = 123;
    const THIS_IS_CONSTANT: i32 = 456;  // 明示的な型指定が必要(あと本当はあらゆるスコープの外で宣言する必要がある)

    println!("variable: {}", this_is_variable);
    println!("constant: {}", THIS_IS_CONSTANT);


    //
    // 配列
    //
    // ref: https://tourofrust.com/08_ja.html

    let arr: [i32; 3] = [1, 2, 3];  // [型; サイズ]で指定する
    println!("{:?}", arr);
}
