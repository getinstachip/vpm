// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vtestbench.h for the primary calling header

#include "Vtestbench__pch.h"
#include "Vtestbench___024root.h"

VlCoroutine Vtestbench___024root___eval_initial__TOP__Vtiming__0(Vtestbench___024root* vlSelf);

void Vtestbench___024root___eval_initial(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___eval_initial\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    Vtestbench___024root___eval_initial__TOP__Vtiming__0(vlSelf);
    vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__a__0 
        = vlSelfRef.testbench__DOT__a;
    vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__b__0 
        = vlSelfRef.testbench__DOT__b;
    vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__cin__0 
        = vlSelfRef.testbench__DOT__cin;
    vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__cout__0 
        = vlSelfRef.testbench__DOT__cout;
    vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__neg__0 
        = vlSelfRef.testbench__DOT__neg;
    vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__opt_sub__0 
        = vlSelfRef.testbench__DOT__opt_sub;
    vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__overflow__0 
        = vlSelfRef.testbench__DOT__overflow;
    vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__sum__0 
        = vlSelfRef.testbench__DOT__sum;
    vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__zero__0 
        = vlSelfRef.testbench__DOT__zero;
}

void Vtestbench___024root___act_sequent__TOP__0(Vtestbench___024root* vlSelf);

void Vtestbench___024root___eval_act(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___eval_act\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((2ULL & vlSelfRef.__VactTriggered.word(0U))) {
        Vtestbench___024root___act_sequent__TOP__0(vlSelf);
        vlSelfRef.__Vm_traceActivity[1U] = 1U;
    }
}

VL_INLINE_OPT void Vtestbench___024root___act_sequent__TOP__0(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___act_sequent__TOP__0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.testbench__DOT__uut__DOT__b_sub = (0xfU 
                                                 & ((- (IData)((IData)(vlSelfRef.testbench__DOT__opt_sub))) 
                                                    ^ (IData)(vlSelfRef.testbench__DOT__b)));
    vlSelfRef.testbench__DOT__cout = (1U & (((IData)(vlSelfRef.testbench__DOT__a) 
                                             + ((IData)(vlSelfRef.testbench__DOT__uut__DOT__b_sub) 
                                                + (
                                                   (0xfU 
                                                    & (- (IData)((IData)(vlSelfRef.testbench__DOT__opt_sub)))) 
                                                   + 
                                                   (0xfU 
                                                    & (- (IData)((IData)(vlSelfRef.testbench__DOT__cin))))))) 
                                            >> 4U));
    vlSelfRef.testbench__DOT__sum = (0xfU & ((IData)(vlSelfRef.testbench__DOT__a) 
                                             + ((IData)(vlSelfRef.testbench__DOT__uut__DOT__b_sub) 
                                                + (
                                                   (- (IData)((IData)(vlSelfRef.testbench__DOT__opt_sub))) 
                                                   + 
                                                   (- (IData)((IData)(vlSelfRef.testbench__DOT__cin)))))));
    vlSelfRef.testbench__DOT__zero = (0U == (IData)(vlSelfRef.testbench__DOT__sum));
    vlSelfRef.testbench__DOT__neg = (1U & ((IData)(vlSelfRef.testbench__DOT__sum) 
                                           >> 3U));
    vlSelfRef.testbench__DOT__overflow = (((1U & ((IData)(vlSelfRef.testbench__DOT__a) 
                                                  >> 3U)) 
                                           == (1U & 
                                               ((IData)(vlSelfRef.testbench__DOT__uut__DOT__b_sub) 
                                                >> 3U))) 
                                          & ((IData)(vlSelfRef.testbench__DOT__neg) 
                                             != (1U 
                                                 & ((IData)(vlSelfRef.testbench__DOT__a) 
                                                    >> 3U))));
}

void Vtestbench___024root___nba_sequent__TOP__0(Vtestbench___024root* vlSelf);

void Vtestbench___024root___eval_nba(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___eval_nba\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1ULL & vlSelfRef.__VnbaTriggered.word(0U))) {
        Vtestbench___024root___nba_sequent__TOP__0(vlSelf);
    }
}

void Vtestbench___024root___timing_resume(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___timing_resume\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((2ULL & vlSelfRef.__VactTriggered.word(0U))) {
        vlSelfRef.__VdlySched.resume();
    }
}

void Vtestbench___024root___eval_triggers__act(Vtestbench___024root* vlSelf);

bool Vtestbench___024root___eval_phase__act(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___eval_phase__act\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    VlTriggerVec<2> __VpreTriggered;
    CData/*0:0*/ __VactExecute;
    // Body
    Vtestbench___024root___eval_triggers__act(vlSelf);
    __VactExecute = vlSelfRef.__VactTriggered.any();
    if (__VactExecute) {
        __VpreTriggered.andNot(vlSelfRef.__VactTriggered, vlSelfRef.__VnbaTriggered);
        vlSelfRef.__VnbaTriggered.thisOr(vlSelfRef.__VactTriggered);
        Vtestbench___024root___timing_resume(vlSelf);
        Vtestbench___024root___eval_act(vlSelf);
    }
    return (__VactExecute);
}

bool Vtestbench___024root___eval_phase__nba(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___eval_phase__nba\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    CData/*0:0*/ __VnbaExecute;
    // Body
    __VnbaExecute = vlSelfRef.__VnbaTriggered.any();
    if (__VnbaExecute) {
        Vtestbench___024root___eval_nba(vlSelf);
        vlSelfRef.__VnbaTriggered.clear();
    }
    return (__VnbaExecute);
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vtestbench___024root___dump_triggers__nba(Vtestbench___024root* vlSelf);
#endif  // VL_DEBUG
#ifdef VL_DEBUG
VL_ATTR_COLD void Vtestbench___024root___dump_triggers__act(Vtestbench___024root* vlSelf);
#endif  // VL_DEBUG

void Vtestbench___024root___eval(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___eval\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Init
    IData/*31:0*/ __VnbaIterCount;
    CData/*0:0*/ __VnbaContinue;
    // Body
    __VnbaIterCount = 0U;
    __VnbaContinue = 1U;
    while (__VnbaContinue) {
        if (VL_UNLIKELY((0x64U < __VnbaIterCount))) {
#ifdef VL_DEBUG
            Vtestbench___024root___dump_triggers__nba(vlSelf);
#endif
            VL_FATAL_MT("add/testbench.v", 3, "", "NBA region did not converge.");
        }
        __VnbaIterCount = ((IData)(1U) + __VnbaIterCount);
        __VnbaContinue = 0U;
        vlSelfRef.__VactIterCount = 0U;
        vlSelfRef.__VactContinue = 1U;
        while (vlSelfRef.__VactContinue) {
            if (VL_UNLIKELY((0x64U < vlSelfRef.__VactIterCount))) {
#ifdef VL_DEBUG
                Vtestbench___024root___dump_triggers__act(vlSelf);
#endif
                VL_FATAL_MT("add/testbench.v", 3, "", "Active region did not converge.");
            }
            vlSelfRef.__VactIterCount = ((IData)(1U) 
                                         + vlSelfRef.__VactIterCount);
            vlSelfRef.__VactContinue = 0U;
            if (Vtestbench___024root___eval_phase__act(vlSelf)) {
                vlSelfRef.__VactContinue = 1U;
            }
        }
        if (Vtestbench___024root___eval_phase__nba(vlSelf)) {
            __VnbaContinue = 1U;
        }
    }
}

#ifdef VL_DEBUG
void Vtestbench___024root___eval_debug_assertions(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___eval_debug_assertions\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
}
#endif  // VL_DEBUG
