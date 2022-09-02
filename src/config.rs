use std::sync::Mutex;

struct Config {
    target: TranspileTarget,
}

enum TranspileTarget {
    GROOVY,
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config {
        target: TranspileTarget::GROOVY
    });
}
