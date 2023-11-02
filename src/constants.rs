pub mod offsets {
    pub const CL_ENTITYLIST: u64 = 0x1d6b5d8;
    // [Miscellaneous] -> cl_entitylist
    pub const LOCAL_PLAYER: u64 = 0x2119848;
    // LocalPlayer
    pub const NAME_LIST: u64 = 0xc26e750;
    // [Miscellaneous] -> NameList
    pub const VIEW_RENDER: u64 = 0x73828a0;
    // ViewRender
    pub const VIEW_MATRIX: u64 = 0x11a350;
    // ViewMatrix
    pub const INPUT_SYSTEM: u64 = 0x170f080;
    pub const CAMERA_POSITION: u64 = 0x1f00;
    //[Miscellaneous] -> CPlayer!camera_origin
    pub const CAMERA_ANGLES: u64 = 0x1f0c;
    //[Miscellaneous] -> CPlayer!CAMERA_ANGLES=0x1e9c
    pub const LEVEL_NAME: u64 = 0x1690670;
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
    pub const TEAM_MEMBER_INDEX: u64 = 0x0384;
    // [RecvTable.DT_BaseEntity] -> m_teamMemberIndex=0x0384


    pub const HEALTH: u64 = 0x036c;
    // [RecvTable.DT_Player] m_iHealth
    pub const MAX_HEALTH: u64 = 0x04a8;
    // [RecvTable.DT_Player] m_iMaxHealth
    pub const HELMET_TYPE: u64 = 0x4630;
    // [RecvTable.DT_Player] m_helmetType=0x45c0
    pub const ARMOR_TYPE: u64 = 0x4634;
    // [RecvTable.DT_Player] m_armorType=0x45c4
    pub const CURRENT_FRAMEMODEL_INDEX: u64 = 0x00d8;
    // [DataMap.C_BaseEntity] m_currentFrame.modelIndex=0x00d8

    pub const LAST_VISIBLE_TIME: u64 = 0x19C0;
    //CPlayer!LAST_VISIBLE_TIME
    pub const SHIELD: u64 = 0x01a0;
    // [RecvTable.DT_TitanSoul] m_shieldHealth
    pub const MAX_SHIELD: u64 = 0x01a4;
    // [RecvTable.DT_TitanSoul] m_shieldHealthMax
    pub const LIFE_STATE: u64 = 0x06c8;
    // [RecvTable.DT_Player] -> m_lifeState
    pub const BLEED_OUT_STATE: u64 = 0x2710;
    // [RecvTable.DT_Player] -> m_bleedoutState
    pub const PLATFORM_USER_ID: u64 = 0x2578;
    // [RecvTable.DT_Player] -> m_platformUserId=0x2578

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
    pub const ITEM_ID: u64 = 0x1588;
    // m_customScriptInt

    // weapon
    pub const AMMO: u64 = 0x15a0;
    // [RecvTable.DT_WeaponX_LocalWeaponData] -> m_ammoInClip
    pub const WEAPON: u64 = 0x1964;
    // m_latestPrimaryWeapons
    pub const SELECTED_SLOT: u64 = 0x1974;
    // m_latestNonOffhandWeapons
    pub const NST_WEAPON_NAMES: u64 = 0x073831c0;
    // WeaponNames=0x07420760
    pub const WEAPON_NAME: u64 = 0x17a8;
    // [RecvTable.DT_WeaponX] -> m_weaponNameIndex
    pub const PLAYER_DATA: u64 = 0x1600;
    // [RecvTable.DT_WeaponX] m_playerData=0x15f0
    pub const WEAPON_SETTING_BASE: u64 = 0x19e0;
    // [WeaponSettingsMeta] -> base
    pub const BULLET_SPEED: u64 = WEAPON_SETTING_BASE + 0x04d4;
    // CWeaponX!m_flProjectileSpeed [WeaponSettingsMeta]
    pub const BULLET_SCALE: u64 = WEAPON_SETTING_BASE + 0x4dc;
    // CWeaponX!m_flProjectileScale [WeaponSettingsMeta]
    pub const ZOOM_FOV: u64 = PLAYER_DATA + 0x00b8;
    // m_playerData + m_curZoomFOV
    pub const SEMI_AUTO: u64 = WEAPON_SETTING_BASE + 0x018c;
    // [WeaponSettingsMeta] is_semi_auto / m_isSemiAuto
    pub const BITFIELD_FROM_PLAYER: u64 = 0x1734;
    // m_modBitfieldFromPlayer=0x1724
    pub const BITFIELD_INTERNAL: u64 = 0x1738;
    // m_modBitfieldInternal=0x1728
    pub const BITFIELD_CURRENT: u64 = 0x173c;
    // m_modBitfieldCurrent=0x172c
    pub const BITFIELD_DISABLED: u64 = 0x1740;
    // m_modBitfieldDisabled=0x1730

    pub const OFFSET_HIGHLIGHTSETTINGS: u64 = 0xB5C4090;
    pub const OFFSET_HIGHLIGHTSERVERACTIVESTATES: u64 = 0x298;
    pub const OFFSET_HIGHLIGHTCURRENTCONTEXTID: u64 = 0x294;
    pub const OFFSET_HIGHLIGHTVISIBILITYTYPE: u64 = 0x278;

    pub const VIEW_ANGLE: u64 = 0x2564 - 0x14;
    // [DataMap.C_Player] -> m_ammoPoolCapacity - 0x14
    
}



pub const SERVER_PUBLIC_PEM: &'static str = "-----BEGIN PUBLIC KEY-----
MFwwDQYJKoZIhvcNAQEBBQADSwAwSAJBAMysAtpmr0mkb3V8rCbysLdGsTqhvDac
+6I8juHMnHR1V4bi6QI9cBhWOYtbnGvjv06Cap8iaSb0YE4PfB7qeFkCAwEAAQ==
-----END PUBLIC KEY-----";

pub const CLIENT_PRIVATE_PEM: &'static str = "-----BEGIN PRIVATE KEY-----
MIIBUgIBADANBgkqhkiG9w0BAQEFAASCATwwggE4AgEAAkEAuF1sjDy8cDuLsFYd
6z/jadOSy1PFrFw/rjJCR34ZEQfY+wyCxoQ6/rAcoIrz7mm/nByLgU4IXTtf1xqp
FN/CswIDAQABAkBNyUUaKioLZGQ8oyj+b7dAmD7QQOkfHtDTa2shhOeIkkK7eGhm
/ZrwkUZAF6CoDSKGn9mSuH9o+vU77QHoEoM5AiEA2nCEEI8nm+T7ErZkRdkXNWJH
OEvfHr3kSykAcdjGrH8CIQDYEPrl24BXKYSvPsNvELkWmg9kFyrYPYE47V7/M/vf
zQIgUv8vXq9V6GjfcOO2Q8fRhqoZz20ckKysAlbI4XjxO/cCHyKalZQ6k1NiUt2+
L46aHwp/ofdlvnbjYs67NXup160CIAqs7+qNppTJ7lFIM6vrjzUoXXXhksBGVRkc
lSPOHXeb
-----END PRIVATE KEY-----";
pub const CLIENT_PUBLIC_PEM: &'static str = "-----BEGIN PUBLIC KEY-----
MFwwDQYJKoZIhvcNAQEBBQADSwAwSAJBALhdbIw8vHA7i7BWHes/42nTkstTxaxc
P64yQkd+GREH2PsMgsaEOv6wHKCK8+5pv5wci4FOCF07X9caqRTfwrMCAwEAAQ==
-----END PUBLIC KEY-----";