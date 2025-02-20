fn main() {
    let s: Option<char> = None;
    let ss: Option<char> = None;

    loop {
        match s {
            Some(_) => {
                break;
            }
            _ => match ss {
                Some(_) => {
                    break;
                }
                _ => {
                    println!("fd");
                    break;
                }
            },
        }
    }
}
