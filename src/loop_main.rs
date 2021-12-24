fn loop_main() {
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        println!("\'{}\'", ch);
        i += 1;
    };
    println!(" \'O\' 的索引为 {}", location);
}