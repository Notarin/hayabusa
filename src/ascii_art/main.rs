pub(crate) struct Art {
    pub(crate) big: &'static str,
    pub(crate) small: &'static str,
}

pub(crate) struct AllArt {
    pub(crate) arch: Art,
    pub(crate) windows: Art,
    pub(crate) ubuntu: Art,
    pub(crate) fallback: Art,
    pub(crate) gentoo: Art,
    pub(crate) nixos: Art,
}

pub(crate) const ALL_ART: AllArt = AllArt {
    arch: Art {
        big: include_str!("art_collection/arch/big.ascii"),
        small: include_str!("art_collection/arch/small.ascii"),
    },
    windows: Art {
        big: include_str!("art_collection/windows/big.ascii"),
        small: include_str!("art_collection/windows/small.ascii"),
    },
    ubuntu: Art {
        big: include_str!("art_collection/ubuntu/big.ascii"),
        small: include_str!("art_collection/ubuntu/small.ascii"),
    },
    fallback: Art {
        big: include_str!("art_collection/fallback/big.ascii"),
        small: include_str!("art_collection/fallback/small.ascii"),
    },
    gentoo: Art {
        big: include_str!("art_collection/gentoo/big.ascii"),
        small: include_str!("art_collection/gentoo/small.ascii"),
    },
    nixos: Art {
        big: include_str!("art_collection/nixos/big.ascii"),
        small: include_str!("art_collection/nixos/small.ascii"),
    },
};
