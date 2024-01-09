pub mod color {

    #[derive(Default, Copy, Clone, Debug)]
    pub struct Rgb(pub [u8; 3]);

    impl From<Lab> for Rgb {
        fn from(value: Lab) -> Self {
            todo!()
        }
    }

    #[derive(Default, Copy, Clone, Debug)]
    pub struct Lab([u8; 3]);

    impl From<Rgb> for Lab {
        fn from(value: Rgb) -> Self {
            todo!()
        }
    }
}
