extern crate tanton;
extern crate tanton_engine;
use tanton::Board;
use tanton_engine::engine::TantonSearcher;
use tanton_engine::time::uci_timer::PreLimits;

fn main() {
    let mut limit = PreLimits::blank();
    limit.depth = Some(8);
    let mut board = Board::start_pos();
    board.pretty_print();
    let mut s = TantonSearcher::init(false);

    for _ in 1..=1 {
        s.search(&board, &limit);
        let bit_move = s.await_move();
        board.apply_move(bit_move);
        board.pretty_print();
    }
}
