fn main() {
    let s = String::from("hello this is rust ");
    let x = 66;
    take_ownership(s);

    makes_copy(x);

    println!("the values of s is :{x}");
    //下面一行無法編譯通過，因為S已經被傳入take_ownership方法且作用域完畢後被釋放了
    //println!("the values of s is :{s}");

}

fn take_ownership(some_string:String ){
    println!("the value of some_thing is : {some_string}");
}

fn makes_copy(some_interger:i32){
    println!("the value of some_thing is : {some_interger}");
}
