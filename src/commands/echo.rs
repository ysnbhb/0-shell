pub fn echo(strs: &[String]) {
    strs.iter().for_each(|s| {
        let s = s.replace("\\n", "\n");
        print!("{s} ");
    });
    println!()
}
