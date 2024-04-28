use rahjong::{hand::Hand, rules::jp::GameRound, tile::*};
use rand::SeedableRng;

fn main() {
    use std::sync::mpsc::channel;
    const EPOCH: usize = 1_000_000;
    const EPOCH_NAME: &str = "Million(s)";
    let thread = num_cpus::get();
    let (tx, rx) = channel::<()>();
    let (hand_tx, hand_rx) = channel::<Hand>();
    let mut counter = 0;
    let mut rolls = vec![];
    for t in 0..thread {
        let tx = tx.clone();
        let hand_tx = hand_tx.clone();
        let handle = std::thread::spawn(move || {
            let hand = loop {
                let round = GameRound::new(
                    Wind::East,
                    Wind::East,
                    counter,
                    rand::rngs::StdRng::from_entropy()
                );
                let hand = &round.deck(Wind::East).hand;

                if hand.can_thirteen_orphans() {
                    break hand.clone();
                }
                counter += 1;
                if counter % EPOCH == 0 {
                    tx.send(()).expect("Failed to send signal");
                };
            };
            hand_tx.send(hand).expect("Failed to send result");
        });
        rolls.push(handle);
    }
    let output = std::thread::spawn(move || {
        let mut count = 0;
        while let Ok(_) = rx.recv() {
            count += 1;
            println!("Total Round: {count} {EPOCH_NAME}");
        }

        println!("Total Round: {count}");
    });
    let hand = hand_rx.recv().unwrap();
    println!("Hand: {hand}");

}

#[test]
fn test_roll_until_thirteen_orphans() {
    macro_rules! tile {
        ($face:ident, $index:expr) => {
            TileId::from_face_idx($face, TileIndex::const_from_u8($index))
        };
    }
    let thirteen_orphans = [
        tile!(B1, 0),
        tile!(B9, 0),
        tile!(C1, 0),
        tile!(C9, 0),
        tile!(D1, 0),
        tile!(D9, 0),
        tile!(WEST, 0),
        tile!(EAST, 0),
        tile!(SOUTH, 0),
        tile!(SOUTH, 1),
        tile!(WHITE, 0),
        tile!(GREEN, 0),
        tile!(RED, 0),
    ];
    let hand = Hand::new(thirteen_orphans);
    assert!(hand.can_thirteen_orphans());
}
