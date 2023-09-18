pub mod offsets {
    pub const CL_ENTITYLIST: u64 = 0x1e754c8;
    // [Miscellaneous] -> cl_entitylist
    pub const NAME_LIST: u64 = 0xc2b1c00;
    // [Miscellaneous] -> NameList
    pub const NAME_INDEX: u64 = 0x06c8;
    // [RecvTable.DT_BaseEntity] m_scriptNameIndex
    pub const LIFE_STATE: u64 = 0x07d0;
    // [RecvTable.DT_Player] -> m_lifeState
    pub const BLEED_OUT_STATE: u64 = 0x2790;
    // m_bleedoutState
    pub const LEVEL_NAME: u64 = 0x16efe10;
    // [Miscellaneous] -> LevelName
    pub const LOCAL_ORIGIN: u64 = 0x0188;
    // [DataMap.CBaseViewModel] -> m_localOrigin
    pub const GLOW_COLOR: u64 = 0x200;
    // [Miscellaneous] -> glow_color
    pub const GLOW_TYPE: u64 = 0x2C4 + 0x30 + 0x4;
    // Script_Highlight_GetState + 4 2f4
    pub const GLOW_ENABLE: u64 = (0x03f0 + 0x8);
    // [RecvTable.DT_HighlightSettings] -> m_highlightServerContextID + 0x8
    pub const GLOW_THROUGH_WALL: u64 = (0x03f0 + 0x10);
    // [RecvTable.DT_HighlightSettings] -> m_highlightServerContextID + 0x10
    pub const GLOW_DISTANCE: u64 = 0x3E4;
    // Script_Highlight_SetFarFadeDist or m_highlightServerFadeEndTimes + 52(0x34)
    pub const TEAM_NUM: u64 = 0x0480;
    // [RecvTable.DT_BaseEntity] ->m_iTeamNum
    pub const HEALTH: u64 = 0x0470;
    // m_iHealth
    pub const MAX_HEALTH: u64 = 0x05b0;
    // [RecvTable.DT_Player] m_iMaxHealth
    pub const SHIELD: u64 = 0x01a0;
    // [RecvTable.DT_TitanSoul] m_shieldHealth
    pub const MAX_SHIELD: u64 = 0x01a4;
    // [RecvTable.DT_TitanSoul] m_shieldHealthMax

    pub const STUDIOHDR: u64 = 0x1118;
    //CBaseAnimating!m_pStudioHdr
    pub const BONE: u64 = 0x0ec8 + 0x48;
    // m_nForceBone + 0x48
    pub const ABS_VECTORORIGIN: u64 = 0x17c;
    // m_vecAbsOrigin
    pub const SIGN_NAME: u64 = 0x05b8 + 0x9;
    // m_iSignifierName
    pub const VIEW_RENDER: u64 = 0x7473a28;
    // ViewRender
    pub const VIEW_MATRIX: u64 = 0x11a350;
    // ViewMatrix
    pub const ITEM_ID: u64 = 0x1668;
    // m_customScriptInt
    pub const AMMO: u64 = 0x1690;
    // [RecvTable.DT_WeaponX_LocalWeaponData] -> m_ammoInClip
    pub const WEAPON: u64 = 0x1a44;
    // m_latestPrimaryWeapons
    pub const WEAPON_NAME: u64 = 0x1674;
    // [RecvTable.DT_WeaponX].m_weaponNameIndex
    pub const BULLET_SPEED: u64 = 0x1AA0 + 0x04cc;
    // CWeaponX!m_flProjectileSpeed [WeaponSettingsMeta]
    pub const BULLET_SCALE: u64 = 0x1AA0 + 0x4d4;
    // CWeaponX!m_flProjectileScale [WeaponSettingsMeta]
    pub const ZOOM_FOV: u64 = 0x16e0 + 0x00b8;
    // m_playerData + m_curZoomFOV
    pub const SEMI_AUTO: u64 = 0x1AA0 + 0x018c;
    // m_isSemiAuto [WeaponSettingsMeta]
    pub const LOCAL_PLAYER: u64 = 0x2225648; // LocalPlayer
}