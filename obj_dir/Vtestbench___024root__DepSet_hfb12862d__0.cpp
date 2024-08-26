// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vtestbench.h for the primary calling header

#include "Vtestbench__pch.h"
#include "Vtestbench__Syms.h"
#include "Vtestbench___024root.h"

VL_INLINE_OPT VlCoroutine Vtestbench___024root___eval_initial__TOP__Vtiming__0(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___eval_initial__TOP__Vtiming__0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSymsp->TOP____024unit.__VmonitorNum = 1U;
    vlSelfRef.testbench__DOT__a = 3U;
    vlSelfRef.testbench__DOT__b = 1U;
    vlSelfRef.testbench__DOT__opt_sub = 0U;
    vlSelfRef.testbench__DOT__cin = 0U;
    co_await vlSelfRef.__VdlySched.delay(0x2710ULL, 
                                         nullptr, "add/testbench.v", 
                                         45);
    vlSelfRef.testbench__DOT__a = 7U;
    vlSelfRef.testbench__DOT__b = 5U;
    vlSelfRef.testbench__DOT__opt_sub = 0U;
    vlSelfRef.testbench__DOT__cin = 1U;
    co_await vlSelfRef.__VdlySched.delay(0x2710ULL, 
                                         nullptr, "add/testbench.v", 
                                         52);
    vlSelfRef.testbench__DOT__a = 4U;
    vlSelfRef.testbench__DOT__b = 3U;
    vlSelfRef.testbench__DOT__opt_sub = 1U;
    vlSelfRef.testbench__DOT__cin = 0U;
    co_await vlSelfRef.__VdlySched.delay(0x2710ULL, 
                                         nullptr, "add/testbench.v", 
                                         59);
    vlSelfRef.testbench__DOT__a = 2U;
    vlSelfRef.testbench__DOT__b = 5U;
    vlSelfRef.testbench__DOT__opt_sub = 1U;
    vlSelfRef.testbench__DOT__cin = 0U;
    co_await vlSelfRef.__VdlySched.delay(0x2710ULL, 
                                         nullptr, "add/testbench.v", 
                                         66);
    vlSelfRef.testbench__DOT__a = 7U;
    vlSelfRef.testbench__DOT__b = 1U;
    vlSelfRef.testbench__DOT__opt_sub = 0U;
    vlSelfRef.testbench__DOT__cin = 0U;
    co_await vlSelfRef.__VdlySched.delay(0x2710ULL, 
                                         nullptr, "add/testbench.v", 
                                         73);
    VL_FINISH_MT("add/testbench.v", 76, "");
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vtestbench___024root___dump_triggers__act(Vtestbench___024root* vlSelf);
#endif  // VL_DEBUG

void Vtestbench___024root___eval_triggers__act(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___eval_triggers__act\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.__VactTriggered.set(0U, ((((((((((IData)(vlSelfRef.testbench__DOT__a) 
                                               != (IData)(vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__a__0)) 
                                              | ((IData)(vlSelfRef.testbench__DOT__b) 
                                                 != (IData)(vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__b__0))) 
                                             | ((IData)(vlSelfRef.testbench__DOT__cin) 
                                                != (IData)(vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__cin__0))) 
                                            | ((IData)(vlSelfRef.testbench__DOT__cout) 
                                               != (IData)(vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__cout__0))) 
                                           | ((IData)(vlSelfRef.testbench__DOT__neg) 
                                              != (IData)(vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__neg__0))) 
                                          | ((IData)(vlSelfRef.testbench__DOT__opt_sub) 
                                             != (IData)(vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__opt_sub__0))) 
                                         | ((IData)(vlSelfRef.testbench__DOT__overflow) 
                                            != (IData)(vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__overflow__0))) 
                                        | ((IData)(vlSelfRef.testbench__DOT__sum) 
                                           != (IData)(vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__sum__0))) 
                                       | ((IData)(vlSelfRef.testbench__DOT__zero) 
                                          != (IData)(vlSelfRef.__Vtrigprevexpr___TOP__testbench__DOT__zero__0))));
    vlSelfRef.__VactTriggered.set(1U, vlSelfRef.__VdlySched.awaitingCurrentTime());
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
    if (VL_UNLIKELY((1U & (~ (IData)(vlSelfRef.__VactDidInit))))) {
        vlSelfRef.__VactDidInit = 1U;
        vlSelfRef.__VactTriggered.set(0U, 1U);
    }
#ifdef VL_DEBUG
    if (VL_UNLIKELY(vlSymsp->_vm_contextp__->debug())) {
        Vtestbench___024root___dump_triggers__act(vlSelf);
    }
#endif
}

VL_INLINE_OPT void Vtestbench___024root___nba_sequent__TOP__0(Vtestbench___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vtestbench__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vtestbench___024root___nba_sequent__TOP__0\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if (VL_UNLIKELY(((~ (IData)(vlSymsp->TOP____024unit.__VmonitorOff)) 
                     & (1U == vlSymsp->TOP____024unit.__VmonitorNum)))) {
        VL_WRITEF_NX("Time: %0t | a: %b | b: %b | opt_sub: %b | cin: %b | sum: %b | cout: %b | zero: %b | neg: %b | overflow: %b\n",0,
                     64,VL_TIME_UNITED_Q(1000),-9,4,
                     (IData)(vlSelfRef.testbench__DOT__a),
                     4,vlSelfRef.testbench__DOT__b,
                     1,(IData)(vlSelfRef.testbench__DOT__opt_sub),
                     1,vlSelfRef.testbench__DOT__cin,
                     4,(IData)(vlSelfRef.testbench__DOT__sum),
                     1,vlSelfRef.testbench__DOT__cout,
                     1,(IData)(vlSelfRef.testbench__DOT__zero),
                     1,vlSelfRef.testbench__DOT__neg,
                     1,(IData)(vlSelfRef.testbench__DOT__overflow));
    }
}
