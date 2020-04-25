mod clock;

use iced::{Settings, Application};

use clock::Clock;

fn main() {
    Clock::run(Settings::default());
}
