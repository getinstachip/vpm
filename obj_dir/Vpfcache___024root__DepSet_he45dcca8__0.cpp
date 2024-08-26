// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vpfcache.h for the primary calling header

#include "Vpfcache__pch.h"
#include "Vpfcache___024root.h"

void Vpfcache___024root___ico_sequent__TOP__0(Vpfcache___024root* vlSelf);

void Vpfcache___024root___eval_ico(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_ico\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1ULL & vlSelfRef.__VicoTriggered.word(0U))) {
        Vpfcache___024root___ico_sequent__TOP__0(vlSelf);
    }
}

VL_INLINE_OPT void Vpfcache___024root___ico_sequent__TOP__0(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___ico_sequent__TOP__0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.pfcache__DOT__w_invalidate_result = ((IData)(vlSelfRef.i_clear_cache) 
                                                   | (IData)(vlSelfRef.i_reset));
    vlSelfRef.pfcache__DOT__w_advance = ((IData)(vlSelfRef.i_new_pc) 
                                         | ((IData)(vlSelfRef.pfcache__DOT__r_v) 
                                            & (IData)(vlSelfRef.i_ready)));
}

void Vpfcache___024root___eval_triggers__ico(Vpfcache___024root* vlSelf);

bool Vpfcache___024root___eval_phase__ico(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_phase__ico\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    CData/*0:0*/ __VicoExecute;
    // Body
    Vpfcache___024root___eval_triggers__ico(vlSelf);
    __VicoExecute = vlSelfRef.__VicoTriggered.any();
    if (__VicoExecute) {
        Vpfcache___024root___eval_ico(vlSelf);
    }
    return (__VicoExecute);
}

void Vpfcache___024root___eval_act(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_act\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
}

void Vpfcache___024root___nba_sequent__TOP__0(Vpfcache___024root* vlSelf);

void Vpfcache___024root___eval_nba(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_nba\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1ULL & vlSelfRef.__VnbaTriggered.word(0U))) {
        Vpfcache___024root___nba_sequent__TOP__0(vlSelf);
    }
}

extern const VlUnpacked<CData/*0:0*/, 256> Vpfcache__ConstPool__TABLE_hbc7d87bb_0;
extern const VlUnpacked<CData/*1:0*/, 256> Vpfcache__ConstPool__TABLE_h2a7c9a01_0;
extern const VlUnpacked<CData/*1:0*/, 256> Vpfcache__ConstPool__TABLE_h772b86e4_0;
extern const VlWide<16>/*511:0*/ Vpfcache__ConstPool__CONST_h93e1b771_0;

VL_INLINE_OPT void Vpfcache___024root___nba_sequent__TOP__0(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___nba_sequent__TOP__0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    CData/*7:0*/ __Vtableidx1;
    __Vtableidx1 = 0;
    CData/*1:0*/ __Vdly__pfcache__DOT__delay;
    __Vdly__pfcache__DOT__delay = 0;
    CData/*0:0*/ __Vdly__o_wb_cyc;
    __Vdly__o_wb_cyc = 0;
    CData/*0:0*/ __Vdly__o_wb_stb;
    __Vdly__o_wb_stb = 0;
    IData/*17:0*/ __VdlyVal__pfcache__DOT__cache_tags__v0;
    __VdlyVal__pfcache__DOT__cache_tags__v0 = 0;
    SData/*8:0*/ __VdlyDim0__pfcache__DOT__cache_tags__v0;
    __VdlyDim0__pfcache__DOT__cache_tags__v0 = 0;
    SData/*11:0*/ __Vdly__pfcache__DOT__wraddr;
    __Vdly__pfcache__DOT__wraddr = 0;
    IData/*29:0*/ __Vdly__o_wb_addr;
    __Vdly__o_wb_addr = 0;
    IData/*31:0*/ __VdlyVal__pfcache__DOT__cache__v0;
    __VdlyVal__pfcache__DOT__cache__v0 = 0;
    SData/*11:0*/ __VdlyDim0__pfcache__DOT__cache__v0;
    __VdlyDim0__pfcache__DOT__cache__v0 = 0;
    CData/*0:0*/ __Vdly__o_illegal;
    __Vdly__o_illegal = 0;
    CData/*0:0*/ __VdlySet__pfcache__DOT__cache_tags__v0;
    __VdlySet__pfcache__DOT__cache_tags__v0 = 0;
    CData/*0:0*/ __VdlySet__pfcache__DOT__cache__v0;
    __VdlySet__pfcache__DOT__cache__v0 = 0;
    // Body
    __VdlySet__pfcache__DOT__cache__v0 = 0U;
    __VdlySet__pfcache__DOT__cache_tags__v0 = 0U;
    __Vdly__pfcache__DOT__delay = vlSelfRef.pfcache__DOT__delay;
    __Vdly__o_wb_addr = vlSelfRef.o_wb_addr;
    __Vdly__pfcache__DOT__wraddr = vlSelfRef.pfcache__DOT__wraddr;
    __Vdly__o_illegal = vlSelfRef.o_illegal;
    __Vdly__o_wb_stb = vlSelfRef.o_wb_stb;
    __Vdly__o_wb_cyc = vlSelfRef.o_wb_cyc;
    if ((((IData)(vlSelfRef.o_wb_stb) & (~ (IData)(vlSelfRef.i_wb_stall))) 
         & (~ (IData)(vlSelfRef.pfcache__DOT__last_addr)))) {
        __Vdly__o_wb_addr = ((0x3ffffff8U & __Vdly__o_wb_addr) 
                             | (7U & ((IData)(1U) + vlSelfRef.o_wb_addr)));
    } else if ((1U & (~ (IData)(vlSelfRef.o_wb_cyc)))) {
        __Vdly__o_wb_addr = (0x3ffffff8U & (vlSelfRef.pfcache__DOT__lastpc 
                                            >> 2U));
    }
    if ((((IData)(vlSelfRef.o_wb_cyc) & (IData)(vlSelfRef.i_wb_ack)) 
         & (~ (IData)(vlSelfRef.pfcache__DOT__last_ack)))) {
        __Vdly__pfcache__DOT__wraddr = ((0xff8U & (IData)(__Vdly__pfcache__DOT__wraddr)) 
                                        | (7U & ((IData)(1U) 
                                                 + (IData)(vlSelfRef.pfcache__DOT__wraddr))));
    } else if ((1U & (~ (IData)(vlSelfRef.o_wb_cyc)))) {
        __Vdly__pfcache__DOT__wraddr = (0xff8U & (vlSelfRef.pfcache__DOT__lastpc 
                                                  >> 2U));
    }
    if ((((IData)(vlSelfRef.i_reset) | (IData)(vlSelfRef.i_clear_cache)) 
         | (IData)(vlSelfRef.i_new_pc))) {
        __Vdly__o_illegal = 0U;
    } else if ((1U & (~ (IData)(vlSelfRef.o_illegal)))) {
        __Vdly__o_illegal = ((((~ (IData)(vlSelfRef.i_wb_err)) 
                               & (IData)(vlSelfRef.pfcache__DOT__illegal_valid)) 
                              & (~ (IData)(vlSelfRef.pfcache__DOT__isrc))) 
                             & (vlSelfRef.pfcache__DOT__illegal_cache 
                                == (vlSelfRef.pfcache__DOT__lastpc 
                                    >> 5U)));
    }
    if (((IData)(vlSelfRef.i_reset) | (IData)(vlSelfRef.i_clear_cache))) {
        __Vdly__o_wb_cyc = 0U;
        __Vdly__o_wb_stb = 0U;
    } else if (vlSelfRef.o_wb_cyc) {
        if (vlSelfRef.i_wb_err) {
            __Vdly__o_wb_stb = 0U;
        } else if ((((IData)(vlSelfRef.o_wb_stb) & 
                     (~ (IData)(vlSelfRef.i_wb_stall))) 
                    & (IData)(vlSelfRef.pfcache__DOT__last_addr))) {
            __Vdly__o_wb_stb = 0U;
        }
        if ((((IData)(vlSelfRef.i_wb_ack) & (IData)(vlSelfRef.pfcache__DOT__last_ack)) 
             | (IData)(vlSelfRef.i_wb_err))) {
            __Vdly__o_wb_cyc = 0U;
        }
    } else if (((IData)(vlSelfRef.pfcache__DOT__needload) 
                & (~ (IData)(vlSelfRef.i_new_pc)))) {
        __Vdly__o_wb_cyc = 1U;
        __Vdly__o_wb_stb = 1U;
    }
    vlSelfRef.pfcache__DOT__r_pc_cache = vlSelfRef.pfcache__DOT__cache
        [(0xfffU & (vlSelfRef.i_pc >> 2U))];
    vlSelfRef.pfcache__DOT__pc_tag_lookup = ((vlSelfRef.pfcache__DOT__cache_tags
                                              [(0x1ffU 
                                                & (vlSelfRef.i_pc 
                                                   >> 5U))] 
                                              << 9U) 
                                             | (0x1ffU 
                                                & (vlSelfRef.i_pc 
                                                   >> 5U)));
    vlSelfRef.pfcache__DOT__r_v_from_last = ((IData)(vlSelfRef.pfcache__DOT__w_v_from_last) 
                                             & (~ (IData)(vlSelfRef.pfcache__DOT__w_invalidate_result)));
    vlSelfRef.pfcache__DOT__r_last_cache = vlSelfRef.pfcache__DOT__cache
        [(0xfffU & (vlSelfRef.pfcache__DOT__lastpc 
                    >> 2U))];
    vlSelfRef.pfcache__DOT__last_tag_lookup = ((vlSelfRef.pfcache__DOT__cache_tags
                                                [(0x1ffU 
                                                  & (vlSelfRef.pfcache__DOT__lastpc 
                                                     >> 5U))] 
                                                << 9U) 
                                               | (0x1ffU 
                                                  & (vlSelfRef.pfcache__DOT__lastpc 
                                                     >> 5U)));
    __Vtableidx1 = (((IData)(vlSelfRef.pfcache__DOT__delay) 
                     << 6U) | (((IData)(vlSelfRef.o_wb_cyc) 
                                << 5U) | (((IData)(vlSelfRef.o_illegal) 
                                           << 4U) | 
                                          (((IData)(vlSelfRef.pfcache__DOT__r_v) 
                                            << 3U) 
                                           | (((IData)(vlSelfRef.pfcache__DOT__w_advance) 
                                               << 2U) 
                                              | (((IData)(vlSelfRef.i_clear_cache) 
                                                  << 1U) 
                                                 | (IData)(vlSelfRef.i_reset)))))));
    vlSelfRef.pfcache__DOT__rvsrc = Vpfcache__ConstPool__TABLE_hbc7d87bb_0
        [__Vtableidx1];
    if ((2U & Vpfcache__ConstPool__TABLE_h2a7c9a01_0
         [__Vtableidx1])) {
        __Vdly__pfcache__DOT__delay = Vpfcache__ConstPool__TABLE_h772b86e4_0
            [__Vtableidx1];
    }
    vlSelfRef.pfcache__DOT__r_v_from_pc = (((((vlSelfRef.i_pc 
                                               >> 5U) 
                                              == (vlSelfRef.pfcache__DOT__lastpc 
                                                  >> 5U)) 
                                             & ((vlSelfRef.pfcache__DOT__tag_lookup 
                                                 == 
                                                 (vlSelfRef.i_pc 
                                                  >> 5U)) 
                                                & (vlSelfRef.pfcache__DOT__valid_mask[
                                                   (0xfU 
                                                    & (vlSelfRef.i_pc 
                                                       >> 0xaU))] 
                                                   >> 
                                                   (0x1fU 
                                                    & (vlSelfRef.i_pc 
                                                       >> 5U))))) 
                                            & (~ (IData)(vlSelfRef.pfcache__DOT__w_invalidate_result))) 
                                           & (~ (IData)(vlSelfRef.o_illegal)));
    if (((IData)(vlSelfRef.o_wb_cyc) & (IData)(vlSelfRef.i_wb_ack))) {
        __VdlyVal__pfcache__DOT__cache_tags__v0 = (0x3ffffU 
                                                   & (vlSelfRef.o_wb_addr 
                                                      >> 0xcU));
        __VdlyDim0__pfcache__DOT__cache_tags__v0 = 
            (0x1ffU & (vlSelfRef.o_wb_addr >> 3U));
        __VdlySet__pfcache__DOT__cache_tags__v0 = 1U;
    }
    if (((IData)(vlSelfRef.i_reset) | (IData)(vlSelfRef.i_clear_cache))) {
        vlSelfRef.pfcache__DOT__valid_mask[0U] = Vpfcache__ConstPool__CONST_h93e1b771_0[0U];
        vlSelfRef.pfcache__DOT__valid_mask[1U] = Vpfcache__ConstPool__CONST_h93e1b771_0[1U];
        vlSelfRef.pfcache__DOT__valid_mask[2U] = Vpfcache__ConstPool__CONST_h93e1b771_0[2U];
        vlSelfRef.pfcache__DOT__valid_mask[3U] = Vpfcache__ConstPool__CONST_h93e1b771_0[3U];
        vlSelfRef.pfcache__DOT__valid_mask[4U] = Vpfcache__ConstPool__CONST_h93e1b771_0[4U];
        vlSelfRef.pfcache__DOT__valid_mask[5U] = Vpfcache__ConstPool__CONST_h93e1b771_0[5U];
        vlSelfRef.pfcache__DOT__valid_mask[6U] = Vpfcache__ConstPool__CONST_h93e1b771_0[6U];
        vlSelfRef.pfcache__DOT__valid_mask[7U] = Vpfcache__ConstPool__CONST_h93e1b771_0[7U];
        vlSelfRef.pfcache__DOT__valid_mask[8U] = Vpfcache__ConstPool__CONST_h93e1b771_0[8U];
        vlSelfRef.pfcache__DOT__valid_mask[9U] = Vpfcache__ConstPool__CONST_h93e1b771_0[9U];
        vlSelfRef.pfcache__DOT__valid_mask[0xaU] = 
            Vpfcache__ConstPool__CONST_h93e1b771_0[0xaU];
        vlSelfRef.pfcache__DOT__valid_mask[0xbU] = 
            Vpfcache__ConstPool__CONST_h93e1b771_0[0xbU];
        vlSelfRef.pfcache__DOT__valid_mask[0xcU] = 
            Vpfcache__ConstPool__CONST_h93e1b771_0[0xcU];
        vlSelfRef.pfcache__DOT__valid_mask[0xdU] = 
            Vpfcache__ConstPool__CONST_h93e1b771_0[0xdU];
        vlSelfRef.pfcache__DOT__valid_mask[0xeU] = 
            Vpfcache__ConstPool__CONST_h93e1b771_0[0xeU];
        vlSelfRef.pfcache__DOT__valid_mask[0xfU] = 
            Vpfcache__ConstPool__CONST_h93e1b771_0[0xfU];
    } else {
        if (vlSelfRef.pfcache__DOT__svmask) {
            vlSelfRef.pfcache__DOT__valid_mask[((IData)(vlSelfRef.pfcache__DOT__saddr) 
                                                >> 5U)] 
                = (((~ ((IData)(1U) << (0x1fU & (IData)(vlSelfRef.pfcache__DOT__saddr)))) 
                    & vlSelfRef.pfcache__DOT__valid_mask[
                    ((IData)(vlSelfRef.pfcache__DOT__saddr) 
                     >> 5U)]) | ((1U & (~ (IData)(vlSelfRef.pfcache__DOT__bus_abort))) 
                                 << (0x1fU & (IData)(vlSelfRef.pfcache__DOT__saddr))));
        }
        if (((~ (IData)(vlSelfRef.o_wb_cyc)) & (IData)(vlSelfRef.pfcache__DOT__needload))) {
            vlSelfRef.pfcache__DOT__valid_mask[(0xfU 
                                                & (vlSelfRef.pfcache__DOT__lastpc 
                                                   >> 0xaU))] 
                = ((~ ((IData)(1U) << (0x1fU & (vlSelfRef.pfcache__DOT__lastpc 
                                                >> 5U)))) 
                   & vlSelfRef.pfcache__DOT__valid_mask[
                   (0xfU & (vlSelfRef.pfcache__DOT__lastpc 
                            >> 0xaU))]);
        }
    }
    if (((IData)(vlSelfRef.o_wb_cyc) & (IData)(vlSelfRef.i_wb_ack))) {
        vlSelfRef.pfcache__DOT__saddr = (0x1ffU & ((IData)(vlSelfRef.pfcache__DOT__wraddr) 
                                                   >> 3U));
    }
    vlSelfRef.o_wb_stb = __Vdly__o_wb_stb;
    if (vlSelfRef.o_wb_cyc) {
        __VdlyVal__pfcache__DOT__cache__v0 = vlSelfRef.i_wb_data;
        __VdlyDim0__pfcache__DOT__cache__v0 = vlSelfRef.pfcache__DOT__wraddr;
        __VdlySet__pfcache__DOT__cache__v0 = 1U;
    }
    if (__VdlySet__pfcache__DOT__cache__v0) {
        vlSelfRef.pfcache__DOT__cache[__VdlyDim0__pfcache__DOT__cache__v0] 
            = __VdlyVal__pfcache__DOT__cache__v0;
    }
    if (__VdlySet__pfcache__DOT__cache_tags__v0) {
        vlSelfRef.pfcache__DOT__cache_tags[__VdlyDim0__pfcache__DOT__cache_tags__v0] 
            = __VdlyVal__pfcache__DOT__cache_tags__v0;
    }
    if (vlSelfRef.pfcache__DOT__w_advance) {
        vlSelfRef.pfcache__DOT__isrc = 1U;
        vlSelfRef.pfcache__DOT__r_pc = vlSelfRef.i_pc;
    } else {
        vlSelfRef.pfcache__DOT__isrc = 0U;
        vlSelfRef.pfcache__DOT__r_pc = vlSelfRef.pfcache__DOT__lastpc;
    }
    vlSelfRef.o_pc = vlSelfRef.pfcache__DOT__r_pc;
    vlSelfRef.pfcache__DOT__r_v = ((IData)(vlSelfRef.pfcache__DOT__rvsrc)
                                    ? (IData)(vlSelfRef.pfcache__DOT__r_v_from_pc)
                                    : (IData)(vlSelfRef.pfcache__DOT__r_v_from_last));
    if (vlSelfRef.pfcache__DOT__isrc) {
        vlSelfRef.o_insn = vlSelfRef.pfcache__DOT__r_pc_cache;
        vlSelfRef.pfcache__DOT__tag_lookup = vlSelfRef.pfcache__DOT__pc_tag_lookup;
    } else {
        vlSelfRef.o_insn = vlSelfRef.pfcache__DOT__r_last_cache;
        vlSelfRef.pfcache__DOT__tag_lookup = vlSelfRef.pfcache__DOT__last_tag_lookup;
    }
    vlSelfRef.pfcache__DOT__svmask = ((1U & (~ ((IData)(vlSelfRef.i_reset) 
                                                | (IData)(vlSelfRef.i_clear_cache)))) 
                                      && ((((IData)(vlSelfRef.o_wb_cyc) 
                                            & (IData)(vlSelfRef.i_wb_ack)) 
                                           & (IData)(vlSelfRef.pfcache__DOT__last_ack)) 
                                          & (~ (IData)(vlSelfRef.pfcache__DOT__bus_abort))));
    vlSelfRef.pfcache__DOT__needload = ((~ ((IData)(vlSelfRef.i_clear_cache) 
                                            | (IData)(vlSelfRef.o_wb_cyc))) 
                                        & ((~ ((IData)(vlSelfRef.pfcache__DOT__w_advance) 
                                               & (~ (IData)(vlSelfRef.o_illegal)))) 
                                           & (((0U 
                                                == (IData)(vlSelfRef.pfcache__DOT__delay)) 
                                               & (~ (IData)(vlSelfRef.pfcache__DOT__w_v_from_last))) 
                                              & ((~ (IData)(vlSelfRef.pfcache__DOT__illegal_valid)) 
                                                 | ((vlSelfRef.pfcache__DOT__lastpc 
                                                     >> 5U) 
                                                    != vlSelfRef.pfcache__DOT__illegal_cache)))));
    if (((IData)(vlSelfRef.i_reset) | (IData)(vlSelfRef.i_clear_cache))) {
        vlSelfRef.pfcache__DOT__illegal_valid = 0U;
        vlSelfRef.pfcache__DOT__illegal_cache = 0U;
    } else if (((IData)(vlSelfRef.o_wb_cyc) & (IData)(vlSelfRef.i_wb_err))) {
        vlSelfRef.pfcache__DOT__illegal_valid = 1U;
        vlSelfRef.pfcache__DOT__illegal_cache = (0x7ffffffU 
                                                 & (vlSelfRef.o_wb_addr 
                                                    >> 3U));
    } else if ((((((IData)(vlSelfRef.o_wb_cyc) & (IData)(vlSelfRef.i_wb_ack)) 
                  & (IData)(vlSelfRef.pfcache__DOT__last_ack)) 
                 & (~ (IData)(vlSelfRef.pfcache__DOT__bus_abort))) 
                & ((0x1ffU & ((IData)(vlSelfRef.pfcache__DOT__wraddr) 
                              >> 3U)) == (0x1ffU & vlSelfRef.pfcache__DOT__illegal_cache)))) {
        vlSelfRef.pfcache__DOT__illegal_valid = 0U;
    }
    if (vlSelfRef.o_wb_cyc) {
        if ((IData)(((6U == (6U & vlSelfRef.o_wb_addr)) 
                     & ((~ (IData)(vlSelfRef.i_wb_stall)) 
                        | vlSelfRef.o_wb_addr)))) {
            vlSelfRef.pfcache__DOT__last_addr = 1U;
        }
        if (((IData)(vlSelfRef.i_clear_cache) | (IData)(vlSelfRef.i_new_pc))) {
            vlSelfRef.pfcache__DOT__bus_abort = 1U;
        }
    } else {
        vlSelfRef.pfcache__DOT__last_addr = 0U;
        vlSelfRef.pfcache__DOT__bus_abort = 0U;
    }
    if (vlSelfRef.pfcache__DOT__w_advance) {
        vlSelfRef.pfcache__DOT__lastpc = vlSelfRef.i_pc;
    }
    vlSelfRef.pfcache__DOT__delay = __Vdly__pfcache__DOT__delay;
    vlSelfRef.o_illegal = __Vdly__o_illegal;
    vlSelfRef.o_valid = ((IData)(vlSelfRef.pfcache__DOT__r_v) 
                         | (IData)(vlSelfRef.o_illegal));
    vlSelfRef.pfcache__DOT__w_advance = ((IData)(vlSelfRef.i_new_pc) 
                                         | ((IData)(vlSelfRef.pfcache__DOT__r_v) 
                                            & (IData)(vlSelfRef.i_ready)));
    vlSelfRef.pfcache__DOT__w_v_from_last = ((vlSelfRef.pfcache__DOT__tag_lookup 
                                              == (vlSelfRef.pfcache__DOT__lastpc 
                                                  >> 5U)) 
                                             & (vlSelfRef.pfcache__DOT__valid_mask[
                                                (0xfU 
                                                 & (vlSelfRef.pfcache__DOT__lastpc 
                                                    >> 0xaU))] 
                                                >> 
                                                (0x1fU 
                                                 & (vlSelfRef.pfcache__DOT__lastpc 
                                                    >> 5U))));
    vlSelfRef.pfcache__DOT__last_ack = ((IData)(vlSelfRef.o_wb_cyc) 
                                        & (IData)((
                                                   (6U 
                                                    == 
                                                    (6U 
                                                     & (IData)(vlSelfRef.pfcache__DOT__wraddr))) 
                                                   & ((IData)(vlSelfRef.pfcache__DOT__wraddr) 
                                                      | (IData)(vlSelfRef.i_wb_ack)))));
    vlSelfRef.pfcache__DOT__wraddr = __Vdly__pfcache__DOT__wraddr;
    vlSelfRef.o_wb_addr = __Vdly__o_wb_addr;
    vlSelfRef.o_wb_cyc = __Vdly__o_wb_cyc;
}

void Vpfcache___024root___eval_triggers__act(Vpfcache___024root* vlSelf);

bool Vpfcache___024root___eval_phase__act(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_phase__act\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    VlTriggerVec<1> __VpreTriggered;
    CData/*0:0*/ __VactExecute;
    // Body
    Vpfcache___024root___eval_triggers__act(vlSelf);
    __VactExecute = vlSelfRef.__VactTriggered.any();
    if (__VactExecute) {
        __VpreTriggered.andNot(vlSelfRef.__VactTriggered, vlSelfRef.__VnbaTriggered);
        vlSelfRef.__VnbaTriggered.thisOr(vlSelfRef.__VactTriggered);
        Vpfcache___024root___eval_act(vlSelf);
    }
    return (__VactExecute);
}

bool Vpfcache___024root___eval_phase__nba(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_phase__nba\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    CData/*0:0*/ __VnbaExecute;
    // Body
    __VnbaExecute = vlSelfRef.__VnbaTriggered.any();
    if (__VnbaExecute) {
        Vpfcache___024root___eval_nba(vlSelf);
        vlSelfRef.__VnbaTriggered.clear();
    }
    return (__VnbaExecute);
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__ico(Vpfcache___024root* vlSelf);
#endif  // VL_DEBUG
#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__nba(Vpfcache___024root* vlSelf);
#endif  // VL_DEBUG
#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__act(Vpfcache___024root* vlSelf);
#endif  // VL_DEBUG

void Vpfcache___024root___eval(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    IData/*31:0*/ __VicoIterCount;
    CData/*0:0*/ __VicoContinue;
    IData/*31:0*/ __VnbaIterCount;
    CData/*0:0*/ __VnbaContinue;
    // Body
    __VicoIterCount = 0U;
    vlSelfRef.__VicoFirstIteration = 1U;
    __VicoContinue = 1U;
    while (__VicoContinue) {
        if (VL_UNLIKELY((0x64U < __VicoIterCount))) {
#ifdef VL_DEBUG
            Vpfcache___024root___dump_triggers__ico(vlSelf);
#endif
            VL_FATAL_MT("vpm_modules/pfcache/pfcache.v", 63, "", "Input combinational region did not converge.");
        }
        __VicoIterCount = ((IData)(1U) + __VicoIterCount);
        __VicoContinue = 0U;
        if (Vpfcache___024root___eval_phase__ico(vlSelf)) {
            __VicoContinue = 1U;
        }
        vlSelfRef.__VicoFirstIteration = 0U;
    }
    __VnbaIterCount = 0U;
    __VnbaContinue = 1U;
    while (__VnbaContinue) {
        if (VL_UNLIKELY((0x64U < __VnbaIterCount))) {
#ifdef VL_DEBUG
            Vpfcache___024root___dump_triggers__nba(vlSelf);
#endif
            VL_FATAL_MT("vpm_modules/pfcache/pfcache.v", 63, "", "NBA region did not converge.");
        }
        __VnbaIterCount = ((IData)(1U) + __VnbaIterCount);
        __VnbaContinue = 0U;
        vlSelfRef.__VactIterCount = 0U;
        vlSelfRef.__VactContinue = 1U;
        while (vlSelfRef.__VactContinue) {
            if (VL_UNLIKELY((0x64U < vlSelfRef.__VactIterCount))) {
#ifdef VL_DEBUG
                Vpfcache___024root___dump_triggers__act(vlSelf);
#endif
                VL_FATAL_MT("vpm_modules/pfcache/pfcache.v", 63, "", "Active region did not converge.");
            }
            vlSelfRef.__VactIterCount = ((IData)(1U) 
                                         + vlSelfRef.__VactIterCount);
            vlSelfRef.__VactContinue = 0U;
            if (Vpfcache___024root___eval_phase__act(vlSelf)) {
                vlSelfRef.__VactContinue = 1U;
            }
        }
        if (Vpfcache___024root___eval_phase__nba(vlSelf)) {
            __VnbaContinue = 1U;
        }
    }
}

#ifdef VL_DEBUG
void Vpfcache___024root___eval_debug_assertions(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_debug_assertions\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if (VL_UNLIKELY((vlSelfRef.i_clk & 0xfeU))) {
        Verilated::overWidthError("i_clk");}
    if (VL_UNLIKELY((vlSelfRef.i_reset & 0xfeU))) {
        Verilated::overWidthError("i_reset");}
    if (VL_UNLIKELY((vlSelfRef.i_new_pc & 0xfeU))) {
        Verilated::overWidthError("i_new_pc");}
    if (VL_UNLIKELY((vlSelfRef.i_clear_cache & 0xfeU))) {
        Verilated::overWidthError("i_clear_cache");}
    if (VL_UNLIKELY((vlSelfRef.i_ready & 0xfeU))) {
        Verilated::overWidthError("i_ready");}
    if (VL_UNLIKELY((vlSelfRef.i_wb_stall & 0xfeU))) {
        Verilated::overWidthError("i_wb_stall");}
    if (VL_UNLIKELY((vlSelfRef.i_wb_ack & 0xfeU))) {
        Verilated::overWidthError("i_wb_ack");}
    if (VL_UNLIKELY((vlSelfRef.i_wb_err & 0xfeU))) {
        Verilated::overWidthError("i_wb_err");}
}
#endif  // VL_DEBUG
