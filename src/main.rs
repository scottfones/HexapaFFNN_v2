extern crate ndarray;

pub mod game;

fn main() {
    let g0 = game::new_game();
    println!("{}", g0);

    // Max move 1
    let max0 = game::Location { m: 0, n: 0 };
    if max0.check_advance(&g0) {
        let g1 = g0.advance(max0);
        println!("Move 1:{}", g1);

        // Min move 1
        let min0 = game::Location { m: 2, n: 2};
        if min0.check_advance(&g1) {
            let g2 = g1.advance(min0);
            println!("Move 2:{}", g2);
        
    
            // Max move 2
            let max1 = game::Location { m: 1, n: 0};
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
}
