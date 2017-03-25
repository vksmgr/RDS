pub fn run() {
    bot_trust();
}

//Bot Trust

fn bot_trust() {
    let test_cases: usize = read!();

    for case in 0..test_cases {
        logic(case);
    }
}

enum OrgBlue {
    O,
    B,
}

struct Button {
    typ: OrgBlue,
    b_numb: i8
}


fn logic(case: usize) {
    let mut input: Vec<Button> = Vec::new();
    let buttons: usize = read!();
    let mut orange_robo: i8 = 1;
    let mut blue_robo: i8 = 1;
    //get input
    for i in 0..buttons {
        let tmp: String = read!("{}");
        let tmp2: i8 = read!();
        if tmp == "O" {
            input.push(Button { typ: OrgBlue::O, b_numb: tmp2 });
        }
        if tmp == "B" {
            input.push(Button { typ: OrgBlue::B, b_numb: tmp2 });
        }
    }

    for i in 0..buttons {
        /*match input.pop() {
            Some(t) => println!("{} {}", t.b_numb, match t.typ {
                OrgBlue::O => "Orange",
                OrgBlue::B => "Blue",
            }),
            None => println!("Error Occur"),
        }*/

        if let Some(inp) = input.pop() {
           match inp.typ {
               OrgBlue::O => { println!("{}, {}", inp.b_numb, "Orange");},
               OrgBlue::B => { println!("{}, {}", inp.b_numb, "Blue");},
           }
        }
    }

}