pub mod srgb {
    use palette::Srgb;

    pub static BLACK: Srgb = Srgb::new(0.0, 0.0, 0.0);
    pub static WHITE: Srgb = Srgb::new(1.0, 1.0, 1.0);

    // primary colours
    pub static RED: Srgb = Srgb::new(1.0, 0.0, 0.0);
    pub static GREEN: Srgb = Srgb::new(0.0, 1.0, 0.0);
    pub static BLUE: Srgb = Srgb::new(0.0, 0.0, 1.0);

    // secondary colours
    pub static YELLOW: Srgb = Srgb::new(1.0, 1.0, 0.0);
    pub static PURPLE: Srgb = Srgb::new(1.0, 0.0, 1.0);
    pub static CYAN: Srgb = Srgb::new(0.0, 1.0, 1.0);

    // other
    pub static PINK: Srgb = Srgb::new(1.0, 0.6, 0.8);
    pub static MAGENTA: Srgb = Srgb::new(1.0, 0.15, 0.8);
    pub static ROSE: Srgb = Srgb::new(1.0, 0.0, 0.59);

    pub static GOLD: Srgb = Srgb::new(1.0, 0.8, 0.16);
    pub static ORANGE: Srgb = Srgb::new(1.0, 0.4, 0.0);
    pub static RUST: Srgb = Srgb::new(0.7, 0.2, 0.0);

    pub static AQUAMARINE: Srgb = Srgb::new(0.0, 1.0, 0.6);
}