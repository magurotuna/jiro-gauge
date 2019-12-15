use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

const BAR_MAX: u64 = 100;
const DUR_BASE: u64 = 10;

fn main() {
    let m = MultiProgress::new();
    let style = ProgressStyle::default_bar()
        .template("{msg:<15}\t{bar:140.cyan/blue}")
        .progress_chars("##-");

    let pb = m.add(ProgressBar::new(BAR_MAX));
    pb.set_style(style.clone());
    pb.set_message("ラーメン二郎");
    let _ = thread::spawn(move || {
        for _ in 0..BAR_MAX {
            pb.inc(1);
            thread::sleep(Duration::from_millis(2 * DUR_BASE));
        }
    });

    let pb = m.add(ProgressBar::new(BAR_MAX));
    pb.set_style(style.clone());
    pb.set_message("千里眼");
    let _ = thread::spawn(move || {
        for _ in 0..BAR_MAX {
            pb.inc(1);
            thread::sleep(Duration::from_millis(3 * DUR_BASE));
        }
    });

    let pb = m.add(ProgressBar::new(BAR_MAX));
    pb.set_style(style.clone());
    pb.set_message("蒙古タンメン中本");
    let _ = thread::spawn(move || {
        for _ in 0..BAR_MAX {
            pb.inc(1);
            thread::sleep(Duration::from_millis(3 * DUR_BASE));
        }
    });

    let pb = m.add(ProgressBar::new(BAR_MAX));
    pb.set_style(style.clone());
    pb.set_message("ラーメン池田屋");
    let _ = thread::spawn(move || {
        for _ in 0..BAR_MAX {
            pb.inc(1);
            thread::sleep(Duration::from_millis(2 * DUR_BASE));
        }
    });
    m.join_and_clear().unwrap();
}
