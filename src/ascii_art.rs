pub(crate) struct Art {
    pub(crate) big: &'static str,
    pub(crate) small: &'static str,
}

pub(crate) struct AllArt {
    pub(crate) arch: Art,
    pub(crate) windows: Art,
}

pub(crate) const ALL_ART: AllArt = AllArt {
    arch: Art {
        big: include_str!("ascii_art/arch/big.ascii"),
        small: include_str!("ascii_art/arch/small.ascii"),
    },
    windows: Art {
        big: include_str!("ascii_art/windows/big.ascii"),
        small: include_str!("ascii_art/windows/small.ascii"),
    },
};
