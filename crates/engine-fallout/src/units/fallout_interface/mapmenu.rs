//! `fallout/interface/menus/mapmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSStringT@D@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01075530: u32 = 0x0107_5530;
/// `007a1c30` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_007A1C30: u32 = 0x007a_1c30;

// Translated from 007a1aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSStringT@D@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `007a1c30(this, 1)`.
pub fn fn_007a1aa0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01075530);
    e.call(ARRAY_RELEASE_007A1C30, &args![this, 1u32]);
}

use crate::types::BSSimpleArray;
use crate::units::fallout_interface::startmenu::ListBox;

layout! {
    /// `MapMenu` (Xbox PDB), 0x230 bytes; the PC constructor and destructor
    /// use the Xbox offsets. `pTiles` (22 tile pointers) starts at +0x28 and
    /// is read through [`map_tile`].
    pub struct MapMenu: 0x230 {
        /// `iCurrentPage` (Xbox PDB): the tab (0x20 to 0x24).
        0x80 iCurrentPage: i32,
        /// `pMapTile` (Xbox PDB).
        0x84 pMapTile: u32,
        /// `fWaveformAnimationStartTime` (Xbox PDB).
        0x88 fWaveformAnimationStartTime: f32,
        /// `bShowingAllNotes` (Xbox PDB).
        0x8C bShowingAllNotes: u8,
        /// `pSelectedNote` (Xbox PDB).
        0x90 pSelectedNote: u32,
        /// `uiNoteEnterStartTime` (Xbox PDB).
        0x94 uiNoteEnterStartTime: u32,
        /// `pCurrentSound` (Xbox PDB): a `BSSimpleList<BSSoundHandle>` node.
        0xB8 pCurrentSound: u32,
        /// `bSoundsLoading` (Xbox PDB).
        0xBC bSoundsLoading: u8,
        /// `iSoundPlayStartTime` (Xbox PDB).
        0xC4 iSoundPlayStartTime: u32,
        /// `fOldPipboyRadioVolume` (Xbox PDB).
        0xC8 fOldPipboyRadioVolume: f32,
        /// `pDisplayedQuest` (Xbox PDB).
        0xCC pDisplayedQuest: u32,
        /// `pDisplayedQuestTarget` (Xbox PDB).
        0xD0 pDisplayedQuestTarget: u32,
        /// `pHighlightedMapMarker` (Xbox PDB).
        0xD4 pHighlightedMapMarker: u32,
        /// `PlayerMarkerSpace` (Xbox PDB).
        0xF8 PlayerMarkerSpace: u32,
        /// `PlayerMarkerPos` (Xbox PDB): `NiPoint3`, x (y at +0x100, z at
        /// +0x104).
        0xFC PlayerMarkerPos: f32,
        /// `pMapSpace` (Xbox PDB).
        0x108 pMapSpace: u32,
        /// `pRootMapSpace` (Xbox PDB).
        0x10C pRootMapSpace: u32,
        /// `bRootSpaceChanged` (Xbox PDB).
        0x110 bRootSpaceChanged: u8,
        /// `pMapPlayerRef` (Xbox PDB).
        0x114 pMapPlayerRef: u32,
        /// `pFastTravelMarker` (Xbox PDB).
        0x118 pFastTravelMarker: u32,
        /// `pAdjustNorthRotationCell` (Xbox PDB).
        0x11C pAdjustNorthRotationCell: u32,
        /// `WorldMapDragStart` (Xbox PDB): `NiPoint2`, x (y at +0x124).
        0x120 WorldMapDragStart: f32,
        /// `uiNextLocalMapGrid` (Xbox PDB).
        0x128 uiNextLocalMapGrid: u32,
        /// `bFogOfWarEnabled` (Xbox PDB).
        0x12C bFogOfWarEnabled: u8,
    }
}

/// `MapMenu::pMe` (Xbox PDB static): the open map menu, or 0.
const MAP_MENU_SINGLETON: u32 = 0x011d_a368;
/// `MapMenu`'s vtable (RTTI `.?AVMapMenu@@`).
const MAP_MENU_VTABLE: u32 = 0x0107_4d44;
/// The byte `ClearWorldMapTexture` sets once it has cleared the texture.
const TEXTURE_CLEARED_011DA36C: u32 = 0x011d_a36c;
/// A byte that selects the challenge list over the notes list (the Xbox PDB
/// has a static `bNotesChallengeMode` in `MapMenu`).
const CHALLENGE_MODE_011DA36D: u32 = 0x011d_a36d;
/// A signed byte the challenge filter steps through 0, 1, 2 (the Xbox PDB has
/// a static `cChallengeTypeDisplayed`).
const CHALLENGE_TYPE_011DA36E: u32 = 0x011d_a36e;
/// The byte `DoIdle` sets when it pauses the note sound.
const SOUND_PAUSED_011DA36F: u32 = 0x011d_a36f;
/// The time (`00825c00` on `011f6394`) `DoIdle` keeps when it pauses the note
/// sound.
const PAUSE_TIME_011DA40C: u32 = 0x011d_a40c;
/// The byte `DoIdle` sets after giving the pip-boy tile trait `0xfa9` 255.0.
const FADED_011DA408: u32 = 0x011d_a408;
/// The table of 403df0 objects (one per challenge type) at `011a03e4`.
const CHALLENGE_TYPE_NAMES_011A03E4: u32 = 0x011a_03e4;
/// The word `007a1480` returns.
const VALUE_011DEA14: u32 = 0x011d_ea14;
/// The player (`PlayerCharacter *` the code passes to `SetActiveQuest`).
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
/// The object `00877720` is called on in `DoIdle`.
const OBJECT_011DEA0C: u32 = 0x011d_ea0c;
/// The calendar (`Calendar` at `011de7b8`).
const CALENDAR_011DE7B8: u32 = 0x011d_e7b8;
/// The timer object `00825c00` is called on.
const TIMER_011F6394: u32 = 0x011f_6394;
/// The three floats the constructor copies into `PlayerMarkerPos`.
const POINT_011F426C: u32 = 0x011f_426c;
/// The objects whose word +4 (`00403df0`) `Create` passes to the tab line,
/// in the order the code reads them.
const TABLINE_ENTRIES: [u32; 5] = [
    0x011d_4aec,
    0x011d_28a4,
    0x011d_5020,
    0x011d_204c,
    0x011d_38ac,
];

/// The trait ids the constructor looks up (`MapMenu` statics `iUT_Title`,
/// `iUT_Value`, `iUT_CurrentTab`, `iUT_Selected`, `iUT_TextAlpha`, `iUT_X`,
/// `iUT_Y`, `iUT_Magnification` in the Xbox PDB).
const TRAIT_TITLE: u32 = 0x011d_a354;
const TRAIT_VALUE: u32 = 0x011d_a358;
const TRAIT_CURRENT_TAB: u32 = 0x011d_a360;
const TRAIT_SELECTED: u32 = 0x011d_a348;
const TRAIT_TEXT_ALPHA: u32 = 0x011d_a344;
const TRAIT_X: u32 = 0x011d_a350;
const TRAIT_Y: u32 = 0x011d_a34c;
const TRAIT_MAGNIFICATION: u32 = 0x011d_a35c;

/// `"_Title"`, `"_Value"`, `"_CurrentTab"`, `"_selected"`, `"_TextAlpha"`,
/// `"_x"`, `"_y"`, `"_Magnification"`.
const TITLE_TEXT: u32 = 0x0106_ed20;
const VALUE_TEXT: u32 = 0x0106_ed18;
const CURRENT_TAB_TEXT: u32 = 0x0107_3994;
const SELECTED_TEXT: u32 = 0x0107_1b64;
const TEXT_ALPHA_TEXT: u32 = 0x0107_3cb4;
const X_COORD_TEXT: u32 = 0x0106_ec90;
const Y_COORD_TEXT: u32 = 0x0106_f8f4;
const MAGNIFICATION_TEXT: u32 = 0x0107_4d2c;
/// `"Data\\Menus\\Main\\map_menu.xml"`.
const MAP_MENU_XML: u32 = 0x0107_4df8;
/// `"MENUS: Map Menu Creation Failed... Are your menu and art resources up
/// to date?"`.
const CREATION_FAILED_TEXT: u32 = 0x0107_4da8;
/// `"MM_ListMarkerTemplate"`.
const LIST_MARKER_TEMPLATE: u32 = 0x0107_4d8c;
/// `""`.
const EMPTY_TEXT: u32 = 0x0101_1584;
/// `"%s, %d:%02d"`.
const DATE_TIME_FORMAT: u32 = 0x0107_4fdc;
/// `"ENGLISH"`.
const ENGLISH_TEXT: u32 = 0x0101_8348;
/// `"_Title_1"` to `"_Title_5"`.
const TITLE_N_TEXT: [u32; 5] = [
    0x0107_3d30,
    0x0107_4fd0,
    0x0107_4fc4,
    0x0107_4fb8,
    0x0107_4fac,
];
/// `"MM_Headline_Title"`.
const HEADLINE_TITLE_TEXT: u32 = 0x0107_4f98;
/// `"UIPipBoyHolotapeStart"`.
const HOLOTAPE_START_SOUND: u32 = 0x0107_4e78;
/// `"UIPopUpMapMarkerAdded "`.
const MARKER_ADDED_SOUND: u32 = 0x0107_4e4c;
/// `"_ShowingAllNotes"`.
const SHOWING_ALL_NOTES_TEXT: u32 = 0x0107_4e64;
/// `"_MarkerIndex"`.
const MARKER_INDEX_TEXT: u32 = 0x0107_4e3c;
/// `"%s %s?"`.
const FAST_TRAVEL_FORMAT: u32 = 0x0107_4e34;
/// `"Companion"`.
const COMPANION_TEXT: u32 = 0x0107_4e28;
/// `"_LocationName"`.
const LOCATION_NAME_TEXT: u32 = 0x0107_4e18;
/// `"_current_value"` (`ListBox`).
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;

/// `6006.0` and `102.0` (`double`): the two `0xfa7` values `Create` checks.
const CREATE_VALUE_A: u32 = 0x0107_04f0;
const CREATE_VALUE_B: u32 = 0x0107_04e8;
/// `-1.0` (`float`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `2.0` (`double`).
const TWO_DOUBLE: u32 = 0x0101_1590;
/// `60.0` (`double`).
const SIXTY_DOUBLE: u32 = 0x0101_2638;
/// `850.0` and `500.0` (`double`): the map area `DoIdle` checks.
const MAP_WIDTH_DOUBLE: u32 = 0x0107_4f68;
const MAP_HEIGHT_DOUBLE: u32 = 0x0103_01a8;
/// `255.0` (`float`).
const FULL_ALPHA_FLOAT: u32 = 0x0102_3cd8;
/// `255.0` (`double`).
const FULL_ALPHA_DOUBLE: u32 = 0x0101_e568;
/// `1000.0` (`double`).
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
/// `128.0` (`float`).
const HALF_ALPHA_FLOAT: u32 = 0x0101_e704;
/// `0.9` (`float`): the note sound volume.
const NOTE_VOLUME_FLOAT: u32 = 0x0101_77e8;
/// `2.0` (`float`).
const TWO_FLOAT: u32 = 0x0101_62c0;
/// `0.5` (`float`).
const HALF_FLOAT: u32 = 0x0101_6248;
/// The two floats `DoClick` reads through `00403e20` for the tab burst.
const BURST_SETTING_A: u32 = 0x011d_b37c;
const BURST_SETTING_B: u32 = 0x011d_b458;

/// `Interface::IsTopMenuID` (engine map), cdecl (menu id): `AL`.
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `Interface::GetPipboy` (engine map), cdecl: an object.
const GET_PIPBOY: u32 = 0x0070_5990;
/// Thiscall on the pip-boy object: `AL`.
const PIPBOY_FLAG_00974D90: u32 = 0x0097_4d90;
/// Thiscall on the pip-boy object: `AL`.
const PIPBOY_FLAG_007FAEF0: u32 = 0x007f_aef0;
/// Cdecl, no arguments: the interface manager (`011d8a80`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
/// Cdecl, no arguments: `AL`.
const FLAG_004B71D0: u32 = 0x004b_71d0;
/// Thiscall: the word at +0x28.
const WORD_AT_28: u32 = 0x0045_cd60;
/// Thiscall: the word at +0x174.
const WORD_AT_174: u32 = 0x0070_4370;
/// Thiscall on a tile (trait id, integer), `RET 8`.
const TILE_SET_INT: u32 = 0x0070_0320;
/// Thiscall on a tile (trait id), `RET 4`: a float in `ST0`.
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// Thiscall on a tile (trait id, float, word), `RET 0xC`.
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// Thiscall on a tile (trait id, text, word), `RET 0xC`.
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `Tile::GetString` (engine map), thiscall (trait id), `RET 4`.
const TILE_GET_STRING: u32 = 0x00a0_11f0;
/// `Tile::IsTrue` (engine map), thiscall (trait id), `RET 4`: `AL`.
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::GetValue` (engine map), thiscall (trait id), `RET 4`.
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// `Tile::Value::CalculateValue` (engine map), thiscall (word), `RET 4`.
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// `Tile::TextToTrait` (engine map), cdecl (name).
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::AddUserTrait` (engine map), cdecl (name, word).
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `Tile::GetMenu` (engine map), thiscall.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetChildByName` (engine map), thiscall (name), `RET 4`.
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `Tile::GetChildByID` (engine map), thiscall (id), `RET 4`.
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
/// Cdecl (tile, name): a tile found by name below it.
const TILE_FIND_BY_NAME: u32 = 0x00a0_8b20;
/// `Tile::DeleteChildren` (engine map), thiscall.
const TILE_DELETE_CHILDREN: u32 = 0x00a0_4150;
/// `Tile::RemoveFadeControl` (engine map), thiscall (trait id), `RET 4`.
const TILE_REMOVE_FADE_CONTROL: u32 = 0x00a0_7dc0;
/// `Tile::GetMenuByClass` (engine map), cdecl (class).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::ReadFile` (engine map), thiscall (path), `RET 4`.
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMaximumDepth` (engine map), cdecl: a float in `ST0`.
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// Thiscall on a tile (`tile.cpp`): a float in `ST0` (with `00a01440`, the
/// two the map position code subtracts from x and y).
const TILE_GETTER_00A013D0: u32 = 0x00a0_13d0;
const TILE_GETTER_00A01440: u32 = 0x00a0_1440;
/// Thiscall on a tile (byte), `RET 4`.
const TILE_FLAG_SETTER_0071AA60: u32 = 0x0071_aa60;
/// Thiscall on a tile: `AL`.
const TILE_FLAG_GETTER_0071AA80: u32 = 0x0071_aa80;
/// `Tile::AddNeedsUpdate` (engine map), thiscall (flags), `RET 4`.
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// `Tile::Value::AddAction_ov2` (engine map), thiscall (action, tile, trait
/// id), `RET 0xC`.
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
/// `Menu::RemoveFromXMLArchive` (engine map), cdecl (path, menu).
const MENU_REMOVE_FROM_XML_ARCHIVE: u32 = 0x00a1_ca20;
/// `Menu::SetMenuTile` (engine map), thiscall (tile, word), `RET 8`.
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `Menu::PrepForVisibility` (engine map), thiscall (word), `RET 4`.
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The `Menu` constructor and destructor bodies (thiscall).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const MENU_DESTRUCT: u32 = 0x00a1_c520;
/// Thiscall: the word at +4 (on a list node: the next node; on the menu: the
/// tile the code reads children of).
const WORD_AT_4: u32 = 0x0072_6070;
/// Thiscall: returns `this` (on a list node: the address of its item).
const SELF: u32 = 0x0068_15c0;
/// Thiscall: the word at +4 of a non-null object, 0 for null.
const OBJECT_FIELD_00403DF0: u32 = 0x0040_3df0;
/// Thiscall: the address of the float the object holds (`effectitem.cpp`).
const OBJECT_VALUE_ADDRESS_00403E20: u32 = 0x0040_3e20;
/// Thiscall on a string (`BSStringT`): its characters.
const STRING_TEXT: u32 = 0x0055_9450;
/// String constructor and destructor (8-byte `BSStringT`), thiscall.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// Cdecl (string, format, ...): formats into the string.
const STRING_FORMAT: u32 = 0x0040_6f60;
/// Cdecl (buffer, size, format, ...): formats into a character buffer.
const BUFFER_FORMAT: u32 = 0x0040_6d00;
/// Cdecl (text, text): `EAX` (the language check of `UpdateMenu`).
const COMPARE_00408B20: u32 = 0x0040_8b20;
/// `__stricmp`, cdecl.
const STRICMP: u32 = 0x00ec_68e4;
/// The compiler's float to integer conversion (`_ftol2`): the value in
/// `ST0` (a leading `f64`), the integer in `EAX`.
const FTOL2: u32 = 0x00ec_62c0;
/// The allocator and its free, cdecl.
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// Cdecl (text): the warning print `CreationFailed` passes its message to.
const PRINT_WARNING: u32 = 0x005b_5e40;
/// Thiscall on the timer: the current time (`EAX`).
const TIMER_NOW: u32 = 0x0082_5c00;
/// `BSAudio::QInstance` (engine map), cdecl.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetRadioVolume` (engine map), thiscall: a float in `ST0`.
const AUDIO_GET_RADIO_VOLUME: u32 = 0x00ad_7ae0;
/// `BSAudio::GetSoundHandleByName` (engine map), thiscall (result, name,
/// flags), `RET 0xC`: the result.
const AUDIO_SOUND_BY_NAME: u32 = 0x00ad_7550;
/// `BSAudio::GetSoundHandleByNumericID` (engine map), thiscall (result, id,
/// flags), `RET 0xC`.
const AUDIO_SOUND_BY_ID: u32 = 0x00ad_73b0;
/// `BSAudio::GetSoundHandleByFilename` (engine map), thiscall (result, name,
/// flags, word), `RET 0x10`.
const AUDIO_SOUND_BY_FILENAME: u32 = 0x00ad_7480;
/// `BSAudio::EnterDialogue` (engine map), thiscall.
const AUDIO_ENTER_DIALOGUE: u32 = 0x00ad_85a0;
/// `BSSoundHandle::Play` (engine map), thiscall (word), `RET 4`.
const SOUND_PLAY: u32 = 0x00ad_8830;
/// `BSSoundHandle::Pause` (engine map), thiscall.
const SOUND_PAUSE: u32 = 0x00ad_88b0;
/// `BSSoundHandle::IsPlaying` (engine map), thiscall: `AL`.
const SOUND_IS_PLAYING: u32 = 0x00ad_8930;
/// `BSSoundHandle::SetVolume` (engine map), thiscall (float), `RET 4`.
const SOUND_SET_VOLUME: u32 = 0x00ad_89e0;
/// The sound handle constructor and destructor (12-byte handle), thiscall.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// Thiscall on a sound handle (handle), `RET 4`: the assignment.
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `BSSimpleList<BSSoundHandle>::AddTail` (engine map), thiscall (handle),
/// `RET 4`.
const SOUND_LIST_ADD_TAIL: u32 = 0x007a_19a0;
/// The `BSSimpleList<BSSoundHandle>` constructor and destructor bodies.
const SOUND_LIST_CONSTRUCT: u32 = 0x0076_b930;
const SOUND_LIST_DESTRUCT: u32 = 0x0076_b8a0;
/// List constructor (`BSSimpleList`), thiscall.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// List destructor, thiscall.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Thiscall on a list: removes its items.
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// The constructor and destructor of the array at +0x220 (`debugWaypoints`).
const WAYPOINTS_CONSTRUCT: u32 = 0x0071_9b80;
const WAYPOINTS_DESTRUCT: u32 = 0x0071_9be0;
/// Thiscall on the array at +0x220 (word), `RET 4`.
const WAYPOINTS_CLEAR: u32 = 0x0084_54f0;
/// `Calendar::GetHour` (`00867da0`) and `Calendar::GetDateString`
/// (`00867970`), thiscall.
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
const CALENDAR_GET_DATE_STRING: u32 = 0x0086_7970;
/// Thiscall on the calendar: marks the days passed for a rewrite.
const CALENDAR_MARK_STALE: u32 = 0x0086_7a20;
/// Thiscall on a tile: an object with a vtable (`tesobjectrefr.cpp`).
const TILE_OBJECT_0056C7F0: u32 = 0x0056_c7f0;
/// Thiscall on that object: a count, and (index) an item, `RET 4`.
const OBJECT_COUNT_0043B480: u32 = 0x0043_b480;
const OBJECT_ITEM_0043B4A0: u32 = 0x0043_b4a0;
/// Thiscall (word), `RET 4` (`tes.cpp`).
const OBJECT_NODE_0045BC00: u32 = 0x0045_bc00;
/// Cdecl, no arguments: 3 (the property type `GetProperty` gets).
const PROPERTY_TYPE_00438220: u32 = 0x0043_8220;
/// `NiAVObject::GetProperty` (engine map), thiscall (type), `RET 4`.
const AV_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
/// Thiscall on the property: `EAX`.
const PROPERTY_CHECK_009C8640: u32 = 0x009c_8640;
/// `TileShaderProperty::SetTileTexture` (engine map), thiscall (texture),
/// `RET 4`.
const SET_TILE_TEXTURE: u32 = 0x00bb_7a10;
/// Thiscall on the property (word), `RET 4` (`hudmainmenu.cpp`).
const PROPERTY_SET_007700D0: u32 = 0x0077_00d0;
/// `TESWorldSpace::GetParentWorldSimple` (engine map), thiscall.
const WORLD_GET_PARENT: u32 = 0x004f_d380;
/// Thiscall on a list box (word), `RET 4` (`statsmenu.cpp`): the item of a
/// tile.
const LIST_BOX_ITEM_OF_007E1110: u32 = 0x007e_1110;
/// Thiscall (buffer, size), `RET 8`: constructs `size` array elements.
const ARRAY_CONSTRUCT_ELEMENTS_007A6310: u32 = 0x007a_6310;
/// Thiscall (buffer, size), `RET 8`: destroys array elements.
const ARRAY_DESTROY_ELEMENTS_007A63B0: u32 = 0x007a_63b0;
/// Thiscall on the array: frees its buffer.
const ARRAY_FREE_006A8500: u32 = 0x006a_8500;
/// Thiscall on a `Conversation`: its destructor body.
const CONVERSATION_DESTRUCT: u32 = 0x0083_b8d0;
/// `ListBox<unsigned int>::Initialize` (engine map), thiscall (tile, template
/// name), `RET 8`.
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// `Interface::InitializeTabline` (engine map), cdecl (tile, count, ...).
const INITIALIZE_TABLINE: u32 = 0x0070_7be0;
/// `InterfaceManager::TutorialManager::ShowMessage` (engine map), thiscall
/// (message, menu id, flags), `RET 0xC`.
const TUTORIAL_SHOW_MESSAGE: u32 = 0x0071_8630;
/// Cdecl: `0x3ff` (the map menu's id).
const MAP_MENU_ID_00720DF0: u32 = 0x0072_0df0;
/// `MapMenu::UpdateCurrentPage` (engine map), thiscall.
const UPDATE_CURRENT_PAGE: u32 = 0x0079_af30;
/// `MapMenu::UpdatePlayingSounds` (engine map), thiscall.
const UPDATE_PLAYING_SOUNDS: u32 = 0x0079_a660;
/// `MapMenu::LoadNote` (engine map), thiscall (note), `RET 4`.
const LOAD_NOTE: u32 = 0x0079_93d0;
/// `MapMenu::StopNoteSound` (engine map), thiscall.
const STOP_NOTE_SOUND: u32 = 0x0079_8ad0;
/// `MapMenu::ToggleNotesChallengeMode` (engine map), thiscall (word), `RET 4`.
const TOGGLE_NOTES_CHALLENGE_MODE: u32 = 0x0079_b460;
/// `MapMenu::MapToWorldCoord` (engine map), thiscall (result, x, y, z, world
/// space), `RET 0x14`.
const MAP_TO_WORLD_COORD: u32 = 0x0079_c450;
/// `MapMenu::WorldToMapCoord` (engine map), thiscall (result, x, y, z, world
/// space), `RET 0x14`.
const WORLD_TO_MAP_COORD: u32 = 0x0079_c380;
/// `MapMenu::CalculatePlayerMarkerSpace` (engine map), thiscall (x, y, z),
/// `RET 0xC`.
const CALCULATE_PLAYER_MARKER_SPACE: u32 = 0x0079_84d0;
/// `MapMenu::GetWorldMapTrackingRef` (engine map), thiscall (reference,
/// word), `RET 8`.
const GET_WORLD_MAP_TRACKING_REF: u32 = 0x0079_f7e0;
/// Thiscall on the menu: shows the map position (`DoIdle`).
const SHOW_POSITION_00799DC0: u32 = 0x0079_9dc0;
/// Thiscall on the hacking-menu object: a float in `ST0` (x), and on the map
/// menu object (`007983c0`): y.
const CURSOR_X_007697F0: u32 = 0x0076_97f0;
const CURSOR_Y_007983C0: u32 = 0x0079_83c0;
/// Cdecl (word) (`mapmenu.cpp`): the `DoIdle` step it runs 25 times.
const MAP_STEP_0079FFB0: u32 = 0x0079_ffb0;
/// Cdecl: `AL` (`mapmenu.cpp`).
const FLAG_00798440: u32 = 0x0079_8440;
/// Thiscall on the menu (`mapmenu.cpp`).
const PAGE_0079C0C0: u32 = 0x0079_c0c0;
/// Thiscall on a note: `AL` (`mapmenu.cpp`).
const NOTE_FLAG_00798420: u32 = 0x0079_8420;
/// Thiscall on the player: `EAX` (`mapmenu.cpp`).
const PLAYER_FLAG_00798400: u32 = 0x0079_8400;
/// Cdecl (word) (`mapmenu.cpp`).
const TAB_KNOB_0079C340: u32 = 0x0079_c340;
/// Thiscall on `011dea0c` (`main.cpp`): an object.
const OBJECT_00877720: u32 = 0x0087_7720;
/// Thiscall on that object (word, word), `RET 8`: `EAX`.
const OBJECT_CHECK_00A23A50: u32 = 0x00a2_3a50;
const OBJECT_CHECK_00A24660: u32 = 0x00a2_4660;
/// Thiscall: the byte `DoIdle` reads (`011da3b8`).
const BYTE_OF_00408D60: u32 = 0x0040_8d60;
/// The object `00408d60` is called on.
const OBJECT_011DA3B8: u32 = 0x011d_a3b8;
/// `ListBox<StartMenu::Option_P>::GetTileFor` (engine map; folded code),
/// thiscall (address of the item), `RET 4`.
const LIST_BOX_GET_TILE_FOR: u32 = 0x007a_22d0;
/// Thiscall on a note (word), `RET 4` (`bgsnote.cpp`).
const NOTE_SET_READ_005E9300: u32 = 0x005e_9300;
/// Thiscall on a note: its type (`bgsnote.cpp`).
const NOTE_TYPE: u32 = 0x005e_8d40;
/// Thiscall on a note: its sound (`bgsnote.cpp`).
const NOTE_SOUND_005E9100: u32 = 0x005e_9100;
/// Thiscall on a note: its topic (`bgsnote.cpp`).
const NOTE_TOPIC_005E9240: u32 = 0x005e_9240;
/// `BGSNote::GetNoteSpeaker` (engine map), thiscall.
const NOTE_GET_SPEAKER: u32 = 0x005e_92a0;
/// Thiscall: the word at +0xC.
const WORD_AT_C: u32 = 0x0084_e3a0;
/// Thiscall: `this + 0x10`.
const PLUS_10: u32 = 0x0046_0140;
/// `Character::Character_ov2` (engine map), thiscall (word), `RET 4`.
const CHARACTER_CONSTRUCT: u32 = 0x008d_1f40;
/// `TESForm::SetTemporary` (engine map), thiscall.
const FORM_SET_TEMPORARY: u32 = 0x0048_4490;
/// `TESObjectREFR::SetObjectReference` (engine map), thiscall (form),
/// `RET 4`.
const REFR_SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
/// `Conversation::Conversation_ov2` (engine map), thiscall (speaker, player,
/// topic), `RET 0xC`.
const CONVERSATION_CONSTRUCT: u32 = 0x0083_b850;
/// `Conversation::FirstItem` (engine map), thiscall.
const CONVERSATION_FIRST_ITEM: u32 = 0x0083_b9a0;
/// `DialogueItem::GetCurrentResponse` (engine map), thiscall.
const GET_CURRENT_RESPONSE: u32 = 0x0083_c820;
/// `DialogueItem::FirstResponse` / `NextResponse` (engine map), thiscall:
/// `AL`.
const FIRST_RESPONSE: u32 = 0x0083_c7b0;
const NEXT_RESPONSE: u32 = 0x0083_c7e0;
/// Thiscall on the subtitle array at +0xA8 (address of the text), `RET 4`.
const SUBTITLES_ADD_007A1AC0: u32 = 0x007a_1ac0;
/// `TESTopicInfo::RunResult` (engine map), thiscall (word, speaker), `RET 8`.
const TOPIC_INFO_RUN_RESULT: u32 = 0x0061_f170;
/// Cdecl (word) (`dialogmenu.cpp`).
const DIALOG_FLAG_00762150: u32 = 0x0076_2150;
/// `PlayerCharacter::SetActiveQuest` (engine map), thiscall (quest), `RET 4`.
const SET_ACTIVE_QUEST: u32 = 0x0095_29d0;
/// Thiscall on a quest: `AL` (`tesconditionfunctions.cpp`).
const QUEST_FLAG_0059E400: u32 = 0x0059_e400;
/// `Interface::PlayMenuSound` (engine map), cdecl (sound).
const PLAY_MENU_SOUND: u32 = 0x0070_6f30;
/// Thiscall on the radio list box (callback, word, word), `RET 0xC`
/// (`bartermenu.cpp`).
const LIST_BOX_FILTER_007314C0: u32 = 0x0073_14c0;
/// The radio list filter callback the code passes (`0079be80`).
const RADIO_FILTER_CALLBACK: u32 = 0x0079_be80;
/// `FalloutRadio::PipboyRadioEnable` (engine map), cdecl (word).
const RADIO_ENABLE: u32 = 0x0083_24e0;
/// Cdecl (station, word) (`falloutradio.cpp`).
const RADIO_SET_STATION_00832240: u32 = 0x0083_2240;
/// `ListBox<BGSNote_P>::Filter` (`007a2e30`) and
/// `ListBox<TESChallenge_P>::Filter` (`007a5730`) (engine map), thiscall
/// (callback), `RET 4`, with the callbacks the code passes.
const NOTE_LIST_FILTER: u32 = 0x007a_2e30;
const NOTE_FILTER_CALLBACK: u32 = 0x0079_b390;
const CHALLENGE_LIST_FILTER: u32 = 0x007a_5730;
const CHALLENGE_FILTER_CALLBACK: u32 = 0x0079_b720;
/// Thiscall on a 12-byte point (x, y, z), `RET 0xC`: the point.
const POINT_SET_00416870: u32 = 0x0041_6870;
/// Thiscall on a point (result, float), `RET 8`: the result (`tesland.cpp`).
const POINT_SCALE_0053D280: u32 = 0x0053_d280;
/// Thiscall on a form: its type byte (+4).
const FORM_TYPE: u32 = 0x0040_1170;
/// `TESWorldSpace::AdjustMapMarkerCoord` (engine map), thiscall (point,
/// word), `RET 8`.
const ADJUST_MAP_MARKER_COORD: u32 = 0x0058_8e40;
/// `TESObjectCELL::AdjustCoordForNorthRotation` (engine map), thiscall
/// (point, point, word), `RET 0xC`.
const ADJUST_FOR_NORTH_ROTATION: u32 = 0x0055_5b10;
/// Cdecl (text, word, word, callback, word, word, float, float, ...): the
/// message box the map shows (`interface.cpp`).
const SHOW_MESSAGE_BOX: u32 = 0x0070_3e80;
/// The callbacks the message boxes get (`SetPlayerMarkerCallback` and the
/// fast travel one).
const SET_PLAYER_MARKER_CALLBACK: u32 = 0x0079_8840;
const FAST_TRAVEL_CALLBACK: u32 = 0x0079_8710;
/// `TESQuestTarget::GetReference` (engine map), thiscall (word), `RET 4`.
const QUEST_TARGET_GET_REFERENCE: u32 = 0x0061_01b0;
/// `TESObjectREFR::GetWorldSpace` (engine map), thiscall.
const REFR_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// Thiscall on the player: `AL` (`playercharacter.cpp`).
const PLAYER_FLAG_0093D660: u32 = 0x0093_d660;
/// `TESObjectREFR::GetMapMarkerData` (engine map), thiscall.
const REFR_GET_MAP_MARKER_DATA: u32 = 0x0056_9060;
/// `MapMarkerData::GetTravelLoc` (engine map), thiscall: `AL`.
const MARKER_GET_TRAVEL_LOC: u32 = 0x0043_8ef0;
/// Thiscall on map marker data: its name (`mapmarkerdata.cpp`).
const MARKER_NAME_00438EB0: u32 = 0x0043_8eb0;
/// Cdecl (text, word, word, word, float, word) (`interface.cpp`).
const SHOW_MESSAGE_007052F0: u32 = 0x0070_52f0;
/// `FORenderedMenu::TriggerBurst` (engine map), thiscall (float, float,
/// float), `RET 0xC`.
const TRIGGER_BURST: u32 = 0x007f_b150;
/// Cdecl: `AL` (`interface.cpp`).
const FLAG_00709D40: u32 = 0x0070_9d40;
/// The `ListBox` base helpers (`menumanager.rs`): the `ListBoxBase`
/// constructor, the registration with the menu manager and its removal.
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
const LIST_BOX_REGISTER: u32 = 0x0072_1130;
const LIST_BOX_UNREGISTER: u32 = 0x0072_11c0;
/// Thiscall on a list box (flag bit), `RET 4`: `AL` (`menumanager.rs`).
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
/// Thiscall on a list box (tile), `RET 4`: `AL`, whether the tile is one of
/// its items (`menumanager.rs`).
const LIST_BOX_BELONGS_TO: u32 = 0x0072_42f0;
/// Thiscall on a list node (word), `RET 4`: `RemoveAll` passes 0 before
/// deleting the node.
const NODE_SET_NEXT: u32 = 0x006e_cd40;
/// Thiscall on a list node (flags), `RET 4`: its deleting destructor.
const NODE_DELETE: u32 = 0x0047_02f0;
/// Thiscall on an item's data (flags), `RET 4`.
const ITEM_DATA_DELETE: u32 = 0x0044_59e0;
/// The byte that, when set, keeps the item tiles when a list box empties.
const KEEP_ITEM_TILES: u32 = 0x011d_8909;
/// Cdecl (float): a float in `ST0` (`teswater.cpp`).
const ROUND_FLOAT: u32 = 0x004e_4430;
/// Cdecl (integer, integer): `EAX`.
const INT_PAIR_00647B70: u32 = 0x0064_7b70;
/// Thiscall on the interface manager (`tesworldspace.cpp`): the object
/// `Tile::ReadFile` runs on.
const READER_00586150: u32 = 0x0058_6150;
/// Thiscall on the player (address of a string), `RET 4`
/// (`tesobjectrefr.cpp`): `UpdateMenu` hands it the date text.
const PLAYER_MESSAGE_00578870: u32 = 0x0057_8870;

/// `"_enabled"`, `"_highlight_y"`, `"_selected_height"`, `"lb_highlight_box"`,
/// `"top"`, `"bottom"`, `"left"`, `"right"`, `"_num_filtered"`,
/// `"_scroll_delta"`, `"_number_of_items"` (the `ListBox` methods).
const ENABLED_TEXT: u32 = 0x0107_0244;
const HIGHLIGHT_Y_TEXT: u32 = 0x0106_f8c0;
const SELECTED_HEIGHT_TEXT: u32 = 0x0106_f8ac;
const HIGHLIGHT_BOX_TEXT: u32 = 0x0106_f898;
const TOP_TEXT: u32 = 0x0106_f894;
const BOTTOM_TEXT: u32 = 0x0106_f88c;
const LEFT_TEXT: u32 = 0x0106_f884;
const RIGHT_TEXT: u32 = 0x0106_f87c;
const NUM_FILTERED_TEXT: u32 = 0x0106_f8d0;
const SCROLL_DELTA_TEXT: u32 = 0x0106_f90c;
const NUMBER_OF_ITEMS_TEXT: u32 = 0x0106_f8f8;

/// The tile pointer `index` of the menu at `menu` (`pTiles`, +0x28).
fn map_tile(e: &Engine, menu: u32, index: u32) -> u32 {
    e.mem.u32(menu + 0x28 + 4 * index)
}

/// `MapMenu::pMe`.
fn map_menu(e: &Engine) -> u32 {
    e.mem.u32(MAP_MENU_SINGLETON)
}

/// The `float` argument a `PUSH ECX; FLD float ptr [..]; FSTP float ptr
/// [ESP]` pair passes: the constant's bits, a signalling NaN quieted.
fn float_constant(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// `FCOMP; FNSTSW AX; TEST AH,0x41; JNZ`: on only when `a` is strictly above
/// `b` (not less, equal or unordered).
fn above(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater)
}

/// `FCOMP; FNSTSW AX; TEST AH,5; JNP` (jump taken): `a` below `b`, ordered.
fn below(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Less)
}

/// `TEST AH,0x44; JP` not taken: `a` equal to `b`, ordered.
fn equal(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Equal)
}

/// The tile object chain `ClearMapMemory`, `ClearWorldMapTexture` and
/// `Create` walk from a tile to its tile shader property: `0056c7f0` on the
/// tile, `0045bc00(.., 0)` on that, then `NiAVObject::GetProperty(3)`; 0
/// where a step gives 0.
fn tile_shader_property(e: &mut Engine, tile: u32) -> u32 {
    let node = if e.call(TILE_OBJECT_0056C7F0, &args![tile]).u32() == 0 {
        0
    } else {
        let object = e.call(TILE_OBJECT_0056C7F0, &args![tile]).u32();
        e.call(OBJECT_NODE_0045BC00, &args![object, 0u32]).u32()
    };
    if node == 0 {
        return 0;
    }
    let kind = e.call(PROPERTY_TYPE_00438220, &args![]).u32();
    e.call(AV_OBJECT_GET_PROPERTY, &args![node, kind]).u32()
}

/// A sound by name played once: `BSAudio::GetSoundHandleByName(name, 0x121)`
/// into a stack handle, `Play(0)`, then the handle destroyed.
fn play_named_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let sound = e
            .call(AUDIO_SOUND_BY_NAME, &args![audio, handle, name, 0x121u32])
            .u32();
        e.call(SOUND_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

/// `a011b0(tile, trait)` as the `float` an `FSTP float` stores.
fn tile_float_bits(e: &mut Engine, tile: u32, id: u32) -> u32 {
    x87_store32(e.call(TILE_GET_FLOAT, &args![tile, id]).f64())
}

// Translated from 0072d040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::AttachTileByID` (Xbox PDB): stores `tile` as `pTiles[id]` for
/// an id in 0..0x16.
pub fn map_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    if (0..0x16).contains(&id) {
        e.mem.set_u32(this.addr() + 0x28 + 4 * id as u32, tile);
    }
}

// Translated from 007312e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::SaveListState` (engine map): saves the highlighted
/// item's index (trait `0xfac`, -1.0 without one) and the scroll bar's
/// `"_current_value"` (-1.0 without a scroll bar).
pub fn list_box_bgs_note_save_list_state(e: &mut Engine, this: Ptr<ListBox>) {
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    let index = if highlighted.is_null() {
        float_constant(e, MINUS_ONE_FLOAT)
    } else {
        tile_float_bits(e, highlighted.addr(), 0xfac)
    };
    e.mem.set_u32(this.addr() + 0x24, x87_quiet_bits(index));
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    let value = if scroll_bar.is_null() {
        float_constant(e, MINUS_ONE_FLOAT)
    } else {
        let current_value = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
        let scroll_bar = e.get(this, ListBox::pScrollBar);
        tile_float_bits(e, scroll_bar.addr(), current_value)
    };
    e.mem.set_u32(this.addr() + 0x28, x87_quiet_bits(value));
}

// Translated from 00796740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00796740 - the C++ exception frame (FS:[0] chain, unwind states 0 to 10 around the members' construction) is not translated.
/// `MapMenu::MapMenu` (Xbox PDB): the `Menu` base, the vtable, the members
/// (sound list, subtitle array, the two marker lists, the four points, the
/// five list boxes and the waypoint array), the eight trait ids of the
/// statics (`Tile::TextToTrait`), then the fields' start values: page 0x24,
/// showing all notes, fog of war on, `PlayerMarkerPos` from `011f426c` and
/// the radio volume saved from `BSAudio::GetRadioVolume`.
pub fn map_menu_map_menu(e: &mut Engine, this: Ptr<MapMenu>) -> Ptr<MapMenu> {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, MAP_MENU_VTABLE);
    e.call(SOUND_LIST_CONSTRUCT, &args![base + 0x98]);
    fn_007a1a70(e, Ptr::new(base + 0xa8));
    e.call(LIST_CONSTRUCT, &args![base + 0xd8]);
    e.call(LIST_CONSTRUCT, &args![base + 0xe0]);
    for offset in [0xe8u32, 0xf0, 0xfc, 0x120] {
        e.call(SELF, &args![base + offset]);
    }
    fn_007a1c90(e, Ptr::new(base + 0x130));
    fn_007a27f0(e, Ptr::new(base + 0x160));
    fn_007a36e0(e, Ptr::new(base + 0x190));
    fn_007a43a0(e, Ptr::new(base + 0x1c0));
    fn_007a50f0(e, Ptr::new(base + 0x1f0));
    e.call(WAYPOINTS_CONSTRUCT, &args![base + 0x220]);
    for (cache, name) in [
        (TRAIT_TITLE, TITLE_TEXT),
        (TRAIT_VALUE, VALUE_TEXT),
        (TRAIT_CURRENT_TAB, CURRENT_TAB_TEXT),
        (TRAIT_SELECTED, SELECTED_TEXT),
        (TRAIT_TEXT_ALPHA, TEXT_ALPHA_TEXT),
        (TRAIT_X, X_COORD_TEXT),
        (TRAIT_Y, Y_COORD_TEXT),
        (TRAIT_MAGNIFICATION, MAGNIFICATION_TEXT),
    ] {
        let id = e.call(TEXT_TO_TRAIT, &args![name]).u32();
        e.mem.set_u32(cache, id);
    }
    e.set(this, MapMenu::fWaveformAnimationStartTime, 0.0);
    e.set(this, MapMenu::bShowingAllNotes, 1);
    e.set(this, MapMenu::pCurrentSound, 0);
    e.set(this, MapMenu::bSoundsLoading, 0);
    e.set(this, MapMenu::pFastTravelMarker, 0);
    e.set(this, MapMenu::iCurrentPage, 0x24);
    for word in 0..3 {
        let value = e.mem.u32(POINT_011F426C + 4 * word);
        e.mem.set_u32(base + 0xfc + 4 * word, value);
    }
    e.set(this, MapMenu::PlayerMarkerSpace, 0);
    e.set(this, MapMenu::pMapSpace, 0);
    e.set(this, MapMenu::pMapTile, 0);
    e.set(this, MapMenu::pDisplayedQuest, 0);
    e.set(this, MapMenu::pSelectedNote, 0);
    e.set(this, MapMenu::pAdjustNorthRotationCell, 0);
    e.set(this, MapMenu::pHighlightedMapMarker, 0);
    e.set(this, MapMenu::uiNoteEnterStartTime, 0);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let volume = e.call(AUDIO_GET_RADIO_VOLUME, &args![audio]).f64();
    e.mem.set_u32(base + 0xc8, x87_store32(volume));
    e.set(this, MapMenu::pDisplayedQuestTarget, 0);
    e.set(this, MapMenu::uiNextLocalMapGrid, 0);
    e.set(this, MapMenu::pMapPlayerRef, 0);
    e.set(this, MapMenu::bFogOfWarEnabled, 1);
    e.set(this, MapMenu::pRootMapSpace, 0);
    e.set(this, MapMenu::bRootSpaceChanged, 0);
    this
}

// Translated from 00796a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::GetClass` (Xbox PDB): `0x3ff`.
pub fn map_menu_get_class(e: &mut Engine, this: Ptr<MapMenu>) -> u32 {
    let _ = (e, this);
    0x3ff
}

// Translated from 00796a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu`'s scalar deleting destructor (Xbox PDB `__vecDelDtor` slot):
/// the destructor body `00796a80`, then the object freed when bit 0 of
/// `flags` is set. Returns `this`.
pub fn map_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<MapMenu>,
    flags: u32,
) -> Ptr<MapMenu> {
    fn_00796a80(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00796a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00796a80 - the C++ exception frame (FS:[0] chain, unwind states 9 down to 0 around the members' destruction) is not translated.
/// The `MapMenu` destructor body: the vtable, `MapMenu::pMe` cleared, then
/// the members destroyed in reverse order (waypoint array, the five list
/// boxes, the two marker lists, the subtitle array, the sound list) and the
/// `Menu` base.
pub fn fn_00796a80(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    e.mem.set_u32(base, MAP_MENU_VTABLE);
    e.mem.set_u32(MAP_MENU_SINGLETON, 0);
    e.call(WAYPOINTS_DESTRUCT, &args![base + 0x220]);
    fn_007a51c0(e, Ptr::new(base + 0x1f0));
    fn_007a4470(e, Ptr::new(base + 0x1c0));
    fn_007a37b0(e, Ptr::new(base + 0x190));
    fn_007a28c0(e, Ptr::new(base + 0x160));
    fn_007a1d60(e, Ptr::new(base + 0x130));
    e.call(LIST_DESTRUCT, &args![base + 0xe0]);
    e.call(LIST_DESTRUCT, &args![base + 0xd8]);
    fn_007a1aa0(e, Ptr::new(base + 0xa8));
    e.call(SOUND_LIST_DESTRUCT, &args![base + 0x98]);
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 00796b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::Create` (Xbox PDB): deletes an open map menu (`GetMenuByClass`
/// 0x3ff, its deleting destructor with 1), reads `map_menu.xml`, takes its
/// menu as `MapMenu::pMe` and removes the file from the XML archive. When
/// that is not a map menu (class 0x3ff) the menu is deleted again (when its
/// tile, `00726070`, is set) and 0 returned. Otherwise: the calendar's days
/// are marked (`00867a20`), the tile is set (`SetMenuTile(tile, 0)`), and
/// unless a tile is missing (`00796f90`, with a warning and 0 returned) the
/// tile's depth trait `0xfad` is set to the maximum depth when trait `0xfa7`
/// is 6006 or 102, the tutorial message 0x1a is shown, the tab line and the
/// five list boxes are set up, the map's property is updated (`007700d0`),
/// the title emptied, tiles 5 and 3 get flag 0x8000 (`00796f40`), the map
/// memory is cleared and the menu prepared for visibility. Returns the tile.
pub fn map_menu_create(e: &mut Engine) -> u32 {
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![0x3ffu32]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(READER_00586150, &args![interface]).u32();
    let tile = e.call(TILE_READ_FILE, &args![reader, MAP_MENU_XML]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.mem.set_u32(MAP_MENU_SINGLETON, menu);
    let menu = map_menu(e);
    e.call(MENU_REMOVE_FROM_XML_ARCHIVE, &args![MAP_MENU_XML, menu]);
    let menu = map_menu(e);
    if menu == 0 || e.vcall(menu, 0x34, &args![]).u32() != 0x3ff {
        let menu = map_menu(e);
        if menu != 0 && e.call(WORD_AT_4, &args![menu]).u32() != 0 {
            let menu = map_menu(e);
            if menu != 0 {
                e.vcall(menu, 0, &args![1u32]);
            }
        }
        return 0;
    }
    let menu = map_menu(e);
    e.call(CALENDAR_MARK_STALE, &args![menu]);
    let menu = map_menu(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, tile, 0u32]);
    // The code stores `pMe` back into itself here.
    let menu = map_menu(e);
    e.mem.set_u32(MAP_MENU_SINGLETON, menu);
    if !fn_00796f90(e, Ptr::new(map_menu(e))) {
        e.call(PRINT_WARNING, &args![CREATION_FAILED_TEXT]);
        return 0;
    }
    let first: f64 = e.global(CREATE_VALUE_A);
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64();
    let matches = equal(value, first) || {
        let second: f64 = e.global(CREATE_VALUE_B);
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64();
        equal(value, second)
    };
    if matches {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfadu32, x87_store32(depth), 1u32],
        );
    }
    let menu_id = e.call(MAP_MENU_ID_00720DF0, &args![]).u32();
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    e.call(
        TUTORIAL_SHOW_MESSAGE,
        &args![interface.wrapping_add(0x4d4), 0x1au32, menu_id, 0x200u32],
    );
    let mut names = [0u32; 5];
    for (i, entry) in TABLINE_ENTRIES.iter().enumerate() {
        names[i] = e.call(OBJECT_FIELD_00403DF0, &args![*entry]).u32();
    }
    let menu = map_menu(e);
    let tabline = map_tile(e, menu, 17);
    e.call(
        INITIALIZE_TABLINE,
        &args![tabline, 0x20u32, names[4], names[3], names[2], names[1], names[0], 0u32],
    );
    for (list, index, flag) in [
        (0x130u32, 7u32, true),
        (0x1c0, 15, true),
        (0x160, 8, true),
        (0x1f0, 19, false),
        (0x190, 10, false),
    ] {
        let menu = map_menu(e);
        let list_tile = map_tile(e, menu, index);
        e.call(
            LIST_BOX_INITIALIZE,
            &args![menu + list, list_tile, LIST_MARKER_TEMPLATE],
        );
        if flag {
            let menu = map_menu(e);
            fn_007a1930(e, Ptr::new(menu + list), 0);
        }
    }
    let menu = map_menu(e);
    let map = map_tile(e, menu, 16);
    let property = tile_shader_property(e, map);
    if property != 0 {
        e.call(PROPERTY_SET_007700D0, &args![property, 1u32]);
    }
    let title = e
        .call(ADD_USER_TRAIT, &args![TITLE_TEXT, 0xffff_ffffu32])
        .u32();
    let menu = map_menu(e);
    let title_tile = map_tile(e, menu, 6);
    e.call(TILE_SET_STRING, &args![title_tile, title, EMPTY_TEXT, 1u32]);
    let menu = map_menu(e);
    let tile_5 = map_tile(e, menu, 5);
    fn_00796f40(e, Ptr::new(tile_5), 1);
    let menu = map_menu(e);
    let tile_3 = map_tile(e, menu, 3);
    fn_00796f40(e, Ptr::new(tile_3), 1);
    map_menu_clear_map_memory(e);
    let menu = map_menu(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 1u32]);
    tile
}

// Translated from 00796f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets (`flag` non-zero) or clears bit 0x8000 of
/// the word at +0x30 (the code calls it on tiles).
pub fn fn_00796f40(e: &mut Engine, this: Ptr, flag: u8) {
    let word = e.mem.u32(this.addr() + 0x30);
    if flag != 0 {
        e.mem.set_u32(this.addr() + 0x30, word | 0x8000);
    } else {
        e.mem.set_u32(this.addr() + 0x30, word & 0xffff_7fff);
    }
}

// Translated from 00796f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether all 22 `pTiles` are set.
pub fn fn_00796f90(e: &mut Engine, this: Ptr<MapMenu>) -> bool {
    (0..0x16).all(|index| map_tile(e, this.addr(), index) != 0)
}

// Translated from 00796fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00796fd0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 6 around the sound handles, the speaker and the conversation) and the /GS security cookie check (00ec408c) are not translated.
/// `MapMenu::DoClick` (Xbox PDB): with no tile, a click on id 0..0x16 uses
/// `pTiles[id]`. Then by id: 2 and 4 click the highlighted map marker (id
/// 0x1a through the vtable); 0xc acts on the current page (sets the player
/// marker on the world or local map, centres the map on the displayed quest
/// target, or switches the notes filter or the challenge type); 0x12 toggles
/// the notes/challenge mode on the notes page; 0x17 makes the highlighted
/// quest active; 0x18 opens the clicked note (and plays its sound or its
/// conversation); 0x19 switches the highlighted radio station; 0x1a asks
/// for a fast travel to the clicked marker (or shows a companion message);
/// 0x20 to 0x24 switch the page. See the body for the exact order.
pub fn map_menu_do_click(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    let base = this.addr();
    let mut tile = tile;
    if tile == 0 && (0..0x16).contains(&id) {
        tile = map_tile(e, base, id as u32);
    }
    // The compiler's switch: a byte table (`00798388`) maps `id - 2` (0 to
    // 0x22, compared unsigned) to the cases of `00798364`.
    match id.wrapping_sub(2) as u32 {
        0 | 2 => {
            let marker = e.get(this, MapMenu::pHighlightedMapMarker);
            if marker != 0 {
                let marker = e.get(this, MapMenu::pHighlightedMapMarker);
                e.vcall(base, 0xc, &args![0x1au32, marker]);
            }
        }
        0xa => do_click_current_page(e, this),
        0x10 => {
            if e.get(this, MapMenu::iCurrentPage) == 0x23 {
                e.call(TOGGLE_NOTES_CHALLENGE_MODE, &args![this, 1u32]);
            }
        }
        0x15 => {
            let quests = Ptr::new(base + 0x130);
            if fn_007a1910(e, quests) != 0 {
                let quest = fn_007a1910(e, quests);
                if !e.call(QUEST_FLAG_0059E400, &args![quest]).bool() {
                    let quest = fn_007a1910(e, quests);
                    let player = e.mem.u32(PLAYER_011DEA3C);
                    e.call(SET_ACTIVE_QUEST, &args![player, quest]);
                    e.call(PLAY_MENU_SOUND, &args![3u32]);
                    e.call(UPDATE_CURRENT_PAGE, &args![this]);
                }
            }
        }
        0x16 => {
            if e.mem.u8(CHALLENGE_MODE_011DA36D) == 0 {
                do_click_note(e, this, tile);
            }
        }
        0x17 => {
            let radio = base + 0x190;
            let highlighted = e.vcall(radio, 4, &args![]).u32();
            let alpha_trait = e.mem.u32(TRAIT_TEXT_ALPHA);
            let alpha = e
                .call(TILE_GET_FLOAT, &args![highlighted, alpha_trait])
                .f64();
            let full: f64 = e.global(FULL_ALPHA_DOUBLE);
            if equal(alpha, full) {
                let selected_trait = e.mem.u32(TRAIT_SELECTED);
                let selected = e
                    .call(TILE_IS_TRUE, &args![highlighted, selected_trait])
                    .bool();
                let turn_on = !selected;
                fn_0079c090(e, this);
                e.call(
                    LIST_BOX_FILTER_007314C0,
                    &args![radio, RADIO_FILTER_CALLBACK, 0u32, 0x7fff_ffffu32],
                );
                if e.call(FLAG_00798440, &args![]).bool() {
                    e.call(RADIO_ENABLE, &args![0u32]);
                }
                if turn_on {
                    e.call(RADIO_ENABLE, &args![1u32]);
                    let station = fn_007a1910(e, Ptr::new(radio));
                    e.call(RADIO_SET_STATION_00832240, &args![station, 0u32]);
                    let selected_trait = e.mem.u32(TRAIT_SELECTED);
                    e.call(TILE_SET_INT, &args![highlighted, selected_trait, 1u32]);
                }
                e.call(UPDATE_CURRENT_PAGE, &args![this]);
            }
        }
        0x18 => do_click_marker(e, this, id, tile),
        0x1e..=0x22 => {
            let tabline = map_tile(e, base, 17);
            let tab_trait = e.mem.u32(TRAIT_CURRENT_TAB);
            e.call(
                TILE_SET_INT,
                &args![tabline, tab_trait, id.wrapping_sub(0x20)],
            );
            e.set(this, MapMenu::iCurrentPage, id);
            let third = float_constant(e, HALF_FLOAT);
            let second_ptr = e
                .call(OBJECT_VALUE_ADDRESS_00403E20, &args![BURST_SETTING_A])
                .u32();
            let second = x87_quiet_bits(e.mem.u32(second_ptr));
            let first_ptr = e
                .call(OBJECT_VALUE_ADDRESS_00403E20, &args![BURST_SETTING_B])
                .u32();
            let first = x87_quiet_bits(e.mem.u32(first_ptr));
            let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
            let rendered = e.call(WORD_AT_174, &args![interface]).u32();
            e.call(TRIGGER_BURST, &args![rendered, first, second, third]);
            if !e.call(FLAG_00709D40, &args![]).bool() && id != 0x20 && id != 0x21 {
                let alpha = float_constant(e, FULL_ALPHA_FLOAT);
                let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
                let pipboy_tile = e.call(WORD_AT_28, &args![interface]).u32();
                e.call(TILE_SET_FLOAT, &args![pipboy_tile, 0xfa9u32, alpha, 1u32]);
            }
            let tile_13 = map_tile(e, base, 13);
            e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 0u32]);
            let tile_12 = map_tile(e, base, 12);
            e.call(TILE_SET_INT, &args![tile_12, 0xfafu32, 1u32]);
            map_menu_update_menu(e);
            let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
            let rendered = e.call(WORD_AT_174, &args![interface]).u32();
            if e.call(PIPBOY_FLAG_00974D90, &args![rendered]).bool() {
                e.call(TAB_KNOB_0079C340, &args![1u32]);
            }
        }
        _ => {}
    }
}

/// `DoClick` id 0xC: by `iCurrentPage` (a switch on `page - 0x20`, 0 to 3).
fn do_click_current_page(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    match e.get(this, MapMenu::iCurrentPage).wrapping_sub(0x20) as u32 {
        0 | 1 => {
            if e.get(this, MapMenu::pMapTile) == 0 {
                return;
            }
            set_player_marker(e, this);
        }
        2 => {
            if e.get(this, MapMenu::pDisplayedQuestTarget) == 0 {
                return;
            }
            e.vcall(base, 0xc, &args![0x21u32, 0u32]);
            let target = e.get(this, MapMenu::pDisplayedQuestTarget);
            let quest_part = e.call(PLUS_10, &args![target]).u32();
            let target = e.get(this, MapMenu::pDisplayedQuestTarget);
            let reference = e
                .call(QUEST_TARGET_GET_REFERENCE, &args![target, 1u32])
                .u32();
            let tracked = e
                .call(
                    GET_WORLD_MAP_TRACKING_REF,
                    &args![this, reference, quest_part],
                )
                .u32();
            if tracked == 0 || e.get(this, MapMenu::pMapTile) == 0 {
                return;
            }
            let world = e.call(REFR_GET_WORLD_SPACE, &args![tracked]).u32();
            let position = e.vcall(tracked, 0x1f4, &args![]).u32();
            let x = e.mem.u32(position);
            let y = e.mem.u32(position + 4);
            let z = e.mem.u32(position + 8);
            e.with_stack(8, |e, map_point| {
                e.call(WORLD_TO_MAP_COORD, &args![this, map_point, x, y, z, world]);
                for (size_trait, position_trait, offset) in
                    [(0xfb1u32, 0xfa1u32, 0u32), (0xfb0, 0xfa2, 4)]
                {
                    let map = e.get(this, MapMenu::pMapTile);
                    let owner = e.call(WORD_AT_28, &args![map]).u32();
                    let size = e.call(TILE_GET_FLOAT, &args![owner, size_trait]).f64();
                    let two: f64 = e.global(TWO_DOUBLE);
                    let half = x87_div(size, two);
                    let map = e.get(this, MapMenu::pMapTile);
                    let scale = e.call(TILE_GET_FLOAT, &args![map, size_trait]).f64();
                    let point = e.mem.u32(map_point.addr() + offset);
                    let scaled = x87_mul(scale, x87_m32(point));
                    let value = x87_store32(x87_sub(half, scaled));
                    let map = e.get(this, MapMenu::pMapTile);
                    e.call(
                        TILE_SET_FLOAT,
                        &args![map, position_trait, x87_quiet_bits(value), 0u32],
                    );
                }
            });
        }
        3 => {
            if e.mem.u8(CHALLENGE_MODE_011DA36D) == 0 {
                let showing = e.get(this, MapMenu::bShowingAllNotes);
                e.set(this, MapMenu::bShowingAllNotes, u8::from(showing == 0));
                e.call(NOTE_LIST_FILTER, &args![base + 0x160, NOTE_FILTER_CALLBACK]);
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 0u32]);
                let showing = e.get(this, MapMenu::bShowingAllNotes);
                let showing_trait = e.call(TEXT_TO_TRAIT, &args![SHOWING_ALL_NOTES_TEXT]).u32();
                let tile_12 = map_tile(e, base, 12);
                e.call(
                    TILE_SET_INT,
                    &args![tile_12, showing_trait, u32::from(showing)],
                );
            } else {
                let kind = e.mem.u8(CHALLENGE_TYPE_011DA36E).wrapping_add(1);
                e.mem.set_u8(CHALLENGE_TYPE_011DA36E, kind);
                if (e.mem.u8(CHALLENGE_TYPE_011DA36E) as i8) > 2 {
                    e.mem.set_u8(CHALLENGE_TYPE_011DA36E, 0);
                }
                e.call(
                    CHALLENGE_LIST_FILTER,
                    &args![base + 0x1f0, CHALLENGE_FILTER_CALLBACK],
                );
                let kind = e.mem.u8(CHALLENGE_TYPE_011DA36E) as i8 as i32;
                let entry = e
                    .mem
                    .u32(CHALLENGE_TYPE_NAMES_011A03E4.wrapping_add((kind as u32).wrapping_mul(4)));
                let name = e.call(OBJECT_FIELD_00403DF0, &args![entry]).u32();
                let tile_21 = map_tile(e, base, 21);
                e.call(TILE_SET_STRING, &args![tile_21, 0xfc4u32, name, 1u32]);
                let tile_20 = map_tile(e, base, 20);
                e.call(TILE_SET_INT, &args![tile_20, 0xfa3u32, 0u32]);
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 0u32]);
            }
        }
        _ => {}
    }
}

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// `DoClick` id 0xC on the world map (0x20) and the local map (0x21): the
/// marker position from the map tile's centre (`004b71d0` set) or from the
/// cursor, the world position (`MapToWorldCoord`), its world space, the
/// north-rotation adjustment, the sound and the confirmation message box.
fn set_player_marker(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    let position = base + 0xfc;
    if e.call(FLAG_004B71D0, &args![]).bool() {
        let z = 0.0f32.to_bits();
        let mut coords = [0u32; 2];
        for (i, (size_trait, offset_trait)) in
            [(0xfb0u32, 0xfa2u32), (0xfb1, 0xfa1)].iter().enumerate()
        {
            let map = e.get(this, MapMenu::pMapTile);
            let owner = e.call(WORD_AT_28, &args![map]).u32();
            let size = e.call(TILE_GET_FLOAT, &args![owner, *size_trait]).f64();
            let two: f64 = e.global(TWO_DOUBLE);
            let half = x87_div(size, two);
            let map = e.get(this, MapMenu::pMapTile);
            let offset = e.call(TILE_GET_FLOAT, &args![map, *offset_trait]).f64();
            coords[i] = x87_quiet_bits(x87_store32(x87_sub(half, offset)));
        }
        e.with_stack(0xc, |e, point| {
            let result = e
                .call(POINT_SET_00416870, &args![point, coords[1], coords[0], z])
                .u32();
            copy_point(e, position, result);
        });
    } else {
        let z = 0.0f32.to_bits();
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let y = x87_store32(e.call(CURSOR_Y_007983C0, &args![interface]).f64());
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let x = x87_store32(e.call(CURSOR_X_007697F0, &args![interface]).f64());
        e.with_stack(0xc, |e, point| {
            let result = e.call(POINT_SET_00416870, &args![point, x, y, z]).u32();
            copy_point(e, position, result);
        });
        let map = e.get(this, MapMenu::pMapTile);
        let left = e.call(TILE_GETTER_00A013D0, &args![map]).f64();
        let x = e.mem.u32(position);
        e.mem
            .set_u32(position, x87_store32(x87_sub(x87_m32(x), left)));
        let map = e.get(this, MapMenu::pMapTile);
        let top = e.call(TILE_GETTER_00A01440, &args![map]).f64();
        let y = e.mem.u32(position + 4);
        e.mem
            .set_u32(position + 4, x87_store32(x87_sub(x87_m32(y), top)));
    }
    if e.get(this, MapMenu::iCurrentPage) == 0x20 {
        let space = e.get(this, MapMenu::pMapSpace);
        e.set(this, MapMenu::PlayerMarkerSpace, space);
        let space = e.get(this, MapMenu::PlayerMarkerSpace);
        let world = if e.call(FORM_TYPE, &args![space]).u32() == 0x41 {
            e.get(this, MapMenu::PlayerMarkerSpace)
        } else {
            0
        };
        let map = e.get(this, MapMenu::pMapTile);
        let scale = tile_float_bits(e, map, 0xfb1);
        e.with_stack(0x18, |e, temps| {
            let scaled = e
                .call(POINT_SCALE_0053D280, &args![position, temps, scale])
                .u32();
            let x = e.mem.u32(scaled);
            let y = e.mem.u32(scaled + 4);
            let z = e.mem.u32(scaled + 8);
            let result = e
                .call(
                    MAP_TO_WORLD_COORD,
                    &args![this, temps.addr() + 0xc, x, y, z, world],
                )
                .u32();
            copy_point(e, position, result);
            copy_point(e, position, position);
        });
    } else {
        let map = e.get(this, MapMenu::pMapTile);
        let scale = tile_float_bits(e, map, 0xfb1);
        e.with_stack(0x18, |e, temps| {
            let scaled = e
                .call(POINT_SCALE_0053D280, &args![position, temps, scale])
                .u32();
            let x = e.mem.u32(scaled);
            let y = e.mem.u32(scaled + 4);
            let z = e.mem.u32(scaled + 8);
            let result = e
                .call(
                    MAP_TO_WORLD_COORD,
                    &args![this, temps.addr() + 0xc, x, y, z, 0u32],
                )
                .u32();
            copy_point(e, position, result);
        });
        let x = e.mem.u32(position);
        let y = e.mem.u32(position + 4);
        let z = e.mem.u32(position + 8);
        let space = e
            .call(CALCULATE_PLAYER_MARKER_SPACE, &args![this, x, y, z])
            .u32();
        e.set(this, MapMenu::PlayerMarkerSpace, space);
        if e.get(this, MapMenu::PlayerMarkerSpace) != 0 {
            let space = e.get(this, MapMenu::PlayerMarkerSpace);
            e.call(ADJUST_MAP_MARKER_COORD, &args![space, position, 1u32]);
        }
    }
    if e.get(this, MapMenu::pAdjustNorthRotationCell) != 0 {
        let cell = e.get(this, MapMenu::pAdjustNorthRotationCell);
        e.call(
            ADJUST_FOR_NORTH_ROTATION,
            &args![cell, position, position, 0u32],
        );
    }
    let tile_6 = map_tile(e, base, 6);
    e.call(TILE_SET_INT, &args![tile_6, 0xfa3u32, 0u32]);
    play_named_sound(e, MARKER_ADDED_SOUND);
    let player = e.mem.u32(PLAYER_011DEA3C);
    let zero = 0.0f32.to_bits();
    if e.call(PLAYER_FLAG_00798400, &args![player]).u32() != 0 {
        let fourth = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4f54u32]).u32();
        let third = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_33a8u32]).u32();
        let second = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4960u32]).u32();
        let text = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4438u32]).u32();
        e.call(
            SHOW_MESSAGE_BOX,
            &args![
                text,
                0u32,
                0u32,
                SET_PLAYER_MARKER_CALLBACK,
                0u32,
                0x17u32,
                zero,
                zero,
                second,
                third,
                fourth,
                0u32
            ],
        );
    } else {
        let third = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_3684u32]).u32();
        let second = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_34f8u32]).u32();
        let text = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_3a2cu32]).u32();
        e.call(
            SHOW_MESSAGE_BOX,
            &args![
                text,
                0u32,
                0u32,
                SET_PLAYER_MARKER_CALLBACK,
                3u32,
                0x17u32,
                zero,
                zero,
                second,
                third,
                0u32
            ],
        );
    }
}

/// `DoClick` id 0x18 (notes page, not in challenge mode): the clicked note
/// is opened or closed; an audio note (type 0) or a conversation note (type
/// 3) queues its sound (`BSSimpleList<BSSoundHandle>::AddTail`) or runs its
/// conversation through a temporary `Character`, then the holotape sound
/// plays and the audio enters dialogue.
fn do_click_note(e: &mut Engine, this: Ptr<MapMenu>, tile: u32) {
    let base = this.addr();
    e.with_stack(0xc, |e, handle| {
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        let note = e
            .call(LIST_BOX_ITEM_OF_007E1110, &args![base + 0x160, tile])
            .u32();
        let audio_note = e.call(NOTE_TYPE, &args![note]).u32() == 0
            || e.call(NOTE_TYPE, &args![note]).u32() == 3;
        if note != 0 && (!e.call(FLAG_004B71D0, &args![]).bool() || audio_note) {
            let same = note == e.get(this, MapMenu::pSelectedNote);
            let mut stopped = false;
            let previous = e
                .call(LIST_BOX_GET_TILE_FOR, &args![base + 0x160, base + 0x90])
                .u32();
            if previous != 0 {
                let selected = e.mem.u32(TRAIT_SELECTED);
                e.call(TILE_SET_INT, &args![previous, selected, 0u32]);
            }
            if !same || (audio_note && e.get(this, MapMenu::pCurrentSound) == 0) {
                e.call(LOAD_NOTE, &args![this, note]);
                let selected = e.mem.u32(TRAIT_SELECTED);
                e.call(TILE_SET_INT, &args![tile, selected, 1u32]);
                e.set(this, MapMenu::pSelectedNote, note);
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0x1004u32, 1u32]);
            } else {
                e.set(this, MapMenu::pSelectedNote, 0);
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0x1004u32, 0u32]);
            }
            if e.get(this, MapMenu::pCurrentSound) != 0 {
                e.call(STOP_NOTE_SOUND, &args![this]);
                stopped = true;
            }
            if audio_note && !e.call(NOTE_FLAG_00798420, &args![note]).bool() {
                let alpha = float_constant(e, HALF_ALPHA_FLOAT);
                let alpha_trait = e.mem.u32(TRAIT_TEXT_ALPHA);
                e.call(TILE_SET_FLOAT, &args![tile, alpha_trait, alpha, 1u32]);
                e.call(NOTE_SET_READ_005E9300, &args![note, 1u32]);
            }
            if audio_note && (!same || !stopped) {
                play_note(e, this, note, handle.addr());
            }
        }
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

/// The sound part of `DoClick` id 0x18 (`handle` is its sound handle).
fn play_note(e: &mut Engine, this: Ptr<MapMenu>, note: u32, handle: u32) {
    let base = this.addr();
    if e.call(NOTE_TYPE, &args![note]).u32() == 0
        && e.call(NOTE_SOUND_005E9100, &args![note]).u32() != 0
    {
        let sound = e.call(NOTE_SOUND_005E9100, &args![note]).u32();
        let sound_id = e.call(WORD_AT_C, &args![sound]).u32();
        e.with_stack(0xc, |e, temp| {
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let found = e
                .call(AUDIO_SOUND_BY_ID, &args![audio, temp, sound_id, 0x101u32])
                .u32();
            e.call(SOUND_LIST_ADD_TAIL, &args![base + 0x98, found]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![temp]);
        });
        e.set(this, MapMenu::bSoundsLoading, 1);
    } else if e.call(NOTE_TYPE, &args![note]).u32() == 3
        && e.call(NOTE_TOPIC_005E9240, &args![note]).u32() != 0
        && e.call(NOTE_GET_SPEAKER, &args![note]).u32() != 0
    {
        run_note_conversation(e, this, note, handle);
    }
    if e.get(this, MapMenu::bSoundsLoading) != 0 {
        play_named_sound(e, HOLOTAPE_START_SOUND);
        e.call(DIALOG_FLAG_00762150, &args![]);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(AUDIO_ENTER_DIALOGUE, &args![audio]);
    }
}

/// A conversation note: a temporary `Character` (0x1c8 bytes) of the note's
/// speaker, a `Conversation` (0xc bytes) with the player and the note's
/// topic, and for each response its subtitle, its result script (before and
/// after), its sound at volume 0.9 queued; then both are deleted.
fn run_note_conversation(e: &mut Engine, this: Ptr<MapMenu>, note: u32, handle: u32) {
    let base = this.addr();
    let block = e.call(ALLOCATE, &args![0x1c8u32]).u32();
    let speaker = if block != 0 {
        e.call(CHARACTER_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.call(FORM_SET_TEMPORARY, &args![speaker]);
    let base_form = e.call(NOTE_GET_SPEAKER, &args![note]).u32();
    e.call(REFR_SET_OBJECT_REFERENCE, &args![speaker, base_form]);
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    let conversation = if block != 0 {
        let topic = e.call(NOTE_TOPIC_005E9240, &args![note]).u32();
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(
            CONVERSATION_CONSTRUCT,
            &args![block, speaker, player, topic],
        )
        .u32()
    } else {
        0
    };
    e.call(CONVERSATION_FIRST_ITEM, &args![conversation]);
    let has_item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32() != 0 && {
        let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
        e.call(FIRST_RESPONSE, &args![item]).bool()
    };
    if has_item {
        loop {
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            let response = e.call(GET_CURRENT_RESPONSE, &args![item]).u32();
            let text = e.call(SELF, &args![response]).u32();
            e.call(SUBTITLES_ADD_007A1AC0, &args![base + 0xa8, text]);
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            let info = e.call(WORD_AT_C, &args![item]).u32();
            e.call(TOPIC_INFO_RUN_RESULT, &args![info, 0u32, speaker]);
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            let response = e.call(GET_CURRENT_RESPONSE, &args![item]).u32();
            let file = e.call(PLUS_10, &args![response]).u32();
            let file_name = e.call(STRING_TEXT, &args![file]).u32();
            e.with_stack(0xc, |e, temp| {
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                let found = e
                    .call(
                        AUDIO_SOUND_BY_FILENAME,
                        &args![audio, temp, file_name, 0x105u32, 0u32],
                    )
                    .u32();
                e.call(SOUND_HANDLE_ASSIGN, &args![handle, found]);
                e.call(SOUND_HANDLE_DESTRUCT, &args![temp]);
            });
            let volume = float_constant(e, NOTE_VOLUME_FLOAT);
            e.call(SOUND_SET_VOLUME, &args![handle, volume]);
            e.call(SOUND_LIST_ADD_TAIL, &args![base + 0x98, handle]);
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            let info = e.call(WORD_AT_C, &args![item]).u32();
            e.call(TOPIC_INFO_RUN_RESULT, &args![info, 1u32, speaker]);
            e.set(this, MapMenu::bSoundsLoading, 1);
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            if !e.call(NEXT_RESPONSE, &args![item]).bool() {
                break;
            }
        }
    }
    if conversation != 0 {
        fn_00798450(e, Ptr::new(conversation), 1);
    }
    if speaker != 0 {
        e.vcall(speaker, 0x10, &args![1u32]);
    }
}

/// `DoClick` id 0x1a: on the local map (0x21), when the map was not dragged
/// (the map tile still at `WorldMapDragStart`, or `004b71d0` set), fast
/// travel is offered (trait `0xfaf` of tile 11 and `0093d660` on the player)
/// to the marker the clicked tile's `"_MarkerIndex"` picks from the world map
/// marker list (+0xd8); otherwise a tile whose `"_LocationName"` is not
/// `"Companion"` shows the message `007052f0`.
fn do_click_marker(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    let base = this.addr();
    if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        let not_dragged = if e.call(FLAG_004B71D0, &args![]).bool() {
            true
        } else if e.get(this, MapMenu::pMapTile) == 0 {
            false
        } else {
            let map = e.get(this, MapMenu::pMapTile);
            let x = e.call(TILE_GET_FLOAT, &args![map, 0xfa1u32]).f64();
            let start_x = x87_load32(e.mem.u32(base + 0x120));
            equal(start_x, x) && {
                let map = e.get(this, MapMenu::pMapTile);
                let y = e.call(TILE_GET_FLOAT, &args![map, 0xfa2u32]).f64();
                let start_y = x87_load32(e.mem.u32(base + 0x124));
                equal(start_y, y)
            }
        };
        if not_dragged {
            let tile_11 = map_tile(e, base, 11);
            if e.call(TILE_IS_TRUE, &args![tile_11, 0xfafu32]).bool() {
                let player = e.mem.u32(PLAYER_011DEA3C);
                if e.call(PLAYER_FLAG_0093D660, &args![player]).bool() {
                    offer_fast_travel(e, this, id, tile);
                    return;
                }
            }
        }
    }
    if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        let tile_11 = map_tile(e, base, 11);
        if !e.call(TILE_IS_TRUE, &args![tile_11, 0xfafu32]).bool() {
            let name_trait = e
                .call(ADD_USER_TRAIT, &args![LOCATION_NAME_TEXT, 0xffff_ffffu32])
                .u32();
            let name = e.call(TILE_GET_STRING, &args![tile, name_trait]).u32();
            if e.call(STRICMP, &args![name, COMPANION_TEXT]).u32() != 0 {
                let text = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_48e8u32]).u32();
                let two = float_constant(e, TWO_FLOAT);
                e.call(
                    SHOW_MESSAGE_007052F0,
                    &args![text, 2u32, 0u32, 0u32, two, 0u32],
                );
            }
        }
    }
}

/// The fast travel offer of `DoClick` id 0x1a.
fn offer_fast_travel(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    let base = this.addr();
    let mut index = 0i32;
    let mut node = base + 0xd8;
    while node != 0 {
        let item = e.call(SELF, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let wanted = f64::from(index);
        let marker_trait = e
            .call(ADD_USER_TRAIT, &args![MARKER_INDEX_TEXT, 0xffff_ffffu32])
            .u32();
        let value = e.call(TILE_GET_FLOAT, &args![tile, marker_trait]).f64();
        // `FCOMP; TEST AH,1; JNZ`: stops when the value is below the index or
        // unordered.
        if value.partial_cmp(&wanted).map_or(true, |o| o.is_lt()) {
            break;
        }
        let item = e.call(SELF, &args![node]).u32();
        let marker = e.mem.u32(item);
        e.set(this, MapMenu::pFastTravelMarker, marker);
        index = index.wrapping_add(1);
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
    let marker = e.get(this, MapMenu::pFastTravelMarker);
    let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
    if e.call(MARKER_GET_TRAVEL_LOC, &args![data]).bool() {
        e.vcall(base, 0x14, &args![id, tile]);
        let marker = e.get(this, MapMenu::pFastTravelMarker);
        let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
        let name = e.call(MARKER_NAME_00438EB0, &args![data]).u32();
        let question = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4a38u32]).u32();
        e.with_stack(0x100, |e, text| {
            e.call(
                BUFFER_FORMAT,
                &args![text, 0x100u32, FAST_TRAVEL_FORMAT, question, name],
            );
            let tile_6 = map_tile(e, base, 6);
            e.call(TILE_SET_INT, &args![tile_6, 0xfa3u32, 0u32]);
            let third = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_3684u32]).u32();
            let second = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_34f8u32]).u32();
            let zero = 0.0f32.to_bits();
            e.call(
                SHOW_MESSAGE_BOX,
                &args![
                    text,
                    0u32,
                    0u32,
                    FAST_TRAVEL_CALLBACK,
                    0u32,
                    0x17u32,
                    zero,
                    zero,
                    second,
                    third,
                    0u32
                ],
            );
        });
    } else {
        e.set(this, MapMenu::pFastTravelMarker, 0);
    }
}

// Translated from 00798450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `Conversation` `DoClick` makes: the
/// destructor body `0083b8d0`, then the object freed when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_00798450(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CONVERSATION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0079a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::DoIdle` (Xbox PDB): pauses the note sound while menu 0x3f5 is on
/// top (keeping the time) and resumes it once neither 0x3f5 nor 0x3e9 is
/// (adding the paused time to `iSoundPlayStartTime`). While the pip-boy is
/// up (`00974d90`): with another menu on top, the pip-boy tile fades in once
/// (trait `0xfa9` 255.0) on the first three pages; with the map menu on top,
/// by page: the world map steps its local map 25 times when asked, and the
/// world and local maps show the cursor position (with the cursor off the
/// 850x500 map the pip-boy tile's alpha is 255, otherwise 0 and a click
/// 0xc when `00a23a50` but not `00a24660`); the notes page marks the
/// selected note read (`"_TextAlpha"` 128) once its delay has passed; the
/// radio page updates (`0079c0c0` or `0079c090`). Then the playing sounds
/// are updated when any are loading or playing.
pub fn map_menu_do_idle(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    let paused = e.call(IS_TOP_MENU_ID, &args![0x3f5u32]).bool()
        && e.get(this, MapMenu::pCurrentSound) != 0
        && {
            let sound = e.get(this, MapMenu::pCurrentSound);
            let handle = e.call(SELF, &args![sound]).u32();
            e.call(SOUND_IS_PLAYING, &args![handle]).bool()
        };
    if paused {
        let sound = e.get(this, MapMenu::pCurrentSound);
        let handle = e.call(SELF, &args![sound]).u32();
        e.call(SOUND_PAUSE, &args![handle]);
        e.mem.set_u8(SOUND_PAUSED_011DA36F, 1);
        let now = e.call(TIMER_NOW, &args![TIMER_011F6394]).u32();
        e.mem.set_u32(PAUSE_TIME_011DA40C, now);
    } else if !e.call(IS_TOP_MENU_ID, &args![0x3f5u32]).bool()
        && !e.call(IS_TOP_MENU_ID, &args![0x3e9u32]).bool()
        && e.get(this, MapMenu::pCurrentSound) != 0
        && e.mem.u8(SOUND_PAUSED_011DA36F) != 0
    {
        let sound = e.get(this, MapMenu::pCurrentSound);
        let handle = e.call(SELF, &args![sound]).u32();
        e.call(SOUND_PLAY, &args![handle, 0u32]);
        e.mem.set_u8(SOUND_PAUSED_011DA36F, 0);
        let now = e.call(TIMER_NOW, &args![TIMER_011F6394]).u32();
        let start = now
            .wrapping_sub(e.mem.u32(PAUSE_TIME_011DA40C))
            .wrapping_add(e.get(this, MapMenu::iSoundPlayStartTime));
        e.set(this, MapMenu::iSoundPlayStartTime, start);
    }
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if !e.call(PIPBOY_FLAG_00974D90, &args![pipboy]).bool() {
        return;
    }
    if e.call(IS_TOP_MENU_ID, &args![0x3ffu32]).bool() {
        let page = e.mem.u32(map_menu(e) + 0x80);
        // The compiler's switch on `page - 0x20` (0 to 4; jump table
        // `0079a5f4`).
        match page.wrapping_sub(0x20) {
            0 => {
                let wanted = e.call(BYTE_OF_00408D60, &args![OBJECT_011DA3B8]).u32();
                if e.mem.u8(wanted) != 0 {
                    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
                    if e.call(PIPBOY_FLAG_007FAEF0, &args![pipboy]).bool() {
                        let menu = map_menu(e);
                        e.mem.set_u32(menu + 0x128, 0);
                        for _ in 0..0x19 {
                            e.call(MAP_STEP_0079FFB0, &args![0u32]);
                        }
                    }
                }
                idle_show_position(e, this);
            }
            1 => idle_show_position(e, this),
            3 => {
                if e.get(this, MapMenu::uiNoteEnterStartTime) != 0 {
                    let now = e.call(TIMER_NOW, &args![TIMER_011F6394]).u32();
                    let elapsed =
                        f64::from(now.wrapping_sub(e.get(this, MapMenu::uiNoteEnterStartTime)));
                    let delay = e
                        .call(OBJECT_VALUE_ADDRESS_00403E20, &args![0x011d_47bcu32])
                        .u32();
                    let thousand: f64 = e.global(THOUSAND_DOUBLE);
                    let wait = x87_mul(x87_load32(e.mem.u32(delay)), thousand);
                    // `FCOMP; TEST AH,0x41; JP`: on when the wait is not
                    // above the elapsed time (ordered).
                    if matches!(
                        wait.partial_cmp(&elapsed),
                        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
                    ) {
                        e.with_stack(4, |e, note_slot| {
                            let selected = e.get(this, MapMenu::pSelectedNote);
                            e.mem.set_u32(note_slot.addr(), selected);
                            if e.call(FLAG_004B71D0, &args![]).bool()
                                || e.get(this, MapMenu::pSelectedNote) == 0
                            {
                                let note = fn_007a1910(e, Ptr::new(base + 0x160));
                                e.mem.set_u32(note_slot.addr(), note);
                            }
                            let note_tile = e
                                .call(LIST_BOX_GET_TILE_FOR, &args![base + 0x160, note_slot])
                                .u32();
                            let note = e.mem.u32(note_slot.addr());
                            if note != 0 && note_tile != 0 {
                                let alpha = float_constant(e, HALF_ALPHA_FLOAT);
                                let alpha_trait = e.mem.u32(TRAIT_TEXT_ALPHA);
                                e.call(TILE_SET_FLOAT, &args![note_tile, alpha_trait, alpha, 1u32]);
                                let note = e.mem.u32(note_slot.addr());
                                e.call(NOTE_SET_READ_005E9300, &args![note, 1u32]);
                                e.set(this, MapMenu::uiNoteEnterStartTime, 0);
                            }
                        });
                    }
                }
            }
            4 => {
                if e.call(FLAG_00798440, &args![]).bool() {
                    e.call(PAGE_0079C0C0, &args![this]);
                } else {
                    fn_0079c090(e, this);
                }
            }
            _ => {}
        }
        e.mem.set_u8(FADED_011DA408, 0);
    } else if e.mem.u8(FADED_011DA408) == 0
        && !e.call(FLAG_004B71D0, &args![]).bool()
        && e.get(this, MapMenu::iCurrentPage) <= 0x21
    {
        let alpha = float_constant(e, FULL_ALPHA_FLOAT);
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let pipboy_tile = e.call(WORD_AT_28, &args![interface]).u32();
        e.call(TILE_SET_FLOAT, &args![pipboy_tile, 0xfa9u32, alpha, 1u32]);
        e.mem.set_u8(FADED_011DA408, 1);
    }
    if e.get(this, MapMenu::bSoundsLoading) != 0 || e.get(this, MapMenu::pCurrentSound) != 0 {
        e.call(UPDATE_PLAYING_SOUNDS, &args![this]);
    }
}

/// The world and local map cases of `DoIdle` (pages 0x20 and 0x21), unless
/// `004b71d0` is set.
fn idle_show_position(e: &mut Engine, this: Ptr<MapMenu>) {
    if e.call(FLAG_004B71D0, &args![]).bool() {
        return;
    }
    let base = this.addr();
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let cursor_x = e.call(CURSOR_X_007697F0, &args![interface]).f64();
    let tile_6 = map_tile(e, base, 6);
    let owner = e.call(WORD_AT_28, &args![tile_6]).u32();
    let left = e.call(TILE_GETTER_00A013D0, &args![owner]).f64();
    let mut x = x87_store32(x87_sub(cursor_x, left));
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let cursor_y = e.call(CURSOR_Y_007983C0, &args![interface]).f64();
    let tile_6 = map_tile(e, base, 6);
    let owner = e.call(WORD_AT_28, &args![tile_6]).u32();
    let top = e.call(TILE_GETTER_00A01440, &args![owner]).f64();
    let mut y = x87_store32(x87_sub(cursor_y, top));
    let zero: f64 = e.global(ZERO_DOUBLE);
    let width: f64 = e.global(MAP_WIDTH_DOUBLE);
    let height: f64 = e.global(MAP_HEIGHT_DOUBLE);
    let outside = below(x87_load32(x), zero)
        || below(x87_load32(y), zero)
        || above(x87_load32(x), width)
        || above(x87_load32(y), height);
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    if outside {
        let alpha = float_constant(e, FULL_ALPHA_FLOAT);
        let pipboy_tile = e.call(WORD_AT_28, &args![interface]).u32();
        e.call(TILE_SET_FLOAT, &args![pipboy_tile, 0xfa9u32, alpha, 1u32]);
    } else {
        let pipboy_tile = e.call(WORD_AT_28, &args![interface]).u32();
        e.call(
            TILE_SET_FLOAT,
            &args![pipboy_tile, 0xfa9u32, 0.0f32.to_bits(), 1u32],
        );
        let holder = e.mem.u32(OBJECT_011DEA0C);
        let object = e.call(OBJECT_00877720, &args![holder]).u32();
        if e.call(OBJECT_CHECK_00A23A50, &args![object, 1u32, 1u32])
            .u32()
            != 0
        {
            let holder = e.mem.u32(OBJECT_011DEA0C);
            let object = e.call(OBJECT_00877720, &args![holder]).u32();
            if e.call(OBJECT_CHECK_00A24660, &args![object, 0xeu32, 1u32])
                .u32()
                == 0
            {
                e.vcall(base, 0xc, &args![0xcu32, 0u32]);
            }
        }
    }
    let two: f64 = e.global(TWO_DOUBLE);
    let tile_6 = map_tile(e, base, 6);
    let width = e.call(TILE_GET_FLOAT, &args![tile_6, 0xfb1u32]).f64();
    x = x87_store32(x87_sub(x87_m32(x), x87_div(width, two)));
    let tile_6 = map_tile(e, base, 6);
    let height = e.call(TILE_GET_FLOAT, &args![tile_6, 0xfb0u32]).f64();
    y = x87_store32(x87_sub(x87_m32(y), x87_div(height, two)));
    let tile_6 = map_tile(e, base, 6);
    e.call(
        TILE_SET_FLOAT,
        &args![tile_6, 0xfa1u32, x87_quiet_bits(x), 1u32],
    );
    let tile_6 = map_tile(e, base, 6);
    e.call(
        TILE_SET_FLOAT,
        &args![tile_6, 0xfa2u32, x87_quiet_bits(y), 1u32],
    );
    e.call(SHOW_POSITION_00799DC0, &args![this]);
}

// Translated from 0079aba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079aba0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the two strings) is not translated.
/// `MapMenu::UpdateMenu` (Xbox PDB): with a map menu and the pip-boy up,
/// formats the date and time (`"%s, %d:%02d"`: the calendar's date string,
/// the hour and the minutes) into tile 1's text and, through `00578870` on
/// the player, tile 0's; when `00408b20(language, "ENGLISH")` is non-zero
/// sets the five tab titles of tiles 11 and 12 (recalculating their `iUT_Title`
/// value), the text of tile 17's child 0x24 and of `"MM_Headline_Title"`.
/// Then `UpdateCurrentPage`.
pub fn map_menu_update_menu(e: &mut Engine) {
    if map_menu(e) == 0 {
        return;
    }
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if !e.call(PIPBOY_FLAG_00974D90, &args![pipboy]).bool() {
        return;
    }
    e.with_stack(0x10, |e, strings| {
        let text = strings.addr();
        let date = strings.addr() + 8;
        e.call(STRING_CONSTRUCT, &args![text]);
        let hour_value = e.call(CALENDAR_GET_HOUR, &args![CALENDAR_011DE7B8]).f64();
        let hour_bits = x87_store32(hour_value);
        let whole = e.call(FTOL2, &args![x87_load32(hour_bits)]).i32();
        let sixty: f64 = e.global(SIXTY_DOUBLE);
        let fraction = x87_sub(x87_m32(hour_bits), f64::from(whole));
        let minutes = e.call(FTOL2, &args![x87_mul(fraction, sixty)]).u32();
        let hour = e.call(FTOL2, &args![x87_load32(hour_bits)]).u32();
        let date_string = e
            .call(CALENDAR_GET_DATE_STRING, &args![CALENDAR_011DE7B8, date])
            .u32();
        let date_text = e.call(STRING_TEXT, &args![date_string]).u32();
        e.call(
            STRING_FORMAT,
            &args![text, DATE_TIME_FORMAT, date_text, hour, minutes],
        );
        e.call(STRING_DESTRUCT, &args![date]);
        let characters = e.call(STRING_TEXT, &args![text]).u32();
        let tile_1 = map_tile(e, map_menu(e), 1);
        e.call(TILE_SET_STRING, &args![tile_1, 0xfc4u32, characters, 1u32]);
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(PLAYER_MESSAGE_00578870, &args![player, text]);
        let characters = e.call(STRING_TEXT, &args![text]).u32();
        let tile_0 = map_tile(e, map_menu(e), 0);
        e.call(TILE_SET_STRING, &args![tile_0, 0xfc4u32, characters, 1u32]);
        let language = e.call(OBJECT_FIELD_00403DF0, &args![0x011c_3cb4u32]).u32();
        if e.call(COMPARE_00408B20, &args![language, ENGLISH_TEXT])
            .u32()
            != 0
        {
            for (tile_index, entries) in [
                (
                    11u32,
                    [
                        0x011d_1da8u32,
                        0x011d_3f64,
                        0x011d_45f4,
                        0x011d_3414,
                        0x011d_2f10,
                    ],
                ),
                (
                    12,
                    [
                        0x011d_3180,
                        0x011d_4cfc,
                        0x011d_27a8,
                        0x011d_3798,
                        0x011d_3210,
                    ],
                ),
            ] {
                for (entry, name) in entries.iter().zip(TITLE_N_TEXT) {
                    let title = e.call(OBJECT_FIELD_00403DF0, &args![*entry]).u32();
                    let title_trait = e.call(TEXT_TO_TRAIT, &args![name]).u32();
                    let tile = map_tile(e, map_menu(e), tile_index);
                    e.call(TILE_SET_STRING, &args![tile, title_trait, title, 1u32]);
                }
                let title_trait = e.mem.u32(TRAIT_TITLE);
                let tile = map_tile(e, map_menu(e), tile_index);
                let value = e.call(TILE_GET_VALUE, &args![tile, title_trait]).u32();
                e.call(VALUE_CALCULATE, &args![value, 1u32]);
            }
            let name = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4aecu32]).u32();
            let tabline = map_tile(e, map_menu(e), 17);
            let child = e.call(TILE_GET_CHILD_BY_ID, &args![tabline, 0x24u32]).u32();
            e.call(TILE_SET_STRING, &args![child, 0xfc4u32, name, 1u32]);
            let headline = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_21e4u32]).u32();
            let menu = map_menu(e);
            let menu_tile = e.call(WORD_AT_4, &args![menu]).u32();
            let child = e
                .call(
                    TILE_GET_CHILD_BY_NAME,
                    &args![menu_tile, HEADLINE_TITLE_TEXT],
                )
                .u32();
            e.call(TILE_SET_STRING, &args![child, 0xfc4u32, headline, 1u32]);
        }
        let menu = map_menu(e);
        e.call(UPDATE_CURRENT_PAGE, &args![menu]);
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

// Translated from 0079bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `this + 0x6bc` (the class is unconfirmed).
pub fn fn_0079bc30(e: &mut Engine, this: Ptr) -> u32 {
    let _ = e;
    this.addr().wrapping_add(0x6bc)
}

// Translated from 0079c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears `fWaveformAnimationStartTime` and
/// tile 16's trait `0xfa3`.
pub fn fn_0079c090(e: &mut Engine, this: Ptr<MapMenu>) {
    e.set(this, MapMenu::fWaveformAnimationStartTime, 0.0);
    let tile = map_tile(e, this.addr(), 16);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
}

// Translated from 007a1480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at `011dea14`.
pub fn fn_007a1480(e: &mut Engine) -> u32 {
    e.mem.u32(VALUE_011DEA14)
}

// Translated from 007a1490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::ClearMapMemory` (Xbox PDB): with a map menu whose tile 2 has an
/// object (`0056c7f0`), hands every item of it to its vtable +0xe8, deletes
/// tile 2's and 4's children, clears trait `0xfa3` of tiles 2 and 3, the
/// fade controls `0xff5` of tiles 3 and 5 and both marker lists, the
/// waypoints, `uiNextLocalMapGrid` and `pMapTile`; clears the map tile's
/// texture (tile 16's shader property, its text trait `0xfcc` emptied) and
/// `0079c090`.
pub fn map_menu_clear_map_memory(e: &mut Engine) {
    if map_menu(e) == 0 {
        return;
    }
    let tile_2 = map_tile(e, map_menu(e), 2);
    let object = e.call(TILE_OBJECT_0056C7F0, &args![tile_2]).u32();
    if object == 0 {
        return;
    }
    let mut index = 0u32;
    while index < e.call(OBJECT_COUNT_0043B480, &args![object]).u32() {
        let item = e.call(OBJECT_ITEM_0043B4A0, &args![object, index]).u32();
        if item != 0 {
            e.vcall(object, 0xe8, &args![item]);
        }
        index = index.wrapping_add(1);
    }
    let tile_2 = map_tile(e, map_menu(e), 2);
    e.call(TILE_DELETE_CHILDREN, &args![tile_2]);
    let tile_2 = map_tile(e, map_menu(e), 2);
    e.call(TILE_SET_INT, &args![tile_2, 0xfa3u32, 0u32]);
    let tile_3 = map_tile(e, map_menu(e), 3);
    e.call(TILE_SET_INT, &args![tile_3, 0xfa3u32, 0u32]);
    let tile_3 = map_tile(e, map_menu(e), 3);
    e.call(TILE_REMOVE_FADE_CONTROL, &args![tile_3, 0xff5u32]);
    let menu = map_menu(e);
    e.call(LIST_REMOVE_ALL, &args![menu + 0xe0]);
    let tile_4 = map_tile(e, map_menu(e), 4);
    e.call(TILE_DELETE_CHILDREN, &args![tile_4]);
    let tile_5 = map_tile(e, map_menu(e), 5);
    e.call(TILE_REMOVE_FADE_CONTROL, &args![tile_5, 0xff5u32]);
    let menu = map_menu(e);
    e.call(LIST_REMOVE_ALL, &args![menu + 0xd8]);
    let menu = map_menu(e);
    e.call(WAYPOINTS_CLEAR, &args![menu + 0x220, 1u32]);
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0x128, 0);
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0x84, 0);
    let tile_16 = map_tile(e, map_menu(e), 16);
    let property = tile_shader_property(e, tile_16);
    if property != 0 && e.call(PROPERTY_CHECK_009C8640, &args![property]).u32() != 0 {
        e.call(SET_TILE_TEXTURE, &args![property, 0u32]);
        let tile_16 = map_tile(e, map_menu(e), 16);
        e.call(TILE_SET_STRING, &args![tile_16, 0xfccu32, EMPTY_TEXT, 1u32]);
    }
    let menu = map_menu(e);
    fn_0079c090(e, Ptr::new(menu));
}

// Translated from 007a1670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::ClearWorldMapTexture` (Xbox PDB): unless the map menu is on top
/// on the local map (page 0x21; `pMe` is read without a null check there),
/// clears tile 4's shader property texture (when `009c8640` says it has
/// one), empties its text trait `0xfcc` and sets the byte `011da36c`.
pub fn map_menu_clear_world_map_texture(e: &mut Engine) {
    if e.call(IS_TOP_MENU_ID, &args![0x3ffu32]).bool() && e.mem.u32(map_menu(e) + 0x80) == 0x21 {
        return;
    }
    if map_menu(e) == 0 {
        return;
    }
    let tile_4 = map_tile(e, map_menu(e), 4);
    let property = tile_shader_property(e, tile_4);
    if property != 0 && e.call(PROPERTY_CHECK_009C8640, &args![property]).u32() != 0 {
        e.call(SET_TILE_TEXTURE, &args![property, 0u32]);
        let tile_4 = map_tile(e, map_menu(e), 4);
        e.call(TILE_SET_STRING, &args![tile_4, 0xfccu32, EMPTY_TEXT, 1u32]);
        e.mem.set_u8(TEXTURE_CLEARED_011DA36C, 1);
    }
}

/// `007a17a0` and `007a17e0`: the magnification trait of the map menu's tile
/// `index`, 1.0 without a map menu.
fn magnification(e: &mut Engine, index: u32) -> f32 {
    let bits = if map_menu(e) == 0 {
        1.0f32.to_bits()
    } else {
        let id = e.mem.u32(TRAIT_MAGNIFICATION);
        let tile = map_tile(e, map_menu(e), index);
        tile_float_bits(e, tile, id)
    };
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 007a17a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the magnification (`iUT_Magnification`) of
/// the map menu's tile 2, 1.0 without a map menu.
pub fn fn_007a17a0(e: &mut Engine) -> f32 {
    magnification(e, 2)
}

// Translated from 007a17e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the magnification (`iUT_Magnification`) of
/// the map menu's tile 4, 1.0 without a map menu.
pub fn fn_007a17e0(e: &mut Engine) -> f32 {
    magnification(e, 4)
}

// Translated from 007a1880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::SetRootMapSpace` (Xbox PDB): with a map menu, sets
/// `pRootMapSpace` to `space` and walks it up to its top parent world
/// (`TESWorldSpace::GetParentWorldSimple`); `bRootSpaceChanged` says whether
/// it differs from before.
pub fn map_menu_set_root_map_space(e: &mut Engine, space: u32) {
    if map_menu(e) == 0 {
        return;
    }
    let old = e.mem.u32(map_menu(e) + 0x10c);
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0x10c, space);
    loop {
        let current = e.mem.u32(map_menu(e) + 0x10c);
        if current == 0 || e.call(WORLD_GET_PARENT, &args![current]).u32() == 0 {
            break;
        }
        let current = e.mem.u32(map_menu(e) + 0x10c);
        let parent = e.call(WORLD_GET_PARENT, &args![current]).u32();
        let menu = map_menu(e);
        e.mem.set_u32(menu + 0x10c, parent);
    }
    let changed = old != e.mem.u32(map_menu(e) + 0x10c);
    let menu = map_menu(e);
    e.mem.set_u8(menu + 0x110, u8::from(changed));
}

// Translated from 007a1910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the item of the list box's highlighted tile
/// (`007e1110(this, pHighlightedItem)`).
pub fn fn_007a1910(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(LIST_BOX_ITEM_OF_007E1110, &args![this, highlighted])
        .u32()
}

// Translated from 007a1930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `007a1950(this, 0, value)` (flag bit 0 of
/// `usFlags`).
pub fn fn_007a1930(e: &mut Engine, this: Ptr<ListBox>, value: u8) {
    fn_007a1950(e, this, 0, value);
}

// Translated from 007a1950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets (`value` non-zero) or clears bit `bit`
/// (the shift count masked to 5 bits, as `SHL` does) of the list box's
/// `usFlags`.
pub fn fn_007a1950(e: &mut Engine, this: Ptr<ListBox>, bit: u8, value: u8) {
    let mask = 1u32 << (u32::from(bit) & 0x1f);
    let flags = u32::from(e.get(this, ListBox::usFlags));
    let flags = if value != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.set(this, ListBox::usFlags, flags as u16);
}

// Translated from 007a1a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<BSStringT<char>,1024>` constructor (RTTI of the vtable
/// `01075530`): the vtable, then `007a63f0(this, 0, 0)`. Returns `this`.
pub fn fn_007a1a70(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01075530);
    fn_007a63f0(e, this, 0, 0);
    this
}

// Translated from 007a1c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<BSStringT<char>,1024>`): with a
/// buffer, destroys its elements (`007a63b0(buffer, iSize)`); with `release`
/// set also frees the buffer (`006a8500`) and clears the buffer and the
/// reserved size; then clears the size.
pub fn fn_007a1c30(e: &mut Engine, this: Ptr<BSSimpleArray>, release: u8) {
    if e.get(this, BSSimpleArray::pBuffer) == 0 {
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    e.call(ARRAY_DESTROY_ELEMENTS_007A63B0, &args![this, buffer, size]);
    if release != 0 {
        e.call(ARRAY_FREE_006A8500, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

// Translated from 007a62e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a list box item (tile, the word at `data`,
/// the filtered byte cleared). Returns `this`.
pub fn fn_007a62e0(e: &mut Engine, this: Ptr, tile: u32, data: u32) -> Ptr {
    e.mem.set_u32(this.addr(), tile);
    let word = e.mem.u32(data);
    e.mem.set_u32(this.addr() + 4, word);
    e.mem.set_u8(this.addr() + 8, 0);
    this
}

// Translated from 007a63f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<BSStringT<char>,1024>`'s
/// constructor body): clears the buffer, size and reserved size; reserves
/// `max(reserve, size)` elements through vtable +4 when that is non-zero,
/// then constructs `size` elements (`007a6310`) and sets the size.
pub fn fn_007a63f0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let reserve = if size > reserve { size } else { reserve };
    if reserve > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_CONSTRUCT_ELEMENTS_007A6310,
            &args![this, buffer, size],
        );
        e.set(this, BSSimpleArray::iSize, size);
    }
}

/// The function-local statics the list box methods keep trait ids in: the
/// guard word (bit `bit` says the id has been looked up) and the id.
#[derive(Clone, Copy)]
struct StaticTrait {
    guard: u32,
    bit: u32,
    cache: u32,
    name: u32,
}

/// Looks the static trait id up the first time: the guard bit is set before
/// `Tile::AddUserTrait(name, -1)` runs, as the code does.
fn static_trait(e: &mut Engine, which: StaticTrait) {
    if e.mem.u32(which.guard) & which.bit == 0 {
        let guard = e.mem.u32(which.guard);
        e.mem.set_u32(which.guard, guard | which.bit);
        let id = e
            .call(ADD_USER_TRAIT, &args![which.name, 0xffff_ffffu32])
            .u32();
        e.mem.set_u32(which.cache, id);
    }
}

/// One instance of the `ListBox<T *>` template whose methods this unit
/// holds: the vtable, the statics of `IsEnabled` and `UpdateHighlightBoxPos`
/// and the instance's `NumUnfiltered` (not in this unit). The instances'
/// code is the same apart from these.
#[derive(Clone, Copy)]
struct ListBoxKind {
    vtable: u32,
    enabled: StaticTrait,
    highlight_y: StaticTrait,
    selected_height: StaticTrait,
    num_unfiltered: u32,
}

const fn list_box_kind(
    vtable: u32,
    enabled_guard: u32,
    highlight_guard: u32,
    num_unfiltered: u32,
) -> ListBoxKind {
    ListBoxKind {
        vtable,
        enabled: StaticTrait {
            guard: enabled_guard,
            bit: 1,
            cache: enabled_guard - 4,
            name: ENABLED_TEXT,
        },
        highlight_y: StaticTrait {
            guard: highlight_guard,
            bit: 1,
            cache: highlight_guard - 4,
            name: HIGHLIGHT_Y_TEXT,
        },
        selected_height: StaticTrait {
            guard: highlight_guard,
            bit: 2,
            cache: highlight_guard - 8,
            name: SELECTED_HEIGHT_TEXT,
        },
        num_unfiltered,
    }
}

/// `ListBox<TESQuest *>` (vtable RTTI `.?AV?$ListBox@PAVTESQuest@@@@`).
const QUEST_LIST: ListBoxKind = list_box_kind(0x0107_5544, 0x011d_a460, 0x011d_a46c, 0x007a_86e0);
/// `ListBox<BGSNote *>` (RTTI `.?AV?$ListBox@PAVBGSNote@@@@`).
const NOTE_LIST: ListBoxKind = list_box_kind(0x0107_556c, 0x011d_a47c, 0x011d_a488, 0x007a_6a50);
/// `ListBox<TESObjectREFR *>` (RTTI `.?AV?$ListBox@PAVTESObjectREFR@@@@`).
const REFERENCE_LIST: ListBoxKind =
    list_box_kind(0x0107_5594, 0x011d_a498, 0x011d_a4a4, 0x007a_71b0);
/// `ListBox<BGSQuestObjective *>` (RTTI
/// `.?AV?$ListBox@PAVBGSQuestObjective@@@@`).
const OBJECTIVE_LIST: ListBoxKind =
    list_box_kind(0x0107_55bc, 0x011d_a4b4, 0x011d_a4c0, 0x007a_7830);
/// `ListBox<TESChallenge *>` (RTTI `.?AV?$ListBox@PAVTESChallenge@@@@`).
const CHALLENGE_LIST: ListBoxKind =
    list_box_kind(0x0107_55e4, 0x011d_a4d0, 0x011d_a4dc, 0x007a_8030);

/// The constructor of a `ListBox<T *>` instance: the `ListBoxBase` base
/// (`007240c0`), the embedded list (+4, `0096a2d0`), the vtable, the fields
/// cleared, `007a1950(this, 0, 1)` and the list box handed to `00721130`.
fn list_box_construct(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) -> Ptr<ListBox> {
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), kind.vtable);
    e.set(this, ListBox::pScrollBar, Ptr::NULL);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    e.set(this, ListBox::pContainerRect, Ptr::NULL);
    e.set(this, ListBox::pTemplateName, Ptr::NULL);
    e.set(this, ListBox::usFlags, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fSavedScrollbarValue, 0.0);
    e.set(this, ListBox::fSavedHighlightIndex, 0.0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    fn_007a1950(e, this, 0, 1);
    e.call(LIST_BOX_REGISTER, &args![this]);
    this
}

/// The destructor of a `ListBox<T *>` instance: the vtable, `RemoveAll`, the
/// list box handed to `007211c0`, then the embedded list destroyed.
fn list_box_destruct(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) {
    e.mem.set_u32(this.addr(), kind.vtable);
    list_box_remove_all(e, this, kind);
    e.call(LIST_BOX_UNREGISTER, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

/// The item word of a list node, reached through `006815c0` as the code
/// does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(SELF, &args![node]).u32();
    e.mem.u32(node)
}

/// Deletes the `ListBoxItem` (`pTile` +0, `data` +4) that `item` reads each
/// time: the tile through its deleting destructor (vtable slot 0, flag 1)
/// unless `KEEP_ITEM_TILES` is set, the data (`004459e0` with 1) when flag
/// bit 1 is set, then the item freed.
fn delete_item(e: &mut Engine, this: Ptr<ListBox>, item: impl Fn(&mut Engine) -> u32) {
    if e.mem.u8(KEEP_ITEM_TILES) == 0 {
        let entry = item(e);
        let tile = e.mem.u32(entry);
        if tile != 0 {
            e.vcall(tile, 0, &args![1u32]);
        }
    }
    if e.call(LIST_BOX_HAS_FLAG, &args![this, 1u32]).bool() {
        let entry = item(e);
        let data = e.mem.u32(entry + 4);
        if data != 0 {
            e.call(ITEM_DATA_DELETE, &args![data, 1u32]);
        }
    }
    let entry = item(e);
    e.call(FREE, &args![entry]);
}

/// `ListBox<T *>::RemoveAll`: while the second node (+8) and its item are
/// non-null, deletes the item, clears the node's next pointer (`006ecd40`
/// with 0), deletes the node (`004702f0` with 1) and makes the following node
/// the second; then deletes the first item (+4). Clears the first item,
/// `usNextIndex` and `fTotalListHeight`, sets the container's
/// `"_num_filtered"` to 0.0, then `SetTotalListHeight`, clears
/// `pHighlightedItem` and `UpdateHighlightBoxPos`.
fn list_box_remove_all(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) {
    let base = this.addr();
    loop {
        let second = e.mem.u32(base + 8);
        if second == 0 || node_item(e, second) == 0 {
            break;
        }
        let second = e.mem.u32(base + 8);
        let next = e.call(WORD_AT_4, &args![second]).u32();
        delete_item(e, this, |e| {
            let node = e.mem.u32(base + 8);
            node_item(e, node)
        });
        let second = e.mem.u32(base + 8);
        e.call(NODE_SET_NEXT, &args![second, 0u32]);
        let second = e.mem.u32(base + 8);
        if second != 0 {
            e.call(NODE_DELETE, &args![second, 1u32]);
        }
        e.mem.set_u32(base + 8, next);
    }
    if e.mem.u32(base + 4) != 0 {
        delete_item(e, this, |e| e.mem.u32(base + 4));
    }
    e.mem.set_u32(base + 4, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    if !e.get(this, ListBox::pContainerRect).is_null() {
        let num_filtered = e.call(TEXT_TO_TRAIT, &args![NUM_FILTERED_TEXT]).u32();
        let container = e.get(this, ListBox::pContainerRect);
        e.call(
            TILE_SET_FLOAT,
            &args![container, num_filtered, 0.0f32.to_bits(), 1u32],
        );
    }
    list_box_set_total_list_height(e, this, kind);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    list_box_update_highlight_box_pos(e, this, kind);
}

/// `ListBox<T *>::Highlight`: when the list is enabled and `tile` is null,
/// or is one of its items (`007242f0`) and `"_enabled"`, makes it
/// `pHighlightedItem`, moves the highlight box and returns true; otherwise
/// false.
fn list_box_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32, kind: ListBoxKind) -> bool {
    if !list_box_is_enabled(e, this, kind) {
        return false;
    }
    if tile != 0 {
        if !e.call(LIST_BOX_BELONGS_TO, &args![this, tile]).bool() {
            return false;
        }
        let enabled = e
            .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
            .u32();
        if !e.call(TILE_IS_TRUE, &args![tile, enabled]).bool() {
            return false;
        }
    }
    e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    list_box_update_highlight_box_pos(e, this, kind);
    true
}

/// `ListBox<T *>::IsEnabled`: the container exists and its `"_enabled"`
/// trait (a function-local static id) is true.
fn list_box_is_enabled(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) -> bool {
    static_trait(e, kind.enabled);
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return false;
    }
    let enabled = e.mem.u32(kind.enabled.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_IS_TRUE, &args![container, enabled]).bool()
}

/// The tile's byte (`0071aa60`) set to `flag` and the tile queued for an
/// update with flags 8.
fn mark_tile(e: &mut Engine, tile: u32, flag: u32) {
    e.call(TILE_FLAG_SETTER_0071AA60, &args![tile, flag]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

/// `ListBox<T *>::UpdateHighlightBoxPos`: looks the `"_highlight_y"` and
/// `"_selected_height"` trait ids up (statics); with a container and no
/// highlighted item, sets them to -1 and 0 on the container; with one, makes
/// both values follow the highlighted tile's traits `0xfa2` and `0xfb0`
/// (action `0x7d0`, then calculated) and gives the container's
/// `"lb_highlight_box"` tile, and its `"top"`, `"bottom"`, `"left"` and
/// `"right"` children when `"top"` exists (the other three are not checked
/// for null), the highlighted tile's byte (`0071aa80`, as 1 or 0) and an
/// update. The code's two branches differ only in that byte.
fn list_box_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) {
    static_trait(e, kind.highlight_y);
    static_trait(e, kind.selected_height);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() {
        let y_trait = e.mem.u32(kind.highlight_y.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, y_trait, 0xffff_ffffu32]);
        let height_trait = e.mem.u32(kind.selected_height.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, height_trait, 0u32]);
        return;
    }
    let y_trait = e.mem.u32(kind.highlight_y.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let y_value = e.call(TILE_GET_VALUE, &args![container, y_trait]).u32();
    let height_trait = e.mem.u32(kind.selected_height.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let height_value = e
        .call(TILE_GET_VALUE, &args![container, height_trait])
        .u32();
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![y_value, 0x7d0u32, highlighted, 0xfa2u32],
    );
    e.call(VALUE_CALCULATE, &args![y_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![height_value, 0x7d0u32, highlighted, 0xfb0u32],
    );
    e.call(VALUE_CALCULATE, &args![height_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    let flag = u32::from(
        e.call(TILE_FLAG_GETTER_0071AA80, &args![highlighted])
            .bool(),
    );
    let container = e.get(this, ListBox::pContainerRect);
    let box_tile = e
        .call(TILE_FIND_BY_NAME, &args![container, HIGHLIGHT_BOX_TEXT])
        .u32();
    if box_tile == 0 {
        return;
    }
    let top = e
        .call(TILE_GET_CHILD_BY_NAME, &args![box_tile, TOP_TEXT])
        .u32();
    if top != 0 {
        mark_tile(e, top, flag);
        for name in [BOTTOM_TEXT, LEFT_TEXT, RIGHT_TEXT] {
            let edge = e.call(TILE_GET_CHILD_BY_NAME, &args![box_tile, name]).u32();
            mark_tile(e, edge, flag);
        }
    }
    mark_tile(e, box_tile, flag);
}

/// `ListBox<T *>::SetTotalListHeight`: with a container, its visible height
/// (trait `0xfb0`, stored as a `float`) and the total list height decide the
/// number of scroll steps: 1, unless both are above 0.0 and the visible
/// height is below the total, then `00647b70(_ftol2(004e4430(unfiltered *
/// (total - visible) / total)) + 1, 2)` (the ratio stored as a `float`).
/// With both above 0.0 and more than one step, the container's
/// `"_scroll_delta"` becomes `(total - visible) / (steps - 1)`; with a scroll
/// bar its `"_number_of_items"` becomes the number of steps.
fn list_box_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return;
    }
    let visible = x87_store32(e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64());
    let mut steps = 1i32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    let total_bits = e.mem.u32(this.addr() + 0x20);
    if above(x87_load32(total_bits), zero) && above(x87_load32(visible), zero) {
        // `FLD total; FLD visible; FCOMPP; TEST AH,5; JP`: on only when the
        // visible height is below the total (ordered).
        let total_bits = e.mem.u32(this.addr() + 0x20);
        if x87_load32(visible) < x87_load32(total_bits) {
            let count = e.call(kind.num_unfiltered, &args![this]).u32();
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let difference = x87_sub(x87_load32(total_bits), x87_m32(visible));
            let product = x87_mul(f64::from(count), difference);
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let ratio = x87_store32(x87_div(product, x87_m32(total_bits)));
            let rounded = e.call(ROUND_FLOAT, &args![x87_quiet_bits(ratio)]).f64();
            let whole = e.call(FTOL2, &args![rounded]).i32();
            steps = e
                .call(INT_PAIR_00647B70, &args![whole.wrapping_add(1), 2i32])
                .i32();
        }
        if steps > 1 {
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let difference = x87_sub(x87_load32(total_bits), x87_m32(visible));
            let delta = x87_store32(x87_div(difference, f64::from(steps - 1)));
            let scroll_delta = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
            let container = e.get(this, ListBox::pContainerRect);
            e.call(
                TILE_SET_FLOAT,
                &args![container, scroll_delta, x87_quiet_bits(delta), 1u32],
            );
        }
    }
    if !e.get(this, ListBox::pScrollBar).is_null() {
        let number_of_items = e.call(TEXT_TO_TRAIT, &args![NUMBER_OF_ITEMS_TEXT]).u32();
        let scroll_bar = e.get(this, ListBox::pScrollBar);
        e.call(TILE_SET_INT, &args![scroll_bar, number_of_items, steps]);
    }
}

// Translated from 007a1c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a1c90 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<TESQuest *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a1c90(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, QUEST_LIST)
}

// Translated from 007a1d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a1d60 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<TESQuest *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a1d60(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, QUEST_LIST);
}

// Translated from 007a2070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_tes_quest_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, QUEST_LIST);
}

// Translated from 007a23e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_tes_quest_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, tile, QUEST_LIST)
}

// Translated from 007a6550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6550 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESQuest_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_tes_quest_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, QUEST_LIST)
}

// Translated from 007a6600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6600 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<TESQuest_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_tes_quest_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, QUEST_LIST);
}

// Translated from 007a6920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_tes_quest_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, QUEST_LIST);
}

// Translated from 007a27f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a27f0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<BGSNote *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a27f0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, NOTE_LIST)
}

// Translated from 007a28c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a28c0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<BGSNote *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a28c0(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, NOTE_LIST);
}

// Translated from 007a2bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_bgs_note_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, NOTE_LIST);
}

// Translated from 007a3170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_bgs_note_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, tile, NOTE_LIST)
}

// Translated from 007a6bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6bd0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSNote_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_bgs_note_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, NOTE_LIST)
}

// Translated from 007a6c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6c80 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<BGSNote_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_bgs_note_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, NOTE_LIST);
}

// Translated from 007a7080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_bgs_note_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, NOTE_LIST);
}

// Translated from 007a36e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a36e0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<TESObjectREFR *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a36e0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, REFERENCE_LIST)
}

// Translated from 007a37b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a37b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<TESObjectREFR *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a37b0(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, REFERENCE_LIST);
}

// Translated from 007a3ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_tes_object_refr_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, REFERENCE_LIST);
}

// Translated from 007a3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_tes_object_refr_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, tile, REFERENCE_LIST)
}

// Translated from 007a7330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a7330 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESObjectREFR_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_tes_object_refr_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, REFERENCE_LIST)
}

// Translated from 007a73e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a73e0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<TESObjectREFR_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_tes_object_refr_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, REFERENCE_LIST);
}

// Translated from 007a7700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_tes_object_refr_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, REFERENCE_LIST);
}

// Translated from 007a43a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a43a0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<BGSQuestObjective *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a43a0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, OBJECTIVE_LIST)
}

// Translated from 007a4470 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a4470 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<BGSQuestObjective *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a4470(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, OBJECTIVE_LIST);
}

// Translated from 007a4780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_bgs_quest_objective_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, OBJECTIVE_LIST);
}

// Translated from 007a4ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_bgs_quest_objective_highlight(
    e: &mut Engine,
    this: Ptr<ListBox>,
    tile: u32,
) -> bool {
    list_box_highlight(e, this, tile, OBJECTIVE_LIST)
}

// Translated from 007a79b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a79b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSQuestObjective_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_bgs_quest_objective_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, OBJECTIVE_LIST)
}

// Translated from 007a7a60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a7a60 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<BGSQuestObjective_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_bgs_quest_objective_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, OBJECTIVE_LIST);
}

// Translated from 007a7d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_bgs_quest_objective_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, OBJECTIVE_LIST);
}

// Translated from 007a50f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a50f0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<TESChallenge *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a50f0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, CHALLENGE_LIST)
}

// Translated from 007a51c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a51c0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<TESChallenge *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a51c0(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, CHALLENGE_LIST);
}

// Translated from 007a54d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_tes_challenge_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, CHALLENGE_LIST);
}

// Translated from 007a5a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_tes_challenge_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, tile, CHALLENGE_LIST)
}

// Translated from 007a81b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a81b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESChallenge_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_tes_challenge_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, CHALLENGE_LIST)
}

// Translated from 007a8260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a8260 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<TESChallenge_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_tes_challenge_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, CHALLENGE_LIST);
}

// Translated from 007a8580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_tes_challenge_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, CHALLENGE_LIST);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007a_1aa0, fn_007a1aa0(Ptr)),
        entry!(
            0x0072_d040,
            map_menu_attach_tile_by_id(Ptr<MapMenu>, i32, u32)
        ),
        entry!(0x0073_12e0, list_box_bgs_note_save_list_state(Ptr<ListBox>)),
        entry!(0x0079_6740, map_menu_map_menu(Ptr<MapMenu>) -> Ptr<MapMenu>),
        entry!(0x0079_6a40, map_menu_get_class(Ptr<MapMenu>) -> u32),
        entry!(
            0x0079_6a50,
            map_menu_scalar_deleting_destructor(Ptr<MapMenu>, u32) -> Ptr<MapMenu>
        ),
        entry!(0x0079_6a80, fn_00796a80(Ptr<MapMenu>)),
        entry!(0x0079_6b90, map_menu_create() -> u32),
        entry!(0x0079_6f40, fn_00796f40(Ptr, u8)),
        entry!(0x0079_6f90, fn_00796f90(Ptr<MapMenu>) -> bool),
        entry!(0x0079_6fd0, map_menu_do_click(Ptr<MapMenu>, i32, u32)),
        entry!(0x0079_8450, fn_00798450(Ptr, u32) -> Ptr),
        entry!(0x0079_a130, map_menu_do_idle(Ptr<MapMenu>)),
        entry!(0x0079_aba0, map_menu_update_menu()),
        entry!(0x0079_bc30, fn_0079bc30(Ptr) -> u32),
        entry!(0x0079_c090, fn_0079c090(Ptr<MapMenu>)),
        entry!(0x007a_1480, fn_007a1480() -> u32),
        entry!(0x007a_1490, map_menu_clear_map_memory()),
        entry!(0x007a_1670, map_menu_clear_world_map_texture()),
        entry!(0x007a_17a0, fn_007a17a0() -> f32),
        entry!(0x007a_17e0, fn_007a17e0() -> f32),
        entry!(0x007a_1880, map_menu_set_root_map_space(u32)),
        entry!(0x007a_1910, fn_007a1910(Ptr<ListBox>) -> u32),
        entry!(0x007a_1930, fn_007a1930(Ptr<ListBox>, u8)),
        entry!(0x007a_1950, fn_007a1950(Ptr<ListBox>, u8, u8)),
        entry!(
            0x007a_1a70,
            fn_007a1a70(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x007a_1c30, fn_007a1c30(Ptr<BSSimpleArray>, u8)),
        entry!(0x007a_62e0, fn_007a62e0(Ptr, u32, u32) -> Ptr),
        entry!(0x007a_63f0, fn_007a63f0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x007a_1c90, fn_007a1c90(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_1d60, fn_007a1d60(Ptr<ListBox>)),
        entry!(0x007a_2070, list_box_tes_quest_remove_all(Ptr<ListBox>)),
        entry!(
            0x007a_23e0,
            list_box_tes_quest_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_6550,
            list_box_tes_quest_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_6600,
            list_box_tes_quest_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_6920,
            list_box_tes_quest_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x007a_27f0, fn_007a27f0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_28c0, fn_007a28c0(Ptr<ListBox>)),
        entry!(0x007a_2bd0, list_box_bgs_note_remove_all(Ptr<ListBox>)),
        entry!(
            0x007a_3170,
            list_box_bgs_note_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_6bd0,
            list_box_bgs_note_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_6c80,
            list_box_bgs_note_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_7080,
            list_box_bgs_note_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x007a_36e0, fn_007a36e0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_37b0, fn_007a37b0(Ptr<ListBox>)),
        entry!(
            0x007a_3ac0,
            list_box_tes_object_refr_remove_all(Ptr<ListBox>)
        ),
        entry!(
            0x007a_3f90,
            list_box_tes_object_refr_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_7330,
            list_box_tes_object_refr_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_73e0,
            list_box_tes_object_refr_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_7700,
            list_box_tes_object_refr_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x007a_43a0, fn_007a43a0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_4470, fn_007a4470(Ptr<ListBox>)),
        entry!(
            0x007a_4780,
            list_box_bgs_quest_objective_remove_all(Ptr<ListBox>)
        ),
        entry!(
            0x007a_4ce0,
            list_box_bgs_quest_objective_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_79b0,
            list_box_bgs_quest_objective_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_7a60,
            list_box_bgs_quest_objective_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_7d80,
            list_box_bgs_quest_objective_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x007a_50f0, fn_007a50f0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_51c0, fn_007a51c0(Ptr<ListBox>)),
        entry!(0x007a_54d0, list_box_tes_challenge_remove_all(Ptr<ListBox>)),
        entry!(
            0x007a_5a60,
            list_box_tes_challenge_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_81b0,
            list_box_tes_challenge_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_8260,
            list_box_tes_challenge_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_8580,
            list_box_tes_challenge_set_total_list_height(Ptr<ListBox>)
        ),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_007a1aa0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x007a_1aa0, ARRAY_VTABLE_01075530, ARRAY_RELEASE_007A1C30);
    }
}

#[cfg(test)]
mod map_menu_tests {
    use super::*;

    /// An engine with the pages of the statics and constants the map menu
    /// code reads mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_a000,
            0x011d_e000,
            0x011d_8000,
            0x011d_1000,
            0x011d_2000,
            0x011d_3000,
            0x011d_4000,
            0x011d_5000,
            0x011c_3000,
            0x011f_4000,
            0x011f_6000,
            0x011a_0000,
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_e000,
            0x0102_3000,
            0x0103_0000,
            0x0107_0000,
            0x0107_4000,
        ] {
            e.map(page, 0x1000);
        }
        e
    }

    /// Doubles that do nothing and return 0.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// A double returning `value` in `EAX`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double returning `value` in `ST0`.
    fn returns_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn called(e: &Engine, addr: u32) -> bool {
        !calls_to(e, addr).is_empty()
    }

    /// A map menu object with `pTiles` filled with distinct tiles
    /// (`tile_base + index`).
    fn menu_with_tiles(e: &mut Engine, tile_base: u32) -> Ptr<MapMenu> {
        let menu: Ptr<MapMenu> = e.new_object();
        for index in 0..0x16 {
            e.mem
                .set_u32(menu.addr() + 0x28 + 4 * index, tile_base + index);
        }
        menu
    }

    /// A vtable at `addr` whose slots are `slots`, and an object using it.
    fn object_with_vtable(e: &mut Engine, addr: u32, slots: &[u32]) -> u32 {
        e.put_vtable(addr, slots);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, addr);
        object
    }

    #[test]
    fn attach_tile_by_id_stores_only_ids_in_range() {
        let mut e = engine();
        let menu: Ptr<MapMenu> = e.new_object();
        e.call(0x0072_d040, &args![menu, 3i32, 0x1234u32]);
        assert_eq!(e.mem.u32(menu.addr() + 0x28 + 12), 0x1234);
        e.call(0x0072_d040, &args![menu, -1i32, 0x5555u32]);
        e.call(0x0072_d040, &args![menu, 0x16i32, 0x5555u32]);
        assert_eq!(e.mem.u32(menu.addr() + 0x24), 0);
        assert_eq!(e.mem.u32(menu.addr() + 0x80), 0);
    }

    #[test]
    fn save_list_state_reads_the_index_and_scroll_value() {
        let mut e = engine();
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0xfac { 4.0 } else { 7.5 },
            ..Ret::default()
        });
        returns(&mut e, TEXT_TO_TRAIT, 0x77);
        let list: Ptr<ListBox> = e.new_object();
        e.call(0x0073_12e0, &args![list]);
        assert_eq!(e.mem.f32(list.addr() + 0x24), -1.0);
        assert_eq!(e.mem.f32(list.addr() + 0x28), -1.0);
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x9000));
        e.set(list, ListBox::pScrollBar, Ptr::new(0x9100));
        e.call_log = Some(vec![]);
        e.call(0x0073_12e0, &args![list]);
        assert_eq!(e.mem.f32(list.addr() + 0x24), 4.0);
        assert_eq!(e.mem.f32(list.addr() + 0x28), 7.5);
        assert_eq!(
            calls_to(&e, TILE_GET_FLOAT),
            vec![vec![0x9000, 0xfac], vec![0x9100, 0x77]]
        );
    }

    /// Doubles for every callee of the constructor.
    fn constructor_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                MENU_CONSTRUCT,
                SOUND_LIST_CONSTRUCT,
                LIST_CONSTRUCT,
                SELF,
                WAYPOINTS_CONSTRUCT,
                LIST_BOX_BASE_CONSTRUCT,
                LIST_BOX_REGISTER,
                ARRAY_CONSTRUCT_ELEMENTS_007A6310,
            ],
        );
        e.register(TEXT_TO_TRAIT, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        returns(e, AUDIO_INSTANCE, 0x5000);
        returns_float(e, AUDIO_GET_RADIO_VOLUME, 0.25);
    }

    #[test]
    fn constructor_builds_the_members_and_the_start_values() {
        let mut e = engine();
        constructor_doubles(&mut e);
        e.set_global(POINT_011F426C, 1.0f32);
        e.set_global(POINT_011F426C + 4, 2.0f32);
        e.set_global(POINT_011F426C + 8, 3.0f32);
        let menu: Ptr<MapMenu> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_6740, &args![menu]).u32(), menu.addr());
        let base = menu.addr();
        assert_eq!(e.mem.u32(base), MAP_MENU_VTABLE);
        assert_eq!(e.mem.u32(base + 0xa8), ARRAY_VTABLE_01075530);
        for (offset, vtable) in [
            (0x130u32, QUEST_LIST.vtable),
            (0x160, NOTE_LIST.vtable),
            (0x190, REFERENCE_LIST.vtable),
            (0x1c0, OBJECTIVE_LIST.vtable),
            (0x1f0, CHALLENGE_LIST.vtable),
        ] {
            assert_eq!(e.mem.u32(base + offset), vtable);
        }
        assert_eq!(
            calls_to(&e, LIST_CONSTRUCT)[..2],
            [vec![base + 0xd8], vec![base + 0xe0]]
        );
        assert_eq!(e.mem.u32(TRAIT_TITLE), TITLE_TEXT & 0xffff);
        assert_eq!(e.mem.u32(TRAIT_MAGNIFICATION), MAGNIFICATION_TEXT & 0xffff);
        assert_eq!(e.get(menu, MapMenu::iCurrentPage), 0x24);
        assert_eq!(e.get(menu, MapMenu::bShowingAllNotes), 1);
        assert_eq!(e.get(menu, MapMenu::bFogOfWarEnabled), 1);
        assert_eq!(e.get(menu, MapMenu::fOldPipboyRadioVolume), 0.25);
        assert_eq!(e.mem.f32(base + 0x100), 2.0);
        assert_eq!(calls_to(&e, AUDIO_GET_RADIO_VOLUME), vec![vec![0x5000]]);
    }

    #[test]
    fn get_class_is_the_map_menu_id() {
        let mut e = engine();
        assert_eq!(e.call(0x0079_6a40, &args![0u32]).u32(), 0x3ff);
    }

    /// Doubles for every callee of the destructor body.
    fn destructor_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                WAYPOINTS_DESTRUCT,
                LIST_DESTRUCT,
                SOUND_LIST_DESTRUCT,
                MENU_DESTRUCT,
                LIST_BOX_UNREGISTER,
                TEXT_TO_TRAIT,
                ADD_USER_TRAIT,
                ARRAY_RELEASE_007A1C30,
                FREE,
            ],
        );
    }

    #[test]
    fn destructor_body_clears_the_singleton_and_destroys_the_members() {
        let mut e = engine();
        destructor_doubles(&mut e);
        let menu: Ptr<MapMenu> = e.new_object();
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.call_log = Some(vec![]);
        e.call(0x0079_6a80, &args![menu]);
        let base = menu.addr();
        assert_eq!(e.mem.u32(MAP_MENU_SINGLETON), 0);
        assert_eq!(e.mem.u32(base), MAP_MENU_VTABLE);
        assert_eq!(calls_to(&e, WAYPOINTS_DESTRUCT), vec![vec![base + 0x220]]);
        assert_eq!(
            calls_to(&e, LIST_BOX_UNREGISTER),
            vec![
                vec![base + 0x1f0],
                vec![base + 0x1c0],
                vec![base + 0x190],
                vec![base + 0x160],
                vec![base + 0x130]
            ]
        );
        let lists = calls_to(&e, LIST_DESTRUCT);
        assert_eq!(
            lists[lists.len() - 2..],
            [vec![base + 0xe0], vec![base + 0xd8]]
        );
        assert_eq!(
            calls_to(&e, ARRAY_RELEASE_007A1C30),
            vec![vec![base + 0xa8, 1]]
        );
        assert_eq!(calls_to(&e, MENU_DESTRUCT), vec![vec![base]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_on_flag_one() {
        let mut e = engine();
        destructor_doubles(&mut e);
        let menu: Ptr<MapMenu> = e.new_object();
        for (flags, frees) in [(0u32, false), (1, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0079_6a50, &args![menu, flags]).u32(), menu.addr());
            assert_eq!(called(&e, FREE), frees);
            assert!(called(&e, MENU_DESTRUCT));
        }
    }

    /// Doubles for `Create`: the menu read from the file is `menu` (its
    /// vtable slot 0x34 returns `class`).
    fn create_doubles(e: &mut Engine, menu: u32, class: u32) {
        let vtable = 0x0107_4d00;
        e.put_vtable(vtable, &[0x0050_0000; 0x10]);
        e.mem.set_u32(menu, vtable);
        e.mem.set_u32(vtable + 0x34, 0x0050_0034);
        returns(e, 0x0050_0034, class);
        quiet(
            e,
            &[
                0x0050_0000,
                MENU_REMOVE_FROM_XML_ARCHIVE,
                CALENDAR_MARK_STALE,
                MENU_SET_MENU_TILE,
                PRINT_WARNING,
                TILE_SET_FLOAT,
                TUTORIAL_SHOW_MESSAGE,
                OBJECT_FIELD_00403DF0,
                INITIALIZE_TABLINE,
                LIST_BOX_INITIALIZE,
                TILE_OBJECT_0056C7F0,
                TILE_SET_STRING,
                MENU_PREP_FOR_VISIBILITY,
                ADD_USER_TRAIT,
                TILE_SET_INT,
                TILE_DELETE_CHILDREN,
                TILE_REMOVE_FADE_CONTROL,
                LIST_REMOVE_ALL,
                WAYPOINTS_CLEAR,
            ],
        );
        returns(e, INTERFACE_MANAGER, 0x6000);
        returns(e, TILE_GET_MENU_BY_CLASS, 0);
        returns(e, READER_00586150, 0x6100);
        returns(e, TILE_READ_FILE, 0x6200);
        returns(e, TILE_GET_MENU, menu);
        returns(e, WORD_AT_4, 0x6300);
        returns(e, MAP_MENU_ID_00720DF0, 0x3ff);
        returns_float(e, TILE_GET_FLOAT, 102.0);
        returns_float(e, TILE_GET_MAXIMUM_DEPTH, 9.0);
        e.set_global(CREATE_VALUE_A, 6006.0f64);
        e.set_global(CREATE_VALUE_B, 102.0f64);
    }

    #[test]
    fn create_rejects_a_menu_that_is_not_the_map_menu() {
        let mut e = engine();
        let menu = e.mem.alloc(0x230);
        create_doubles(&mut e, menu, 0x3f0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_6b90, &args![]).u32(), 0);
        assert_eq!(e.mem.u32(MAP_MENU_SINGLETON), menu);
        // The menu is deleted through vtable slot 0.
        assert_eq!(calls_to(&e, 0x0050_0000), vec![vec![menu, 1]]);
        assert!(!called(&e, MENU_SET_MENU_TILE));
    }

    #[test]
    fn create_warns_when_a_tile_is_missing() {
        let mut e = engine();
        let menu = e.mem.alloc(0x230);
        create_doubles(&mut e, menu, 0x3ff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_6b90, &args![]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![vec![CREATION_FAILED_TEXT]]
        );
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![menu, 0x6200, 0]]
        );
    }

    #[test]
    fn create_sets_the_menu_up() {
        let mut e = engine();
        e.map(0x7000, 0x1000);
        let menu = menu_with_tiles(&mut e, 0x7000).addr();
        create_doubles(&mut e, menu, 0x3ff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_6b90, &args![]).u32(), 0x6200);
        // Trait 0xfa7 is 102: the depth trait is set.
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6200, 0xfad, 9.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, TUTORIAL_SHOW_MESSAGE),
            vec![vec![0x6000 + 0x4d4, 0x1a, 0x3ff, 0x200]]
        );
        let lists = calls_to(&e, LIST_BOX_INITIALIZE);
        assert_eq!(lists[0], vec![menu + 0x130, 0x7007, LIST_MARKER_TEMPLATE]);
        assert_eq!(lists[4], vec![menu + 0x190, 0x700a, LIST_MARKER_TEMPLATE]);
        // Tiles 5 and 3 get bit 0x8000 of their word +0x30.
        assert_ne!(e.mem.u32(0x7005 + 0x30) & 0x8000, 0);
        assert_ne!(e.mem.u32(0x7003 + 0x30) & 0x8000, 0);
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 1]]);
    }

    #[test]
    fn flag_8000_is_set_and_cleared() {
        let mut e = engine();
        let tile = e.mem.alloc(0x40);
        e.mem.set_u32(tile + 0x30, 0x0001_0001);
        e.call(0x0079_6f40, &args![tile, 1u8]);
        assert_eq!(e.mem.u32(tile + 0x30), 0x0001_8001);
        e.call(0x0079_6f40, &args![tile, 0u8]);
        assert_eq!(e.mem.u32(tile + 0x30), 0x0001_0001);
    }

    #[test]
    fn all_tiles_present_check() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        assert!(e.call(0x0079_6f90, &args![menu]).bool());
        e.mem.set_u32(menu.addr() + 0x28 + 4 * 0x15, 0);
        assert!(!e.call(0x0079_6f90, &args![menu]).bool());
    }

    #[test]
    fn do_click_on_the_markers_clicks_the_highlighted_marker() {
        let mut e = engine();
        let vtable = 0x0107_4d00;
        e.put_vtable(vtable, &[0x0050_0000; 0x10]);
        quiet(&mut e, &[0x0050_0000]);
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(menu.addr(), vtable);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 2i32, 0u32]);
        assert!(!called(&e, 0x0050_0000));
        e.set(menu, MapMenu::pHighlightedMapMarker, 0x9999);
        e.call(0x0079_6fd0, &args![menu, 4i32, 0u32]);
        assert_eq!(
            calls_to(&e, 0x0050_0000),
            vec![vec![menu.addr(), 0x1a, 0x9999]]
        );
        // Ids outside the table do nothing.
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 3i32, 0u32]);
        e.call(0x0079_6fd0, &args![menu, 0x25i32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn do_click_activates_the_highlighted_quest() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(PLAYER_011DEA3C, 0x4444);
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x5150);
        returns(&mut e, QUEST_FLAG_0059E400, 0);
        quiet(
            &mut e,
            &[SET_ACTIVE_QUEST, PLAY_MENU_SOUND, UPDATE_CURRENT_PAGE],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x17i32, 0u32]);
        assert_eq!(calls_to(&e, SET_ACTIVE_QUEST), vec![vec![0x4444, 0x5150]]);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![3]]);
        // A quest for which `0059e400` is set stays.
        returns(&mut e, QUEST_FLAG_0059E400, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x17i32, 0u32]);
        assert!(!called(&e, SET_ACTIVE_QUEST));
    }

    #[test]
    fn do_click_switches_the_page() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(TRAIT_CURRENT_TAB, 0x55);
        e.set_global(HALF_FLOAT, 0.5f32);
        e.set_global(FULL_ALPHA_FLOAT, 255.0f32);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 3.0);
        returns(&mut e, OBJECT_VALUE_ADDRESS_00403E20, setting);
        returns(&mut e, INTERFACE_MANAGER, 0x6000);
        returns(&mut e, WORD_AT_174, 0x6100);
        returns(&mut e, WORD_AT_28, 0x6200);
        returns(&mut e, FLAG_00709D40, 0);
        returns(&mut e, GET_PIPBOY, 0);
        returns(&mut e, PIPBOY_FLAG_00974D90, 0);
        quiet(
            &mut e,
            &[
                TILE_SET_INT,
                TRIGGER_BURST,
                TILE_SET_FLOAT,
                TAB_KNOB_0079C340,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x23i32, 0u32]);
        assert_eq!(e.get(menu, MapMenu::iCurrentPage), 0x23);
        let ints = calls_to(&e, TILE_SET_INT);
        assert_eq!(ints[0], vec![0x7011, 0x55, 3]);
        assert_eq!(ints[1], vec![0x700d, 0xfa3, 0]);
        assert_eq!(ints[2], vec![0x700c, 0xfaf, 1]);
        assert_eq!(
            calls_to(&e, TRIGGER_BURST),
            vec![vec![
                0x6100,
                3.0f32.to_bits(),
                3.0f32.to_bits(),
                0.5f32.to_bits()
            ]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6200, 0xfa9, 255.0f32.to_bits(), 1]]
        );
        assert!(!called(&e, TAB_KNOB_0079C340));
        // The world map page leaves the pip-boy alpha alone.
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x20i32, 0u32]);
        assert!(!called(&e, TILE_SET_FLOAT));
        assert_eq!(calls_to(&e, TAB_KNOB_0079C340), vec![vec![1]]);
    }

    #[test]
    fn do_click_toggles_the_notes_filter_or_steps_the_challenge_type() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::iCurrentPage, 0x23);
        e.set(menu, MapMenu::bShowingAllNotes, 1);
        returns(&mut e, TEXT_TO_TRAIT, 0x66);
        quiet(
            &mut e,
            &[
                NOTE_LIST_FILTER,
                CHALLENGE_LIST_FILTER,
                TILE_SET_INT,
                TILE_SET_STRING,
            ],
        );
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x8888);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0xci32, 0u32]);
        assert_eq!(e.get(menu, MapMenu::bShowingAllNotes), 0);
        assert_eq!(
            calls_to(&e, NOTE_LIST_FILTER),
            vec![vec![menu.addr() + 0x160, NOTE_FILTER_CALLBACK]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT)[1], vec![0x700c, 0x66, 0]);
        // Challenge mode: the type wraps from 2 to 0.
        e.mem.set_u8(CHALLENGE_MODE_011DA36D, 1);
        e.mem.set_u8(CHALLENGE_TYPE_011DA36E, 2);
        e.mem.set_u32(CHALLENGE_TYPE_NAMES_011A03E4, 0x1230);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0xci32, 0u32]);
        assert_eq!(e.mem.u8(CHALLENGE_TYPE_011DA36E), 0);
        assert_eq!(calls_to(&e, OBJECT_FIELD_00403DF0), vec![vec![0x1230]]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x7015, 0xfc4, 0x8888, 1]]
        );
    }

    #[test]
    fn do_click_sets_the_player_marker_on_the_local_map() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set(menu, MapMenu::pMapTile, 0x7100);
        e.mem.set_u32(PLAYER_011DEA3C, 0x4444);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, INTERFACE_MANAGER, 0x6000);
        returns_float(&mut e, CURSOR_Y_007983C0, 20.0);
        returns_float(&mut e, CURSOR_X_007697F0, 10.0);
        returns_float(&mut e, TILE_GETTER_00A013D0, 1.0);
        returns_float(&mut e, TILE_GETTER_00A01440, 2.0);
        returns_float(&mut e, TILE_GET_FLOAT, 0.5);
        e.register(POINT_SET_00416870, |e, a| {
            for (i, word) in a[1..4].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, *word);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(POINT_SCALE_0053D280, |e, a| {
            let scale = f32::from_bits(a[2]);
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) * scale;
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(MAP_TO_WORLD_COORD, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[1] + 4 * i, a[2 + i as usize]);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        returns(&mut e, CALCULATE_PLAYER_MARKER_SPACE, 0x3300);
        returns(&mut e, AUDIO_INSTANCE, 0x5000);
        e.register(AUDIO_SOUND_BY_NAME, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        returns(&mut e, PLAYER_FLAG_00798400, 0);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x8800);
        quiet(
            &mut e,
            &[
                ADJUST_MAP_MARKER_COORD,
                TILE_SET_INT,
                SOUND_PLAY,
                SOUND_HANDLE_DESTRUCT,
                SHOW_MESSAGE_BOX,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0xci32, 0u32]);
        let base = menu.addr();
        // Cursor (10, 20) minus the tile's (1, 2), scaled by 0.5.
        assert_eq!(e.mem.f32(base + 0xfc), 4.5);
        assert_eq!(e.mem.f32(base + 0x100), 9.0);
        assert_eq!(e.get(menu, MapMenu::PlayerMarkerSpace), 0x3300);
        assert_eq!(
            calls_to(&e, ADJUST_MAP_MARKER_COORD),
            vec![vec![0x3300, base + 0xfc, 1]]
        );
        assert_eq!(
            calls_to(&e, AUDIO_SOUND_BY_NAME)[0][2..],
            [MARKER_ADDED_SOUND, 0x121]
        );
        let boxes = calls_to(&e, SHOW_MESSAGE_BOX);
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].len(), 11);
        assert_eq!(boxes[0][3..6], [SET_PLAYER_MARKER_CALLBACK, 3, 0x17]);
    }

    #[test]
    fn do_click_opens_an_audio_note() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        let base = menu.addr();
        e.mem.set_u32(TRAIT_SELECTED, 0x41);
        e.mem.set_u32(TRAIT_TEXT_ALPHA, 0x42);
        e.set_global(HALF_ALPHA_FLOAT, 128.0f32);
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x5100);
        returns(&mut e, NOTE_TYPE, 0);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, LIST_BOX_GET_TILE_FOR, 0x7300);
        returns(&mut e, NOTE_FLAG_00798420, 0);
        returns(&mut e, NOTE_SOUND_005E9100, 0x5200);
        returns(&mut e, WORD_AT_C, 0x99);
        returns(&mut e, AUDIO_INSTANCE, 0x5000);
        e.register(AUDIO_SOUND_BY_ID, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(AUDIO_SOUND_BY_NAME, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_DESTRUCT,
                TILE_SET_INT,
                LOAD_NOTE,
                TILE_SET_FLOAT,
                NOTE_SET_READ_005E9300,
                SOUND_LIST_ADD_TAIL,
                SOUND_PLAY,
                DIALOG_FLAG_00762150,
                AUDIO_ENTER_DIALOGUE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x18i32, 0x7400u32]);
        assert_eq!(calls_to(&e, LOAD_NOTE), vec![vec![base, 0x5100]]);
        assert_eq!(e.get(menu, MapMenu::pSelectedNote), 0x5100);
        assert_eq!(
            calls_to(&e, LIST_BOX_GET_TILE_FOR),
            vec![vec![base + 0x160, base + 0x90]]
        );
        let ints = calls_to(&e, TILE_SET_INT);
        assert_eq!(ints[0], vec![0x7300, 0x41, 0]);
        assert_eq!(ints[1], vec![0x7400, 0x41, 1]);
        assert_eq!(ints[2], vec![0x700d, 0x1004, 1]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x7400, 0x42, 128.0f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&e, AUDIO_SOUND_BY_ID)[0][2..], [0x99, 0x101]);
        assert_eq!(calls_to(&e, SOUND_LIST_ADD_TAIL)[0][0], base + 0x98);
        assert_eq!(e.get(menu, MapMenu::bSoundsLoading), 1);
        assert_eq!(
            calls_to(&e, AUDIO_SOUND_BY_NAME)[0][2..],
            [HOLOTAPE_START_SOUND, 0x121]
        );
        assert_eq!(calls_to(&e, AUDIO_ENTER_DIALOGUE), vec![vec![0x5000]]);
        // In challenge mode the click does nothing.
        e.mem.set_u8(CHALLENGE_MODE_011DA36D, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x18i32, 0x7400u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn do_click_runs_a_conversation_note() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        let base = menu.addr();
        e.mem.set_u32(PLAYER_011DEA3C, 0x4444);
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x5100);
        returns(&mut e, NOTE_TYPE, 3);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, LIST_BOX_GET_TILE_FOR, 0);
        returns(&mut e, NOTE_FLAG_00798420, 1);
        returns(&mut e, NOTE_TOPIC_005E9240, 0x5300);
        returns(&mut e, NOTE_GET_SPEAKER, 0x5400);
        let speaker = object_with_vtable(&mut e, 0x0107_9000, &[0x0050_0000; 8]);
        e.mem.set_u32(0x0107_9010, 0x0050_0010);
        let blocks = [speaker, 0x6600];
        let mut next = 0;
        e.register_double(ALLOCATE, move |_, _| {
            let block = blocks[next];
            next += 1;
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(CHARACTER_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CONVERSATION_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returns(&mut e, GET_CURRENT_RESPONSE, 0x6700);
        returns(&mut e, FIRST_RESPONSE, 1);
        let mut more = 1;
        e.register_double(NEXT_RESPONSE, move |_, _| {
            let value = more;
            more = 0;
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returns(&mut e, WORD_AT_C, 0x6800);
        e.register(PLUS_10, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        returns(&mut e, STRING_TEXT, 0x6900);
        returns(&mut e, AUDIO_INSTANCE, 0x5000);
        e.register(AUDIO_SOUND_BY_FILENAME, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(AUDIO_SOUND_BY_NAME, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.set_global(NOTE_VOLUME_FLOAT, 0.9f32);
        quiet(
            &mut e,
            &[
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_DESTRUCT,
                TILE_SET_INT,
                LOAD_NOTE,
                FORM_SET_TEMPORARY,
                REFR_SET_OBJECT_REFERENCE,
                CONVERSATION_FIRST_ITEM,
                SUBTITLES_ADD_007A1AC0,
                TOPIC_INFO_RUN_RESULT,
                SOUND_HANDLE_ASSIGN,
                SOUND_SET_VOLUME,
                SOUND_LIST_ADD_TAIL,
                CONVERSATION_DESTRUCT,
                FREE,
                0x0050_0010,
                SOUND_PLAY,
                DIALOG_FLAG_00762150,
                AUDIO_ENTER_DIALOGUE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x18i32, 0x7400u32]);
        assert_eq!(calls_to(&e, CHARACTER_CONSTRUCT), vec![vec![speaker, 0]]);
        assert_eq!(
            calls_to(&e, REFR_SET_OBJECT_REFERENCE),
            vec![vec![speaker, 0x5400]]
        );
        assert_eq!(
            calls_to(&e, CONVERSATION_CONSTRUCT),
            vec![vec![0x6600, speaker, 0x4444, 0x5300]]
        );
        // Two responses, each with its result script before and after.
        assert_eq!(
            calls_to(&e, TOPIC_INFO_RUN_RESULT),
            vec![
                vec![0x6800, 0, speaker],
                vec![0x6800, 1, speaker],
                vec![0x6800, 0, speaker],
                vec![0x6800, 1, speaker]
            ]
        );
        assert_eq!(
            calls_to(&e, AUDIO_SOUND_BY_FILENAME)[0][2..],
            [0x6900, 0x105, 0]
        );
        assert_eq!(calls_to(&e, SOUND_SET_VOLUME)[0][1], 0.9f32.to_bits());
        assert_eq!(
            calls_to(&e, SUBTITLES_ADD_007A1AC0)[0],
            vec![base + 0xa8, 0x6700]
        );
        assert_eq!(calls_to(&e, CONVERSATION_DESTRUCT), vec![vec![0x6600]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x6600]]);
        assert_eq!(calls_to(&e, 0x0050_0010), vec![vec![speaker, 1]]);
        assert!(called(&e, AUDIO_ENTER_DIALOGUE));
    }

    #[test]
    fn do_click_switches_the_radio_station() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        let base = menu.addr();
        e.put_vtable(0x0107_9000, &[0x0050_0000, 0x0050_0004]);
        e.mem.set_u32(base + 0x190, 0x0107_9000);
        returns(&mut e, 0x0050_0004, 0x7700);
        e.mem.set_u32(TRAIT_TEXT_ALPHA, 0x42);
        e.mem.set_u32(TRAIT_SELECTED, 0x41);
        e.set_global(FULL_ALPHA_DOUBLE, 255.0f64);
        returns_float(&mut e, TILE_GET_FLOAT, 255.0);
        returns(&mut e, TILE_IS_TRUE, 0);
        returns(&mut e, FLAG_00798440, 1);
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x5500);
        quiet(
            &mut e,
            &[
                TILE_SET_INT,
                LIST_BOX_FILTER_007314C0,
                RADIO_ENABLE,
                RADIO_SET_STATION_00832240,
                UPDATE_CURRENT_PAGE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x19i32, 0u32]);
        assert_eq!(calls_to(&e, RADIO_ENABLE), vec![vec![0], vec![1]]);
        assert_eq!(
            calls_to(&e, RADIO_SET_STATION_00832240),
            vec![vec![0x5500, 0]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_FILTER_007314C0),
            vec![vec![base + 0x190, RADIO_FILTER_CALLBACK, 0, 0x7fff_ffff]]
        );
        let ints = calls_to(&e, TILE_SET_INT);
        assert_eq!(ints.last().unwrap(), &vec![0x7700, 0x41, 1]);
        // A station that is not fully visible is not switched.
        returns_float(&mut e, TILE_GET_FLOAT, 128.0);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x19i32, 0u32]);
        assert!(!called(&e, RADIO_ENABLE));
    }

    #[test]
    fn do_click_offers_fast_travel_to_the_clicked_marker() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        let base = menu.addr();
        e.put_vtable(0x0107_9000, &[0x0050_0000; 8]);
        e.mem.set_u32(base, 0x0107_9000);
        e.mem.set_u32(0x0107_9014, 0x0050_0014);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        // Two markers in the list at +0xd8.
        let second = e.mem.alloc(8);
        e.mem.set_u32(base + 0xd8, 0x3100);
        e.mem.set_u32(base + 0xdc, second);
        e.mem.set_u32(second, 0x3200);
        returns(&mut e, FLAG_004B71D0, 1);
        returns(&mut e, TILE_IS_TRUE, 1);
        returns(&mut e, PLAYER_FLAG_0093D660, 1);
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(WORD_AT_4, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        returns(&mut e, ADD_USER_TRAIT, 0x61);
        returns_float(&mut e, TILE_GET_FLOAT, 1.0);
        returns(&mut e, REFR_GET_MAP_MARKER_DATA, 0x3400);
        returns(&mut e, MARKER_GET_TRAVEL_LOC, 1);
        returns(&mut e, MARKER_NAME_00438EB0, 0x3500);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x3600);
        quiet(
            &mut e,
            &[0x0050_0014, BUFFER_FORMAT, TILE_SET_INT, SHOW_MESSAGE_BOX],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x1ai32, 0x7400u32]);
        // `"_MarkerIndex"` 1.0 picks the second marker.
        assert_eq!(e.get(menu, MapMenu::pFastTravelMarker), 0x3200);
        assert_eq!(calls_to(&e, 0x0050_0014), vec![vec![base, 0x1a, 0x7400]]);
        let format = calls_to(&e, BUFFER_FORMAT);
        assert_eq!(format[0][1..], [0x100, FAST_TRAVEL_FORMAT, 0x3600, 0x3500]);
        let boxes = calls_to(&e, SHOW_MESSAGE_BOX);
        assert_eq!(boxes[0][0], format[0][0]);
        assert_eq!(boxes[0][3], FAST_TRAVEL_CALLBACK);
        // No travel location: the marker is dropped.
        returns(&mut e, MARKER_GET_TRAVEL_LOC, 0);
        e.call(0x0079_6fd0, &args![menu, 0x1ai32, 0x7400u32]);
        assert_eq!(e.get(menu, MapMenu::pFastTravelMarker), 0);
    }

    #[test]
    fn do_click_shows_the_companion_message() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set_global(TWO_FLOAT, 2.0f32);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, TILE_IS_TRUE, 0);
        returns(&mut e, ADD_USER_TRAIT, 0x61);
        returns(&mut e, TILE_GET_STRING, 0x3700);
        returns(&mut e, STRICMP, 1);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x3600);
        quiet(&mut e, &[SHOW_MESSAGE_007052F0]);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x1ai32, 0x7400u32]);
        assert_eq!(calls_to(&e, STRICMP), vec![vec![0x3700, COMPANION_TEXT]]);
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE_007052F0),
            vec![vec![0x3600, 2, 0, 0, 2.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn conversation_deleting_destructor_frees_on_flag_one() {
        let mut e = engine();
        quiet(&mut e, &[CONVERSATION_DESTRUCT, FREE]);
        for (flags, frees) in [(0u32, false), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0079_8450, &args![0x4000u32, flags]).u32(), 0x4000);
            assert_eq!(calls_to(&e, CONVERSATION_DESTRUCT), vec![vec![0x4000]]);
            assert_eq!(called(&e, FREE), frees);
        }
    }

    #[test]
    fn do_idle_pauses_and_resumes_the_note_sound() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::pCurrentSound, 0x5600);
        e.set(menu, MapMenu::iSoundPlayStartTime, 100);
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returns(&mut e, SOUND_IS_PLAYING, 1);
        returns(&mut e, IS_TOP_MENU_ID, 1);
        returns(&mut e, TIMER_NOW, 1000);
        returns(&mut e, GET_PIPBOY, 0);
        returns(&mut e, PIPBOY_FLAG_00974D90, 0);
        quiet(&mut e, &[SOUND_PAUSE, SOUND_PLAY]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, SOUND_PAUSE), vec![vec![0x5600]]);
        assert_eq!(e.mem.u8(SOUND_PAUSED_011DA36F), 1);
        assert_eq!(e.mem.u32(PAUSE_TIME_011DA40C), 1000);
        // Menu 0x3f5 gone: the sound plays on, the pause added to the start.
        returns(&mut e, IS_TOP_MENU_ID, 0);
        returns(&mut e, TIMER_NOW, 1500);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, SOUND_PLAY), vec![vec![0x5600, 0]]);
        assert_eq!(e.mem.u8(SOUND_PAUSED_011DA36F), 0);
        assert_eq!(e.get(menu, MapMenu::iSoundPlayStartTime), 600);
    }

    #[test]
    fn do_idle_fades_the_pipboy_in_under_another_menu() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set(menu, MapMenu::bSoundsLoading, 1);
        e.set_global(FULL_ALPHA_FLOAT, 255.0f32);
        returns(&mut e, IS_TOP_MENU_ID, 0);
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, INTERFACE_MANAGER, 0x6000);
        returns(&mut e, WORD_AT_28, 0x6200);
        quiet(&mut e, &[TILE_SET_FLOAT, UPDATE_PLAYING_SOUNDS]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6200, 0xfa9, 255.0f32.to_bits(), 1]]
        );
        assert_eq!(e.mem.u8(FADED_011DA408), 1);
        assert_eq!(calls_to(&e, UPDATE_PLAYING_SOUNDS), vec![vec![menu.addr()]]);
        // Once faded it is not done again.
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert!(!called(&e, TILE_SET_FLOAT));
    }

    #[test]
    fn do_idle_shows_the_cursor_position_on_the_local_map() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(MAP_WIDTH_DOUBLE, 850.0f64);
        e.set_global(MAP_HEIGHT_DOUBLE, 500.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(FADED_011DA408, 1u8);
        e.register(IS_TOP_MENU_ID, |_, a| Ret {
            eax: u32::from(a[0] == 0x3ff),
            ..Ret::default()
        });
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, INTERFACE_MANAGER, 0x6000);
        returns(&mut e, WORD_AT_28, 0x6200);
        returns_float(&mut e, CURSOR_X_007697F0, 110.0);
        returns_float(&mut e, CURSOR_Y_007983C0, 70.0);
        returns_float(&mut e, TILE_GETTER_00A013D0, 10.0);
        returns_float(&mut e, TILE_GETTER_00A01440, 20.0);
        returns_float(&mut e, TILE_GET_FLOAT, 40.0);
        returns(&mut e, OBJECT_00877720, 0x6400);
        returns(&mut e, OBJECT_CHECK_00A23A50, 0);
        quiet(&mut e, &[TILE_SET_FLOAT, SHOW_POSITION_00799DC0]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        let floats = calls_to(&e, TILE_SET_FLOAT);
        // Inside the map: the pip-boy alpha 0, then x 100 - 20 and y 50 - 20.
        assert_eq!(floats[0], vec![0x6200, 0xfa9, 0, 1]);
        assert_eq!(floats[1], vec![0x7006, 0xfa1, 80.0f32.to_bits(), 1]);
        assert_eq!(floats[2], vec![0x7006, 0xfa2, 30.0f32.to_bits(), 1]);
        assert_eq!(
            calls_to(&e, SHOW_POSITION_00799DC0),
            vec![vec![menu.addr()]]
        );
        assert_eq!(e.mem.u8(FADED_011DA408), 0);
        // Off the map (x above 850): alpha 255.
        e.set_global(FULL_ALPHA_FLOAT, 255.0f32);
        returns_float(&mut e, CURSOR_X_007697F0, 900.0);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT)[0],
            vec![0x6200, 0xfa9, 255.0f32.to_bits(), 1]
        );
    }

    #[test]
    fn do_idle_marks_the_selected_note_read_after_its_delay() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::iCurrentPage, 0x23);
        e.set(menu, MapMenu::uiNoteEnterStartTime, 1000);
        e.set(menu, MapMenu::pSelectedNote, 0x5100);
        e.mem.set_u32(TRAIT_TEXT_ALPHA, 0x42);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(HALF_ALPHA_FLOAT, 128.0f32);
        let delay = e.mem.alloc(4);
        e.mem.set_f32(delay, 2.0);
        returns(&mut e, OBJECT_VALUE_ADDRESS_00403E20, delay);
        e.register(IS_TOP_MENU_ID, |_, a| Ret {
            eax: u32::from(a[0] == 0x3ff),
            ..Ret::default()
        });
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, TIMER_NOW, 2500);
        returns(&mut e, LIST_BOX_GET_TILE_FOR, 0x7300);
        quiet(&mut e, &[TILE_SET_FLOAT, NOTE_SET_READ_005E9300]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        // 1500 ms of 2000: nothing yet.
        assert!(!called(&e, NOTE_SET_READ_005E9300));
        returns(&mut e, TIMER_NOW, 3000);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, NOTE_SET_READ_005E9300), vec![vec![0x5100, 1]]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x7300, 0x42, 128.0f32.to_bits(), 1]]
        );
        assert_eq!(e.get(menu, MapMenu::uiNoteEnterStartTime), 0);
    }

    #[test]
    fn do_idle_updates_the_radio_page() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::iCurrentPage, 0x24);
        e.register(IS_TOP_MENU_ID, |_, a| Ret {
            eax: u32::from(a[0] == 0x3ff),
            ..Ret::default()
        });
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns(&mut e, FLAG_00798440, 0);
        quiet(&mut e, &[TILE_SET_INT, PAGE_0079C0C0]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x7010, 0xfa3, 0]]);
        returns(&mut e, FLAG_00798440, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, PAGE_0079C0C0), vec![vec![menu.addr()]]);
    }

    #[test]
    fn update_menu_formats_the_date_and_time() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.mem.set_u32(PLAYER_011DEA3C, 0x4444);
        e.set_global(SIXTY_DOUBLE, 60.0f64);
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns_float(&mut e, CALENDAR_GET_HOUR, 13.5);
        e.register(FTOL2, |_, a| Ret {
            eax: f64::take(a, &mut 0) as i32 as u32,
            ..Ret::default()
        });
        e.register(CALENDAR_GET_DATE_STRING, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(STRING_TEXT, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x3800);
        returns(&mut e, COMPARE_00408B20, 0);
        quiet(
            &mut e,
            &[
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
                STRING_FORMAT,
                TILE_SET_STRING,
                PLAYER_MESSAGE_00578870,
                UPDATE_CURRENT_PAGE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_aba0, &args![]);
        let format = calls_to(&e, STRING_FORMAT);
        assert_eq!(format.len(), 1);
        let text = format[0][0];
        assert_eq!(format[0][1..], [DATE_TIME_FORMAT, text + 8 + 0x100, 13, 30]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![
                vec![0x7001, 0xfc4, text + 0x100, 1],
                vec![0x7000, 0xfc4, text + 0x100, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, PLAYER_MESSAGE_00578870),
            vec![vec![0x4444, text]]
        );
        assert_eq!(
            calls_to(&e, STRING_DESTRUCT),
            vec![vec![text + 8], vec![text]]
        );
        assert_eq!(calls_to(&e, UPDATE_CURRENT_PAGE), vec![vec![menu.addr()]]);
    }

    #[test]
    fn update_menu_sets_the_titles_for_the_language() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.mem.set_u32(TRAIT_TITLE, 0x51);
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns_float(&mut e, CALENDAR_GET_HOUR, 1.0);
        returns(&mut e, FTOL2, 1);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x3800);
        returns(&mut e, COMPARE_00408B20, 1);
        e.register(TEXT_TO_TRAIT, |_, a| Ret {
            eax: a[0] & 0xff,
            ..Ret::default()
        });
        returns(&mut e, TILE_GET_VALUE, 0x3900);
        returns(&mut e, TILE_GET_CHILD_BY_ID, 0x3a00);
        returns(&mut e, WORD_AT_4, 0x3b00);
        returns(&mut e, TILE_GET_CHILD_BY_NAME, 0x3c00);
        quiet(
            &mut e,
            &[
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
                STRING_FORMAT,
                STRING_TEXT,
                CALENDAR_GET_DATE_STRING,
                TILE_SET_STRING,
                PLAYER_MESSAGE_00578870,
                UPDATE_CURRENT_PAGE,
                VALUE_CALCULATE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_aba0, &args![]);
        let strings = calls_to(&e, TILE_SET_STRING);
        assert_eq!(strings.len(), 2 + 10 + 2);
        assert_eq!(strings[2], vec![0x700b, TITLE_N_TEXT[0] & 0xff, 0x3800, 1]);
        assert_eq!(strings[11], vec![0x700c, TITLE_N_TEXT[4] & 0xff, 0x3800, 1]);
        assert_eq!(strings[12], vec![0x3a00, 0xfc4, 0x3800, 1]);
        assert_eq!(strings[13], vec![0x3c00, 0xfc4, 0x3800, 1]);
        assert_eq!(
            calls_to(&e, TILE_GET_VALUE),
            vec![vec![0x700b, 0x51], vec![0x700c, 0x51]]
        );
        assert_eq!(calls_to(&e, TILE_GET_CHILD_BY_ID), vec![vec![0x7011, 0x24]]);
        assert_eq!(
            calls_to(&e, TILE_GET_CHILD_BY_NAME),
            vec![vec![0x3b00, HEADLINE_TITLE_TEXT]]
        );
        // Without a map menu nothing happens.
        e.mem.set_u32(MAP_MENU_SINGLETON, 0);
        e.call_log = Some(vec![]);
        e.call(0x0079_aba0, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn offset_6bc() {
        let mut e = engine();
        assert_eq!(e.call(0x0079_bc30, &args![0x1000u32]).u32(), 0x16bc);
    }

    #[test]
    fn waveform_reset_clears_the_time_and_tile_16() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::fWaveformAnimationStartTime, 5.0);
        quiet(&mut e, &[TILE_SET_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0079_c090, &args![menu]);
        assert_eq!(e.get(menu, MapMenu::fWaveformAnimationStartTime), 0.0);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x7010, 0xfa3, 0]]);
    }

    #[test]
    fn word_011dea14() {
        let mut e = engine();
        e.mem.set_u32(VALUE_011DEA14, 0x1357);
        assert_eq!(e.call(0x007a_1480, &args![]).u32(), 0x1357);
    }

    /// Doubles for the shader-property chain of `ClearMapMemory` and
    /// `ClearWorldMapTexture`.
    fn texture_doubles(e: &mut Engine, object: u32) {
        returns(e, TILE_OBJECT_0056C7F0, object);
        returns(e, OBJECT_NODE_0045BC00, 0x3d00);
        returns(e, PROPERTY_TYPE_00438220, 3);
        returns(e, AV_OBJECT_GET_PROPERTY, 0x3e00);
        returns(e, PROPERTY_CHECK_009C8640, 1);
        quiet(e, &[SET_TILE_TEXTURE, TILE_SET_STRING]);
    }

    #[test]
    fn clear_map_memory_releases_the_map_tiles() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::pMapTile, 0x7100);
        e.set(menu, MapMenu::uiNextLocalMapGrid, 7);
        let object = object_with_vtable(&mut e, 0x0107_9000, &[0x0050_0000; 0x40]);
        e.mem.set_u32(0x0107_90e8, 0x0050_00e8);
        texture_doubles(&mut e, object);
        returns(&mut e, OBJECT_COUNT_0043B480, 3);
        e.register(OBJECT_ITEM_0043B4A0, |_, a| Ret {
            eax: if a[1] == 1 { 0 } else { 0x100 + a[1] },
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x0050_00e8,
                TILE_DELETE_CHILDREN,
                TILE_SET_INT,
                TILE_REMOVE_FADE_CONTROL,
                LIST_REMOVE_ALL,
                WAYPOINTS_CLEAR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x007a_1490, &args![]);
        let base = menu.addr();
        assert_eq!(
            calls_to(&e, 0x0050_00e8),
            vec![vec![object, 0x100], vec![object, 0x102]]
        );
        assert_eq!(
            calls_to(&e, TILE_DELETE_CHILDREN),
            vec![vec![0x7002], vec![0x7004]]
        );
        assert_eq!(
            calls_to(&e, TILE_REMOVE_FADE_CONTROL),
            vec![vec![0x7003, 0xff5], vec![0x7005, 0xff5]]
        );
        assert_eq!(
            calls_to(&e, LIST_REMOVE_ALL),
            vec![vec![base + 0xe0], vec![base + 0xd8]]
        );
        assert_eq!(calls_to(&e, WAYPOINTS_CLEAR), vec![vec![base + 0x220, 1]]);
        assert_eq!(e.get(menu, MapMenu::pMapTile), 0);
        assert_eq!(e.get(menu, MapMenu::uiNextLocalMapGrid), 0);
        assert_eq!(calls_to(&e, SET_TILE_TEXTURE), vec![vec![0x3e00, 0]]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x7010, 0xfcc, EMPTY_TEXT, 1]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT).last().unwrap(),
            &vec![0x7010, 0xfa3, 0]
        );
        // Without the tile object nothing else runs.
        returns(&mut e, TILE_OBJECT_0056C7F0, 0);
        e.call_log = Some(vec![]);
        e.call(0x007a_1490, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn clear_world_map_texture_skips_the_open_local_map() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        texture_doubles(&mut e, 0x3f00);
        returns(&mut e, IS_TOP_MENU_ID, 1);
        e.call_log = Some(vec![]);
        e.call(0x007a_1670, &args![]);
        assert!(!called(&e, SET_TILE_TEXTURE));
        returns(&mut e, IS_TOP_MENU_ID, 0);
        e.call_log = Some(vec![]);
        e.call(0x007a_1670, &args![]);
        assert_eq!(calls_to(&e, TILE_OBJECT_0056C7F0)[0], vec![0x7004]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x7004, 0xfcc, EMPTY_TEXT, 1]]
        );
        assert_eq!(e.mem.u8(TEXTURE_CLEARED_011DA36C), 1);
    }

    #[test]
    fn magnification_of_tiles_2_and_4() {
        let mut e = engine();
        assert_eq!(e.call(0x007a_17a0, &args![]).f32(), 1.0);
        assert_eq!(e.call(0x007a_17e0, &args![]).f32(), 1.0);
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.mem.set_u32(TRAIT_MAGNIFICATION, 0x45);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: f64::from(a[0] - 0x7000),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_17a0, &args![]).f32(), 2.0);
        assert_eq!(e.call(0x007a_17e0, &args![]).f32(), 4.0);
        assert_eq!(
            calls_to(&e, TILE_GET_FLOAT),
            vec![vec![0x7002, 0x45], vec![0x7004, 0x45]]
        );
    }

    #[test]
    fn set_root_map_space_walks_to_the_top_world() {
        let mut e = engine();
        let menu: Ptr<MapMenu> = e.new_object();
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.register(WORLD_GET_PARENT, |_, a| Ret {
            eax: match a[0] {
                0x100 => 0x200,
                0x200 => 0x300,
                _ => 0,
            },
            ..Ret::default()
        });
        e.call(0x007a_1880, &args![0x100u32]);
        assert_eq!(e.get(menu, MapMenu::pRootMapSpace), 0x300);
        assert_eq!(e.get(menu, MapMenu::bRootSpaceChanged), 1);
        e.call(0x007a_1880, &args![0x200u32]);
        assert_eq!(e.get(menu, MapMenu::bRootSpaceChanged), 0);
        // Without a map menu it does nothing.
        e.mem.set_u32(MAP_MENU_SINGLETON, 0);
        e.call(0x007a_1880, &args![0x100u32]);
    }

    #[test]
    fn highlighted_item_of_a_list_box() {
        let mut e = engine();
        let list: Ptr<ListBox> = e.new_object();
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x9000));
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_1910, &args![list]).u32(), 0x77);
        assert_eq!(
            calls_to(&e, LIST_BOX_ITEM_OF_007E1110),
            vec![vec![list.addr(), 0x9000]]
        );
    }

    #[test]
    fn list_box_flags_bits() {
        let mut e = engine();
        let list: Ptr<ListBox> = e.new_object();
        e.call(0x007a_1950, &args![list, 3u8, 1u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 8);
        e.call(0x007a_1930, &args![list, 1u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 9);
        e.call(0x007a_1930, &args![list, 0u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 8);
        // The shift count is masked to 5 bits: 35 is bit 3.
        e.call(0x007a_1950, &args![list, 35u8, 0u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 0);
        // Bits above 15 fall off the 16-bit flags.
        e.call(0x007a_1950, &args![list, 20u8, 1u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 0);
    }

    #[test]
    fn string_array_constructor_and_release() {
        let mut e = engine();
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr() + 4, 0x1111);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_1a70, &args![array]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr()), ARRAY_VTABLE_01075530);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        quiet(
            &mut e,
            &[ARRAY_DESTROY_ELEMENTS_007A63B0, ARRAY_FREE_006A8500],
        );
        // Without a buffer the release does nothing.
        e.set(array, BSSimpleArray::iSize, 5);
        e.call(0x007a_1c30, &args![array, 1u8]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 5);
        e.set(array, BSSimpleArray::pBuffer, 0x2000);
        e.set(array, BSSimpleArray::iReservedSize, 8);
        e.call_log = Some(vec![]);
        e.call(0x007a_1c30, &args![array, 0u8]);
        assert_eq!(
            calls_to(&e, ARRAY_DESTROY_ELEMENTS_007A63B0),
            vec![vec![array.addr(), 0x2000, 5]]
        );
        assert!(!called(&e, ARRAY_FREE_006A8500));
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x2000);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        e.call(0x007a_1c30, &args![array, 1u8]);
        assert_eq!(calls_to(&e, ARRAY_FREE_006A8500), vec![vec![array.addr()]]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn list_box_item_construction() {
        let mut e = engine();
        let item = e.mem.alloc(0xc);
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, 0x4321);
        e.mem.set_u8(item + 8, 1);
        assert_eq!(
            e.call(0x007a_62e0, &args![item, 0x7000u32, data]).u32(),
            item
        );
        assert_eq!(e.mem.u32(item), 0x7000);
        assert_eq!(e.mem.u32(item + 4), 0x4321);
        assert_eq!(e.mem.u8(item + 8), 0);
    }

    #[test]
    fn string_array_constructor_body_reserves_and_constructs() {
        let mut e = engine();
        let array = object_with_vtable(&mut e, 0x0107_9000, &[0x0050_0000, 0x0050_0004]);
        e.register(0x0050_0004, |_, a| Ret {
            eax: 0x8000 + a[1],
            ..Ret::default()
        });
        quiet(&mut e, &[ARRAY_CONSTRUCT_ELEMENTS_007A6310]);
        e.call_log = Some(vec![]);
        e.call(0x007a_63f0, &args![array, 2u32, 5u32]);
        assert_eq!(e.mem.u32(array + 4), 0x8005);
        assert_eq!(e.mem.u32(array + 8), 5);
        assert_eq!(e.mem.u32(array + 0xc), 5);
        assert_eq!(
            calls_to(&e, ARRAY_CONSTRUCT_ELEMENTS_007A6310),
            vec![vec![array, 0x8005, 5]]
        );
        e.call_log = Some(vec![]);
        e.call(0x007a_63f0, &args![array, 6u32, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0x8006);
        assert_eq!(e.mem.u32(array + 8), 0);
        assert!(!called(&e, ARRAY_CONSTRUCT_ELEMENTS_007A6310));
        e.call(0x007a_63f0, &args![array, 0u32, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
    }

    /// The five instances' functions: constructor, destructor, `RemoveAll`,
    /// `Highlight`, `IsEnabled`, `UpdateHighlightBoxPos`,
    /// `SetTotalListHeight`.
    const INSTANCES: [(ListBoxKind, [u32; 7]); 5] = [
        (
            QUEST_LIST,
            [
                0x007a_1c90,
                0x007a_1d60,
                0x007a_2070,
                0x007a_23e0,
                0x007a_6550,
                0x007a_6600,
                0x007a_6920,
            ],
        ),
        (
            NOTE_LIST,
            [
                0x007a_27f0,
                0x007a_28c0,
                0x007a_2bd0,
                0x007a_3170,
                0x007a_6bd0,
                0x007a_6c80,
                0x007a_7080,
            ],
        ),
        (
            REFERENCE_LIST,
            [
                0x007a_36e0,
                0x007a_37b0,
                0x007a_3ac0,
                0x007a_3f90,
                0x007a_7330,
                0x007a_73e0,
                0x007a_7700,
            ],
        ),
        (
            OBJECTIVE_LIST,
            [
                0x007a_43a0,
                0x007a_4470,
                0x007a_4780,
                0x007a_4ce0,
                0x007a_79b0,
                0x007a_7a60,
                0x007a_7d80,
            ],
        ),
        (
            CHALLENGE_LIST,
            [
                0x007a_50f0,
                0x007a_51c0,
                0x007a_54d0,
                0x007a_5a60,
                0x007a_81b0,
                0x007a_8260,
                0x007a_8580,
            ],
        ),
    ];

    #[test]
    fn list_box_constructors_set_the_vtable_and_register() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            quiet(
                &mut e,
                &[LIST_BOX_BASE_CONSTRUCT, LIST_CONSTRUCT, LIST_BOX_REGISTER],
            );
            let list: Ptr<ListBox> = e.new_object();
            e.mem.set_u32(list.addr() + 0x14, 0x55);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(functions[0], &args![list]).u32(), list.addr());
            assert_eq!(e.mem.u32(list.addr()), kind.vtable);
            assert_eq!(e.mem.u32(list.addr() + 0x14), 0);
            assert_eq!(e.get(list, ListBox::usFlags), 1);
            assert_eq!(calls_to(&e, LIST_CONSTRUCT), vec![vec![list.addr() + 4]]);
            assert_eq!(calls_to(&e, LIST_BOX_REGISTER), vec![vec![list.addr()]]);
        }
    }

    /// Doubles for `RemoveAll` and what it runs.
    fn remove_all_doubles(e: &mut Engine) {
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(WORD_AT_4, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        returns(e, LIST_BOX_HAS_FLAG, 1);
        returns(e, TEXT_TO_TRAIT, 0x66);
        returns_float(e, TILE_GET_FLOAT, 0.0);
        quiet(
            e,
            &[
                NODE_SET_NEXT,
                NODE_DELETE,
                ITEM_DATA_DELETE,
                FREE,
                TILE_SET_FLOAT,
                TILE_SET_INT,
                ADD_USER_TRAIT,
                0x0050_0000,
                LIST_BOX_UNREGISTER,
                LIST_DESTRUCT,
            ],
        );
    }

    /// A list box with a container and two items (tiles with the vtable at
    /// `0107_9000`, data words `0xd1` and `0xd2`): the first in the
    /// embedded node, the second in a node after it.
    fn list_with_two_items(e: &mut Engine) -> (Ptr<ListBox>, [u32; 2], u32) {
        e.put_vtable(0x0107_9000, &[0x0050_0000]);
        let list: Ptr<ListBox> = e.new_object();
        e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
        e.set(list, ListBox::usNextIndex, 2);
        e.set(list, ListBox::fTotalListHeight, 40.0);
        let mut items = [0u32; 2];
        for (i, item) in items.iter_mut().enumerate() {
            let tile = e.mem.alloc(8);
            e.mem.set_u32(tile, 0x0107_9000);
            *item = e.mem.alloc(0xc);
            e.mem.set_u32(*item, tile);
            e.mem.set_u32(*item + 4, 0xd1 + i as u32);
        }
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, items[1]);
        e.mem.set_u32(list.addr() + 4, items[0]);
        e.mem.set_u32(list.addr() + 8, node);
        (list, items, node)
    }

    #[test]
    fn list_box_remove_all_deletes_every_item() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            remove_all_doubles(&mut e);
            let (list, items, node) = list_with_two_items(&mut e);
            e.call_log = Some(vec![]);
            e.call(functions[2], &args![list]);
            assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![node, 1]]);
            assert_eq!(
                calls_to(&e, ITEM_DATA_DELETE),
                vec![vec![0xd2, 1], vec![0xd1, 1]]
            );
            assert_eq!(calls_to(&e, FREE), vec![vec![items[1]], vec![items[0]]]);
            assert_eq!(calls_to(&e, 0x0050_0000).len(), 2);
            assert_eq!(e.mem.u32(list.addr() + 4), 0);
            assert_eq!(e.get(list, ListBox::usNextIndex), 0);
            assert_eq!(calls_to(&e, TILE_SET_FLOAT), vec![vec![0x9100, 0x66, 0, 1]]);
            // `UpdateHighlightBoxPos` without a highlighted item.
            let y = e.mem.u32(kind.highlight_y.cache);
            assert_eq!(calls_to(&e, TILE_SET_INT)[0], vec![0x9100, y, 0xffff_ffff]);
            // With the keep-tiles byte set the tiles stay.
            let (list, _, _) = list_with_two_items(&mut e);
            e.mem.set_u8(KEEP_ITEM_TILES, 1);
            e.call_log = Some(vec![]);
            e.call(functions[2], &args![list]);
            assert!(!called(&e, 0x0050_0000));
            e.mem.set_u8(KEEP_ITEM_TILES, 0);
        }
    }

    #[test]
    fn list_box_destructors_empty_the_list() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            remove_all_doubles(&mut e);
            let (list, _, _) = list_with_two_items(&mut e);
            e.call_log = Some(vec![]);
            e.call(functions[1], &args![list]);
            assert_eq!(e.mem.u32(list.addr()), kind.vtable);
            assert_eq!(calls_to(&e, FREE).len(), 2);
            assert_eq!(calls_to(&e, LIST_BOX_UNREGISTER), vec![vec![list.addr()]]);
            assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![list.addr() + 4]]);
        }
    }

    #[test]
    fn list_box_is_enabled_looks_the_trait_up_once() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            returns(&mut e, ADD_USER_TRAIT, 0x71);
            returns(&mut e, TILE_IS_TRUE, 1);
            let list: Ptr<ListBox> = e.new_object();
            e.call_log = Some(vec![]);
            assert!(!e.call(functions[4], &args![list]).bool());
            assert_eq!(e.mem.u32(kind.enabled.cache), 0x71);
            assert_eq!(e.mem.u32(kind.enabled.guard) & 1, 1);
            e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
            assert!(e.call(functions[4], &args![list]).bool());
            assert_eq!(
                calls_to(&e, ADD_USER_TRAIT),
                vec![vec![ENABLED_TEXT, 0xffff_ffff]]
            );
            assert_eq!(calls_to(&e, TILE_IS_TRUE), vec![vec![0x9100, 0x71]]);
        }
    }

    #[test]
    fn list_box_highlight_needs_an_enabled_item() {
        for (_, functions) in INSTANCES {
            let mut e = engine();
            let list: Ptr<ListBox> = e.new_object();
            e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
            returns(&mut e, ADD_USER_TRAIT, 0x71);
            returns(&mut e, TILE_IS_TRUE, 1);
            returns(&mut e, LIST_BOX_BELONGS_TO, 0);
            returns(&mut e, TILE_GET_VALUE, 0x3900);
            returns(&mut e, TILE_FLAG_GETTER_0071AA80, 1);
            returns(&mut e, TILE_FIND_BY_NAME, 0);
            quiet(
                &mut e,
                &[VALUE_ADD_ACTION_TILE, VALUE_CALCULATE, TILE_SET_INT],
            );
            assert!(!e.call(functions[3], &args![list, 0x7400u32]).bool());
            returns(&mut e, LIST_BOX_BELONGS_TO, 1);
            assert!(e.call(functions[3], &args![list, 0x7400u32]).bool());
            assert_eq!(e.get(list, ListBox::pHighlightedItem).addr(), 0x7400);
            // A null tile clears the highlight.
            assert!(e.call(functions[3], &args![list, 0u32]).bool());
            assert!(e.get(list, ListBox::pHighlightedItem).is_null());
        }
    }

    #[test]
    fn list_box_update_highlight_box_pos_marks_the_box() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            let list: Ptr<ListBox> = e.new_object();
            e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
            e.set(list, ListBox::pHighlightedItem, Ptr::new(0x7400));
            e.register(ADD_USER_TRAIT, |_, a| Ret {
                eax: a[0] & 0xff,
                ..Ret::default()
            });
            e.register(TILE_GET_VALUE, |_, a| Ret {
                eax: 0x3900 + a[1],
                ..Ret::default()
            });
            returns(&mut e, TILE_FLAG_GETTER_0071AA80, 1);
            returns(&mut e, TILE_FIND_BY_NAME, 0x3a00);
            e.register(TILE_GET_CHILD_BY_NAME, |_, a| Ret {
                eax: a[1] & 0xfff,
                ..Ret::default()
            });
            quiet(
                &mut e,
                &[
                    VALUE_ADD_ACTION_TILE,
                    VALUE_CALCULATE,
                    TILE_FLAG_SETTER_0071AA60,
                    TILE_ADD_NEEDS_UPDATE,
                ],
            );
            e.call_log = Some(vec![]);
            e.call(functions[5], &args![list]);
            let y = HIGHLIGHT_Y_TEXT & 0xff;
            let height = SELECTED_HEIGHT_TEXT & 0xff;
            assert_eq!(e.mem.u32(kind.highlight_y.cache), y);
            assert_eq!(e.mem.u32(kind.selected_height.cache), height);
            assert_eq!(e.mem.u32(kind.highlight_y.guard) & 3, 3);
            assert_eq!(
                calls_to(&e, VALUE_ADD_ACTION_TILE),
                vec![
                    vec![0x3900 + y, 0x7d0, 0x7400, 0xfa2],
                    vec![0x3900 + height, 0x7d0, 0x7400, 0xfb0]
                ]
            );
            let marked: Vec<u32> = calls_to(&e, TILE_FLAG_SETTER_0071AA60)
                .iter()
                .map(|c| c[0])
                .collect();
            assert_eq!(
                marked,
                vec![
                    TOP_TEXT & 0xfff,
                    BOTTOM_TEXT & 0xfff,
                    LEFT_TEXT & 0xfff,
                    RIGHT_TEXT & 0xfff,
                    0x3a00
                ]
            );
            assert!(calls_to(&e, TILE_FLAG_SETTER_0071AA60)
                .iter()
                .all(|c| c[1] == 1));
        }
    }

    #[test]
    fn list_box_set_total_list_height_computes_the_steps() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            e.set_global(ZERO_DOUBLE, 0.0f64);
            let list: Ptr<ListBox> = e.new_object();
            e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
            e.set(list, ListBox::pScrollBar, Ptr::new(0x9200));
            e.set(list, ListBox::fTotalListHeight, 300.0);
            returns_float(&mut e, TILE_GET_FLOAT, 100.0);
            returns(&mut e, kind.num_unfiltered, 6);
            e.register(ROUND_FLOAT, |_, a| Ret {
                st0: f64::from(f32::from_bits(a[0])),
                ..Ret::default()
            });
            e.register(FTOL2, |_, a| Ret {
                eax: f64::take(a, &mut 0) as i32 as u32,
                ..Ret::default()
            });
            e.register(INT_PAIR_00647B70, |_, a| Ret {
                eax: a[0].max(a[1]),
                ..Ret::default()
            });
            e.register(TEXT_TO_TRAIT, |_, a| Ret {
                eax: a[0] & 0xff,
                ..Ret::default()
            });
            quiet(&mut e, &[TILE_SET_FLOAT, TILE_SET_INT]);
            e.call_log = Some(vec![]);
            e.call(functions[6], &args![list]);
            // 6 * (300 - 100) / 300 = 4: five steps of (300 - 100) / 4.
            assert_eq!(calls_to(&e, INT_PAIR_00647B70), vec![vec![5, 2]]);
            assert_eq!(
                calls_to(&e, TILE_SET_FLOAT),
                vec![vec![0x9100, SCROLL_DELTA_TEXT & 0xff, 50.0f32.to_bits(), 1]]
            );
            assert_eq!(
                calls_to(&e, TILE_SET_INT),
                vec![vec![0x9200, NUMBER_OF_ITEMS_TEXT & 0xff, 5]]
            );
            // A list that fits: one step, no scroll delta.
            e.set(list, ListBox::fTotalListHeight, 50.0);
            e.call_log = Some(vec![]);
            e.call(functions[6], &args![list]);
            assert!(!called(&e, TILE_SET_FLOAT));
            assert_eq!(
                calls_to(&e, TILE_SET_INT),
                vec![vec![0x9200, NUMBER_OF_ITEMS_TEXT & 0xff, 1]]
            );
        }
    }
}
