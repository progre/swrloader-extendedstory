#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use std::mem::transmute;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::d3d9::IDirect3DDevice9;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::BYTE;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::LPDWORD;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::ntdef::LONG;
use winapi::shared::ntdef::LPCSTR;

// #ifndef SWR_H_INCLUDED
// #define SWR_H_INCLUDED

// // ---------------- ここからテンプレ ----------------

// DWORD書き換え
pub unsafe fn TamperDword(addr: usize, target: usize) -> usize {
    let old = *(addr as *const usize);
    *(addr as *mut usize) = target;
    old
}

// // DWORD加算
// __inline
// DWORD TamperDwordAdd(DWORD addr, DWORD delta)
// {
// 	DWORD old = *(DWORD*)addr;
// 	*(DWORD*)addr += delta;
// 	return old;
// }

// NEAR JMPオペランド書き換え
pub unsafe fn TamperNearJmpOpr(addr: usize, target: usize) -> usize {
    let old = (*((addr + 1) as *const i32) + (addr as i32 + 5)) as usize;
    *((addr + 1) as *mut i32) = ((target as i64) - (addr + 5) as i64) as i32;
    old
}

// // NEAR JMP書き換え
// __inline
// void TamperNearJmp(DWORD addr, DWORD target)
// {
// 	*(PBYTE)(addr + 0) = 0xE9;
// 	TamperNearJmpOpr(addr, target);
// }

// // NEAR CALL書き換え
// __inline
// void TamperNearCall(DWORD addr, DWORD target)
// {
// 	*(PBYTE)(addr + 0) = 0xE8;
// 	TamperNearJmpOpr(addr, target);
// }

// フォントディスクリプタ
#[repr(C, packed(4))]
pub struct SWRFONTDESC {
    pub FaceName: [u8; 0x100],
    pub R1: BYTE,
    pub R2: BYTE,
    pub G1: BYTE,
    pub G2: BYTE,
    pub B1: BYTE,
    pub B2: BYTE,
    pub Height: LONG,
    pub Weight: LONG,
    pub Italic: BYTE,
    pub Shadow: BYTE,
    pub UseOffset: BYTE,
    pub BufferSize: DWORD,
    pub OffsetX: DWORD,
    pub OffsetY: DWORD,
    pub CharSpaceX: DWORD,
    pub CharSpaceY: DWORD,
}

// // コンパクト文字列
// struct SWRSTR {
// 	union {
// 		char  str[16];
// 		char* pstr;
// 	};
// 	size_t length;

// 	operator char *() {
// 		return length > 15 ? pstr : str;
// 	}
// };

// // エフェクトマネージャインターフェース
// struct IEffectManager {
// 	virtual void* Destruct(int dynamic) = 0;
// 	virtual void LoadPattern(LPCSTR fileName, int) = 0;
// 	virtual void ClearPattern() = 0;
// 	virtual void AppendRegion(int arg_0, float arg_4, float arg_8, char arg_c, char arg_10, int arg_14) = 0;
// 	virtual void ClearRegion() = 0;
// };

// // よくわからないもの
// struct UnknownF {
// 	void *Unknown[3];
// 	float Aaxis;
// 	float Baxis;
// };

// // static_assert
// template<bool value>
// struct static_assert { typedef struct assert_failed Type; };
// template<>
// struct static_assert<true> { typedef void* Type; };

// thiscall
#[macro_export]
macro_rules! Ccall {
    ($addr: expr, $type: ty) => {{
        let func: $type = transmute($addr);
        func
    }};
}

// フォントオブジェクトメソッド
pub unsafe fn SWRFont_Create(p: LPVOID) {
    Ccall!(0x410DA0, extern "thiscall" fn(this: LPVOID))(p);
}
pub unsafe fn SWRFont_Destruct(p: LPVOID) {
    Ccall!(0x410E30, extern "thiscall" fn(this: LPVOID))(p);
}
pub unsafe fn SWRFont_SetIndirect(p: LPVOID, pdesc: LPVOID) {
    Ccall!(0x410F10, extern "thiscall" fn(this: LPVOID, LPVOID))(p, pdesc);
}

// テクスチャマネージャメソッド
// #define CTextureManager_LoadTexture(p, ret, path, unk1, unk2) \
// 	Ccall(p,0x404EC0,int*,(int*, LPCSTR, void*, void*))(ret, path, unk1, unk2)
pub unsafe fn CTextureManager_CreateTextTexture(
    p: LPVOID,
    ret: LPDWORD,
    str: LPCSTR,
    pdesc: LPVOID,
    width: DWORD,
    height: DWORD,
) -> LPDWORD {
    Ccall!(
        0x404F30,
        extern "thiscall" fn(this: LPVOID, LPDWORD, LPCSTR, LPVOID, DWORD, DWORD) -> LPDWORD
    )(p, ret, str, pdesc, width, height)
}
pub unsafe fn CTextureManager_Remove(p: LPVOID, id: DWORD) -> LPVOID {
    Ccall!(
        0x404FA0,
        extern "thiscall" fn(this: LPVOID, DWORD) -> LPVOID
    )(p, id)
}
pub unsafe fn CTextureManager_SetTexture(p: LPVOID, id: DWORD, stage: DWORD) {
    Ccall!(0x405020, extern "thiscall" fn(this: LPVOID, DWORD, DWORD))(p, id, stage)
}
// #define CTextureManager_GetSize(p, w, h) \
// 	Ccall(p,0x405090,void,(int*, int*))(w, h)
// // テクスチャマネージャメソッド(ハンドルマネージャからの継承)
// #define CTextureManager_Get(p, id) \
// 	((IDirect3DTexture9**)CHandleManager_Get(p, id))
// #define CTextureManager_Allocate(p, id) \
// 	((IDirect3DTexture9**)CHandleManager_Allocate(p, id))
// #define CTextureManager_Deallocate(p, id) \
// 	CHandleManager_Deallocate((void *)p, id)

// // ハンドルマネージャメソッド
// #define CHandleManager_Get(t, p, id) \
// 	Ccall(p,0x417010,t*,(int))(id)
//  __declspec(naked) void**
// Thunk_CHandleManager_Allocate(void *p, int *ret) {
// 	// thiscallなのに落ちると思ったらediを使っていたでござる　の巻
// 	__asm push edi
// 	__asm mov edi, [esp+8]
// 	__asm push [esp+12]
// 	__asm mov eax, 0x402680
// 	__asm call eax
// 	__asm pop edi
// 	__asm retn
// }
// #define CHandleManager_Allocate(p, ret) \
// 	Thunk_CHandleManager_Allocate(p, ret)
//  __declspec(naked) void
// Thunk_CHandleManager_Deallocate(void *p, int id) {
// 	 // thiscall(中略)eaxを使っていたでござる　の巻
// 	__asm mov eax, [esp+4]
// 	__asm push [esp+8]
// 	__asm mov ecx, 0x4027F0
// 	__asm call ecx
// 	__asm retn
// }
// #define CHandleManager_Deallocate(p, id) \
// 	Thunk_CHandleManager_Deallocate(p, id)

// レンダラメソッド
pub unsafe fn CRenderer_Begin(p: LPVOID) -> BOOL {
    Ccall!(0x401000, extern "thiscall" fn(this: LPVOID) -> BOOL)(p)
}
pub unsafe fn CRenderer_End(p: LPVOID) {
    Ccall!(0x401040, extern "thiscall" fn(this: LPVOID))(p)
}

// // セレクトエフェクトマネージャメソッド
// #define CSelectEffectManager_Create_Address 0x420CE0
// #define CSelectEffectManager_Free_Address   0x4221F0
// #define CSelectEffectManager_Create(p) \
// 	Ccall(p, CSelectEffectManager_Create_Address, void, ())()

// // インプットマネージャメソッド
// #define CInputManager_ReadReplay(p, name) \
// 	Ccall(p, 0x42B6C0, bool,(char *))(name)

// // ベクタオブジェクト
// #define Vector_Create \
// 	((void (__stdcall *)(void *, size_t, size_t, int, int))0x67BA4C)
// #define Vector_Destruct \
// 	((void (__stdcall *)(void *, size_t, size_t, int))0x67B9E9)

// // システムキーワンショット
// #define CheckKeyOneshot  \
// 	((bool(*)(int, int, int, int))0x4397F0)

// // パレットロード
// #define LoadPackagePalette(pflag, name, pal, bpp) \
// 	Ccall(pflag, 0x408F10, void, (LPCSTR, void *, int))(name, pal, bpp)

// // キャラクタ略称取得
// #define GetCharacterAbbr \
// 	((LPCSTR (__cdecl *)(int id))0x43ABE0)

// // バトルモード設定
// #define SetBattleMode \
// 	((void (__cdecl *)(int comm, int sub))0x43A560)

// // 角度cos
// #define DegreeCosine \
//     ((float (__cdecl *)(int deg))0x406680)

// テクスチャマネージャ
// CHandleManager<IDirect3DTexture *>
pub const g_textureMgr: LPVOID = 0x6ECD40 as LPVOID;
// Direct3Dデバイス
// IDirect3DDevice9*
pub unsafe fn g_pd3dDev() -> &'static IDirect3DDevice9 {
    &**(0x6ED250 as *const *const IDirect3DDevice9)
}
// レンダラ
// CRenderer
pub const g_renderer: LPVOID = 0x6E3DBC as LPVOID;
// // ネットワークオブジェクト
// // CNetworkServer/CNetworkClient
// #define g_pnetObject (*(char**)0x6E62FC)
// // プロファイル名
// // char *
// #define g_pprofP1 ((char*)(g_pnetObject + 0x04))
// #define g_pprofP2 ((char*)(g_pnetObject + 0x24))
// // UDPネットワークオブジェクト
// // CNetworkBase
// #define g_pnetUdp    (g_pnetObject + 0x3B4)
// // ピア情報
// // vector<SWRClientInfo> ?
// #define g_psvClients (g_pnetUdp + 0xFC)
// // サーバアドレス
// // in_addr
// #define g_ptoAddr    (g_pnetUdp + 0x2C)
// // バトルマネージャ
// // CBattleManager *
// #define g_pbattleMgr (*(void **)0x6E6244)
// // インフォマネージャ
// // CInfoManager *
// #define g_pinfoMgr   (*(void **)0x6E6248)
// // モード
// // int
// #define g_commMode   (*(DWORD*)0x6E62EC)
// #define g_subMode    (*(DWORD*)0x6E62E4)
// #define g_menuMode   (*(DWORD*)0x6D094C)
// // シーンID
// // DWORD
// #define g_sceneIdNew (*(DWORD*)0x6ECE78)
// #define g_sceneId    (*(DWORD*)0x6ECE7C)
// // コンバートデータ利用フラグ
// // bool
// #define g_useCVxData (*(bool*)0x6ECE80)
// // パレットオブジェクト
// // void *
// #define g_paletter   (*(void **)0x6E3DF8)
// // インプットマネージャ
// // CInputManager ?
// #define g_inputMgr   ((void *)0x6E6370)
// // インプットマネージャクラスタ
// // CInputManagerCluster
// #define g_inputMgrs  ((void *)0x6E7520)
// // キャラクタID
// // int
// #define g_leftCharID (*(int*)0x6E6FF0)
// #define g_rightCharID (*(int*)0x6E7010)
// // argc/argv
// #define __argc       (*(int*)0x6E7988)
// #define __argv       (*(char***)0x6E798C)

// 仮想関数テーブル
pub const vtbl_CLogo: usize = 0x6AC6FC;
pub const vtbl_Opening: usize = 0x6AC798;
pub const vtbl_CLoading: usize = 0x6AC6AC;
pub const vtbl_CTitle: usize = 0x6ACCF4;
pub const vtbl_CSelect: usize = 0x6ACBCC;
pub const vtbl_CSelectScenario: usize = 0x6ACC20;
pub const vtbl_CBattle: usize = 0x6AC470;
pub const vtbl_Ending: usize = 0x6AC5E8;
pub const vtbl_CSelectSV: usize = 0x6AC4AC;
pub const vtbl_CLoadingSV: usize = 0x6AC4CC;
pub const vtbl_CBattleSV: usize = 0x6AC4E8;
pub const vtbl_CSelectCL: usize = 0x6AC504;
pub const vtbl_CLoadingCL: usize = 0x6AC524;
pub const vtbl_CBattleCL: usize = 0x6AC540;
pub const vtbl_CLoadingWatch: usize = 0x6ACE68;
pub const vtbl_CBattleWatch: usize = 0x6AC55C;
pub const vtbl_CBattleManager: usize = 0x6AD50C;

// クラス構築関数caller
pub const CLogo_Creater: usize = 0x41D827;
pub const Opening_Creater: usize = 0x41D861;
pub const CLoading_Creater: usize = 0x41D89B;
pub const CTitle_Creater: usize = 0x41D8D5;
pub const CSelect_Creater: usize = 0x41D90F;
pub const CSelectScenario_Creater: usize = 0x41D949;
pub const CBattle_Creater: usize = 0x41D980;
pub const Ending_Creater: usize = 0x41D9BA;
pub const CSelectSV_Creater: usize = 0x41D9F4;
pub const CLoadingSV_Creater: usize = 0x41DA2E;
pub const CBattleSV_Creater: usize = 0x41DA65;
pub const CSelectCL_Creater: usize = 0x41DA9F;
pub const CLoadingCL_Creater: usize = 0x41DAD9;
pub const CBattleCL_Creater: usize = 0x41DB10;
pub const CLoadingWatch_Creater: usize = 0x41DB46;
pub const CBattleWatch_Creater: usize = 0x41DB79;
pub const CBattleManager_Creater: usize = 0x437D90;

// クラスサイズオペランド
macro_rules! define_raw_ptr_value {
    ($name: ident, $type: ty, $value: expr) => {
        pub const $name: *mut $type = $value as *mut $type;
    };
}
define_raw_ptr_value!(CLogo_Size, DWORD, 0x41D805);
define_raw_ptr_value!(Opening_Size, DWORD, 0x41D83F);
define_raw_ptr_value!(CLoading_Size, DWORD, 0x41D879);
define_raw_ptr_value!(CTitle_Size, DWORD, 0x41D8B3);
define_raw_ptr_value!(CSelect_Size, DWORD, 0x41D8ED);
define_raw_ptr_value!(CSelectScenario_Size, DWORD, 0x41D927);
define_raw_ptr_value!(CBattle_Size, BYTE, 0x41D961);
define_raw_ptr_value!(Ending_Size, DWORD, 0x41D998);
define_raw_ptr_value!(CSelectSV_Size, DWORD, 0x41D9D2);
define_raw_ptr_value!(CLoadingSV_Size, DWORD, 0x41DA0C);
define_raw_ptr_value!(CBattleSV_Size, BYTE, 0x41DA46);
define_raw_ptr_value!(CSelectCL_Size, DWORD, 0x41DA7D);
define_raw_ptr_value!(CLoadingCL_Size, DWORD, 0x41DAB7);
define_raw_ptr_value!(CBattleCL_Size, BYTE, 0x41DAF1);
define_raw_ptr_value!(CLoadingWatch_Size, DWORD, 0x41DB28);
define_raw_ptr_value!(CBattleWatch_Size, BYTE, 0x41DB5E);
define_raw_ptr_value!(CBattleManager_Size, DWORD, 0x437D72);

// セクションサイズ
pub const text_Offset: LPVOID = 0x401000 as LPVOID;
pub const text_Size: SIZE_T = 0x2AB000;
pub const rdata_Offset: LPVOID = 0x6AC000 as LPVOID;
pub const rdata_Size: SIZE_T = 0x24000;
pub const data_Offset: LPVOID = 0x6D0000 as LPVOID;
pub const data_Size: SIZE_T = 0x1E5A8;

// // ---------------- ここまでテンプレ ----------------
// #endif
