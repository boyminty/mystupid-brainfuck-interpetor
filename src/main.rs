use libc::{getchar, putchar};
#[allow(dead_code)]
enum InterpreterState {
    Backward,
    Forward,
}
fn main() {
    let source = std::fs::read_to_string("src/hello.bf")
        .unwrap()
        .split(|f| f == '\n' || f == '\r' || f == ' ')
        .filter(|s| !s.is_empty())
        .into_iter()
        .map(|str| str.to_string())
        .collect::<String>()
        .chars()
        .filter(|p| {
            *p == '+'
                || *p == '-'
                || *p == ','
                || *p == '.'
                || *p == '['
                || *p == ']'
                || *p == '<'
                || *p == '>'
        })
        .collect::<Vec<char>>();
    // let source = ",+++----".chars().collect::<Vec<char>>();

    let mut index = 0;
    let mut pointerindex = 0;
    let mut cells: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut prevjumpnonzero: Option<usize> = None;
    let mut prevjumpzero: Option<usize> = None;

    let mut inter_state = InterpreterState::Forward;
    println!("{:?}", source);
    loop {
        match source[index] {
            '+' => cells[pointerindex] += 1,
            '-' => cells[pointerindex] -= 1,
            '>' => pointerindex += 1,
            '<' => pointerindex -= 1,
            ',' => unsafe {
                cells[pointerindex] = getchar();
                while getchar() != 10 {}
            },
            '.' => unsafe {
                putchar(cells[pointerindex]);
            },
            ']' => {
                if cells[pointerindex] == 0 {
                    prevjumpzero = Some(index);
                } else {
                    match prevjumpnonzero {
                        Some(point) => {
                            index = point;
                        }
                        None => {
                            panic!("this is wrong")
                        }
                    }
                }
            }
            '[' => {
                if cells[pointerindex] != 0 {
                    prevjumpnonzero = Some(index);
                } else {
                    match prevjumpzero {
                        Some(point) => {
                            index = point;
                        }
                        None => {
                            panic!("something is wrong")
                        }
                    }
                }
            }
            _ => {}
        }
        match inter_state {
            InterpreterState::Backward => index -= 1,
            InterpreterState::Forward => index += 1,
        }
        // println!("{:?}", cells);

        if index >= source.len() {
            break;
        }
    }
}
