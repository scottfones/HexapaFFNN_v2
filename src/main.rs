extern crate ndarray;

pub mod game;

fn main() {
    let g0 = game::GameState::new_game();
    println!("{}", g0);

    // Max move 1
    let max0 = game::Location { m: 0, n: 0 };
    if max0.check_advance(&g0) {
        let g1 = g0.advance(max0);
        println!("Move 1:{}", g1);

        // Min move 1
        let min0 = game::Location { m: 2, n: 2 };
        if min0.check_advance(&g1) {
            let g2 = g1.advance(min0);
            println!("Move 2:{}", g2);

            // Max move 2
            let max1 = game::Location { m: 1, n: 0 };
            if max1.check_capture_left(&g2) {
                println!("{}", g2);
            } else {
                println!("Capture Left Failed.\n");

                if max1.check_capture_right(&g2) {
                    let g3 = g2.capture_right(max1);
                    println!("Move 3:{}", g3);
                }
            }
        }
    }

    println!("{}", g0);
    let acts = g0.actions();
    for a in &acts {
        println!("{}", a);
    }

    let s1 = g0.advance(game::Location { m: 0, n: 0 });
    println!("{}", s1);
    s1.actions();

    let s2 = s1.capture_left(game::Location { m: 2, n: 1 });
    println!("{}", s2);
    s2.actions();

    let s3 = s2.advance(game::Location { m: 0, n: 1 });
    println!("{}", s3);
    s3.actions();

    let s4 = s3.advance(game::Location { m: 2, n: 2 });
    println!("{}", s4);
    s4.actions();

    let s5 = s4.advance(game::Location { m: 1, n: 1 });
    println!("{}", s5);
    s5.actions();

    println!("{}", s5.is_terminal());
    s5.to_vector();
}
