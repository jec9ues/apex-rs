pub mod offsets {
    pub const CL_ENTITYLIST: u64 = 0x1e23418;
    // [Miscellaneous] -> cl_entitylist
    pub const LOCAL_PLAYER: u64 = 0x21d3758;
    // LocalPlayer
    pub const NAME_LIST: u64 = 0xc266a60;
    // [Miscellaneous] -> NameList
    pub const VIEW_RENDER: u64 = 0x74210a8;
    // ViewRender
    pub const VIEW_MATRIX: u64 = 0x11a350;
    // ViewMatrix
    pub const CAMERA_POSITION: u64 = 0x1e90;
    //[Miscellaneous] -> CPlayer!camera_origin
    pub const LEVEL_NAME: u64 = 0x16f71e0;
    // [Miscellaneous] -> LevelName
    pub const GLOW_COLOR: u64 = 0x200;
    // [Miscellaneous] -> glow_color
    pub const GLOW_TYPE: u64 = 0x29c;
    // Script_Highlight_GetState + 4 2f4  0x2C4 + 0x30 + 0x4
    pub const GLOW_ENABLE: u64 = 0x294;
    // [RecvTable.DT_HighlightSettings] -> `m_highlightServerContextID` + 0x8 (0x03f0 + 0x8)
    pub const GLOW_THROUGH_WALL: u64 = 0x278;
    // [RecvTable.DT_HighlightSettings] -> m_highlightServerContextID + 0x10 (0x03f0 + 0x10)
    pub const GLOW_DISTANCE: u64 = 0x26c;
    // Script_Highlight_SetFarFadeDist or m_highlightServerFadeEndTimes + 52(0x34)
    pub const TEAM_NUM: u64 = 0x037c;
    // [RecvTable.DT_BaseEntity] -> m_iTeamNum
    pub const HEALTH: u64 = 0x036c;
    // [RecvTable.DT_Player] m_iHealth
    pub const MAX_HEALTH: u64 = 0x04a8;
    // [RecvTable.DT_Player] m_iMaxHealth
    pub const SHIELD: u64 = 0x01a0;
    // [RecvTable.DT_TitanSoul] m_shieldHealth
    pub const MAX_SHIELD: u64 = 0x01a4;
    // [RecvTable.DT_TitanSoul] m_shieldHealthMax
    pub const LIFE_STATE: u64 = 0x06c8;
    // [RecvTable.DT_Player] -> m_lifeState
    pub const BLEED_OUT_STATE: u64 = 0x26a0;
    // [RecvTable.DT_Player] -> m_bleedoutState
    pub const STUDIOHDR: u64 = 0x1020;
    //CBaseAnimating!m_pStudioHdr
    pub const BONE: u64 = 0x0dd0 + 0x48;
    // m_nForceBone + 0x48
    pub const LOCAL_ORIGIN: u64 = 0x0188;
    // [DataMap.CBaseViewModel] -> m_localOrigin
    pub const ABS_VECTORORIGIN: u64 = 0x17c;
    // [DataMap.CBaseViewModel] -> m_vecAbsOrigin
    pub const SIGN_NAME: u64 = 0x04b0 + 0x9;
    // m_iSignifierName
    pub const ITEM_ID: u64 = 0x1578;
    // m_customScriptInt
    pub const AMMO: u64 = 0x15a0;
    // [RecvTable.DT_WeaponX_LocalWeaponData] -> m_ammoInClip
    pub const WEAPON: u64 = 0x1954;
    // m_latestPrimaryWeapons
    pub const WEAPON_NAME: u64 = 0x1584;
    // [RecvTable.DT_WeaponX] -> m_weaponNameIndex
    pub const WEAPON_SETTING_BASE: u64 = 0x19c8;
    // [WeaponSettingsMeta] -> base
    pub const BULLET_SPEED: u64 = WEAPON_SETTING_BASE + 0x04d4;
    // CWeaponX!m_flProjectileSpeed [WeaponSettingsMeta]
    pub const BULLET_SCALE: u64 = WEAPON_SETTING_BASE + 0x4dc;
    // CWeaponX!m_flProjectileScale [WeaponSettingsMeta]
    pub const ZOOM_FOV: u64 = 0x15f0 + 0x00b8;
    // m_playerData + m_curZoomFOV
    pub const SEMI_AUTO: u64 = WEAPON_SETTING_BASE + 0x018c;
    // [WeaponSettingsMeta] is_semi_auto / m_isSemiAuto


    pub const OFFSET_HIGHLIGHTSETTINGS: u64 = 0xb5f9620;
    pub const OFFSET_HIGHLIGHTSERVERACTIVESTATES: u64 = 0x298;
    pub const OFFSET_HIGHLIGHTCURRENTCONTEXTID: u64 = 0x294;
    pub const OFFSET_HIGHLIGHTVISIBILITYTYPE: u64 = 0x278;

    pub const VIEW_ANGLE: u64 = 0x24f4 - 0x14;
    // [DataMap.C_Player] -> m_ammoPoolCapacity - 0x14
    
}