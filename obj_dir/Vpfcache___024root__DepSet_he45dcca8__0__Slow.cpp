// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vpfcache.h for the primary calling header

#include "Vpfcache__pch.h"
#include "Vpfcache___024root.h"

VL_ATTR_COLD void Vpfcache___024root___eval_static(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_static\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
}

VL_ATTR_COLD void Vpfcache___024root___eval_initial__TOP(Vpfcache___024root* vlSelf);

VL_ATTR_COLD void Vpfcache___024root___eval_initial(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_initial\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    Vpfcache___024root___eval_initial__TOP(vlSelf);
    vlSelfRef.__Vtrigprevexpr___TOP__i_clk__0 = vlSelfRef.i_clk;
}

extern const VlWide<16>/*511:0*/ Vpfcache__ConstPool__CONST_h93e1b771_0;

VL_ATTR_COLD void Vpfcache___024root___eval_initial__TOP(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_initial__TOP\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.pfcache__DOT__r_pc = 0U;
    vlSelfRef.pfcache__DOT__lastpc = 0U;
    vlSelfRef.pfcache__DOT__delay = 3U;
    vlSelfRef.pfcache__DOT__r_v_from_pc = 0U;
    vlSelfRef.pfcache__DOT__r_v_from_last = 0U;
    vlSelfRef.pfcache__DOT__needload = 0U;
    vlSelfRef.pfcache__DOT__last_addr = 0U;
    vlSelfRef.pfcache__DOT__last_ack = 0U;
    vlSelfRef.pfcache__DOT__bus_abort = 0U;
    vlSelfRef.o_wb_cyc = 0U;
    vlSelfRef.o_wb_stb = 0U;
    vlSelfRef.pfcache__DOT__wraddr = 0U;
    vlSelfRef.o_wb_addr = 0U;
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
    vlSelfRef.pfcache__DOT__valid_mask[0xaU] = Vpfcache__ConstPool__CONST_h93e1b771_0[0xaU];
    vlSelfRef.pfcache__DOT__valid_mask[0xbU] = Vpfcache__ConstPool__CONST_h93e1b771_0[0xbU];
    vlSelfRef.pfcache__DOT__valid_mask[0xcU] = Vpfcache__ConstPool__CONST_h93e1b771_0[0xcU];
    vlSelfRef.pfcache__DOT__valid_mask[0xdU] = Vpfcache__ConstPool__CONST_h93e1b771_0[0xdU];
    vlSelfRef.pfcache__DOT__valid_mask[0xeU] = Vpfcache__ConstPool__CONST_h93e1b771_0[0xeU];
    vlSelfRef.pfcache__DOT__valid_mask[0xfU] = Vpfcache__ConstPool__CONST_h93e1b771_0[0xfU];
    vlSelfRef.pfcache__DOT__svmask = 0U;
    vlSelfRef.pfcache__DOT__illegal_cache = 0U;
    vlSelfRef.pfcache__DOT__illegal_valid = 0U;
    vlSelfRef.o_illegal = 0U;
    vlSelfRef.o_wb_we = 0U;
    vlSelfRef.o_wb_data = 0U;
}

VL_ATTR_COLD void Vpfcache___024root___eval_final(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_final\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__stl(Vpfcache___024root* vlSelf);
#endif  // VL_DEBUG
VL_ATTR_COLD bool Vpfcache___024root___eval_phase__stl(Vpfcache___024root* vlSelf);

VL_ATTR_COLD void Vpfcache___024root___eval_settle(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_settle\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    IData/*31:0*/ __VstlIterCount;
    CData/*0:0*/ __VstlContinue;
    // Body
    __VstlIterCount = 0U;
    vlSelfRef.__VstlFirstIteration = 1U;
    __VstlContinue = 1U;
    while (__VstlContinue) {
        if (VL_UNLIKELY((0x64U < __VstlIterCount))) {
#ifdef VL_DEBUG
            Vpfcache___024root___dump_triggers__stl(vlSelf);
#endif
            VL_FATAL_MT("vpm_modules/pfcache/pfcache.v", 63, "", "Settle region did not converge.");
        }
        __VstlIterCount = ((IData)(1U) + __VstlIterCount);
        __VstlContinue = 0U;
        if (Vpfcache___024root___eval_phase__stl(vlSelf)) {
            __VstlContinue = 1U;
        }
        vlSelfRef.__VstlFirstIteration = 0U;
    }
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__stl(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___dump_triggers__stl\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1U & (~ vlSelfRef.__VstlTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelfRef.__VstlTriggered.word(0U))) {
        VL_DBG_MSGF("         'stl' region trigger index 0 is active: Internal 'stl' trigger - first iteration\n");
    }
}
#endif  // VL_DEBUG

VL_ATTR_COLD void Vpfcache___024root___stl_sequent__TOP__0(Vpfcache___024root* vlSelf);

VL_ATTR_COLD void Vpfcache___024root___eval_stl(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_stl\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1ULL & vlSelfRef.__VstlTriggered.word(0U))) {
        Vpfcache___024root___stl_sequent__TOP__0(vlSelf);
    }
}

VL_ATTR_COLD void Vpfcache___024root___stl_sequent__TOP__0(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___stl_sequent__TOP__0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.o_pc = vlSelfRef.pfcache__DOT__r_pc;
    vlSelfRef.pfcache__DOT__w_invalidate_result = ((IData)(vlSelfRef.i_clear_cache) 
                                                   | (IData)(vlSelfRef.i_reset));
    if (vlSelfRef.pfcache__DOT__isrc) {
        vlSelfRef.o_insn = vlSelfRef.pfcache__DOT__r_pc_cache;
        vlSelfRef.pfcache__DOT__tag_lookup = vlSelfRef.pfcache__DOT__pc_tag_lookup;
    } else {
        vlSelfRef.o_insn = vlSelfRef.pfcache__DOT__r_last_cache;
        vlSelfRef.pfcache__DOT__tag_lookup = vlSelfRef.pfcache__DOT__last_tag_lookup;
    }
    vlSelfRef.pfcache__DOT__r_v = ((IData)(vlSelfRef.pfcache__DOT__rvsrc)
                                    ? (IData)(vlSelfRef.pfcache__DOT__r_v_from_pc)
                                    : (IData)(vlSelfRef.pfcache__DOT__r_v_from_last));
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
    vlSelfRef.o_valid = ((IData)(vlSelfRef.pfcache__DOT__r_v) 
                         | (IData)(vlSelfRef.o_illegal));
    vlSelfRef.pfcache__DOT__w_advance = ((IData)(vlSelfRef.i_new_pc) 
                                         | ((IData)(vlSelfRef.pfcache__DOT__r_v) 
                                            & (IData)(vlSelfRef.i_ready)));
}

VL_ATTR_COLD void Vpfcache___024root___eval_triggers__stl(Vpfcache___024root* vlSelf);

VL_ATTR_COLD bool Vpfcache___024root___eval_phase__stl(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_phase__stl\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    CData/*0:0*/ __VstlExecute;
    // Body
    Vpfcache___024root___eval_triggers__stl(vlSelf);
    __VstlExecute = vlSelfRef.__VstlTriggered.any();
    if (__VstlExecute) {
        Vpfcache___024root___eval_stl(vlSelf);
    }
    return (__VstlExecute);
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__ico(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___dump_triggers__ico\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1U & (~ vlSelfRef.__VicoTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelfRef.__VicoTriggered.word(0U))) {
        VL_DBG_MSGF("         'ico' region trigger index 0 is active: Internal 'ico' trigger - first iteration\n");
    }
}
#endif  // VL_DEBUG

#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__act(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___dump_triggers__act\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1U & (~ vlSelfRef.__VactTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelfRef.__VactTriggered.word(0U))) {
        VL_DBG_MSGF("         'act' region trigger index 0 is active: @(posedge i_clk)\n");
    }
}
#endif  // VL_DEBUG

#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__nba(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___dump_triggers__nba\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1U & (~ vlSelfRef.__VnbaTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelfRef.__VnbaTriggered.word(0U))) {
        VL_DBG_MSGF("         'nba' region trigger index 0 is active: @(posedge i_clk)\n");
    }
}
#endif  // VL_DEBUG

VL_ATTR_COLD void Vpfcache___024root___ctor_var_reset(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___ctor_var_reset\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelf->i_clk = VL_RAND_RESET_I(1);
    vlSelf->i_reset = VL_RAND_RESET_I(1);
    vlSelf->i_new_pc = VL_RAND_RESET_I(1);
    vlSelf->i_clear_cache = VL_RAND_RESET_I(1);
    vlSelf->i_ready = VL_RAND_RESET_I(1);
    vlSelf->i_pc = VL_RAND_RESET_I(32);
    vlSelf->o_valid = VL_RAND_RESET_I(1);
    vlSelf->o_illegal = VL_RAND_RESET_I(1);
    vlSelf->o_insn = VL_RAND_RESET_I(32);
    vlSelf->o_pc = VL_RAND_RESET_I(32);
    vlSelf->o_wb_cyc = VL_RAND_RESET_I(1);
    vlSelf->o_wb_stb = VL_RAND_RESET_I(1);
    vlSelf->o_wb_we = VL_RAND_RESET_I(1);
    vlSelf->o_wb_addr = VL_RAND_RESET_I(30);
    vlSelf->o_wb_data = VL_RAND_RESET_I(32);
    vlSelf->i_wb_stall = VL_RAND_RESET_I(1);
    vlSelf->i_wb_ack = VL_RAND_RESET_I(1);
    vlSelf->i_wb_err = VL_RAND_RESET_I(1);
    vlSelf->i_wb_data = VL_RAND_RESET_I(32);
    vlSelf->pfcache__DOT__r_v = VL_RAND_RESET_I(1);
    for (int __Vi0 = 0; __Vi0 < 4096; ++__Vi0) {
        vlSelf->pfcache__DOT__cache[__Vi0] = VL_RAND_RESET_I(32);
    }
    for (int __Vi0 = 0; __Vi0 < 512; ++__Vi0) {
        vlSelf->pfcache__DOT__cache_tags[__Vi0] = VL_RAND_RESET_I(18);
    }
    VL_RAND_RESET_W(512, vlSelf->pfcache__DOT__valid_mask);
    vlSelf->pfcache__DOT__r_v_from_pc = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__r_v_from_last = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__rvsrc = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__w_v_from_last = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__lastpc = VL_RAND_RESET_I(32);
    vlSelf->pfcache__DOT__wraddr = VL_RAND_RESET_I(12);
    vlSelf->pfcache__DOT__pc_tag_lookup = VL_RAND_RESET_I(27);
    vlSelf->pfcache__DOT__last_tag_lookup = VL_RAND_RESET_I(27);
    vlSelf->pfcache__DOT__tag_lookup = VL_RAND_RESET_I(27);
    vlSelf->pfcache__DOT__illegal_valid = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__illegal_cache = VL_RAND_RESET_I(27);
    vlSelf->pfcache__DOT__r_pc_cache = VL_RAND_RESET_I(32);
    vlSelf->pfcache__DOT__r_last_cache = VL_RAND_RESET_I(32);
    vlSelf->pfcache__DOT__r_pc = VL_RAND_RESET_I(32);
    vlSelf->pfcache__DOT__isrc = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__delay = VL_RAND_RESET_I(2);
    vlSelf->pfcache__DOT__svmask = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__last_ack = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__needload = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__last_addr = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__bus_abort = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__saddr = VL_RAND_RESET_I(9);
    vlSelf->pfcache__DOT__w_advance = VL_RAND_RESET_I(1);
    vlSelf->pfcache__DOT__w_invalidate_result = VL_RAND_RESET_I(1);
    vlSelf->__Vtrigprevexpr___TOP__i_clk__0 = VL_RAND_RESET_I(1);
}
