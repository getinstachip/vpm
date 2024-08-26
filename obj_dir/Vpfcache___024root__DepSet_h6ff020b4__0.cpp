// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vpfcache.h for the primary calling header

#include "Vpfcache__pch.h"
#include "Vpfcache__Syms.h"
#include "Vpfcache___024root.h"

#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__ico(Vpfcache___024root* vlSelf);
#endif  // VL_DEBUG

void Vpfcache___024root___eval_triggers__ico(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_triggers__ico\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.__VicoTriggered.set(0U, (IData)(vlSelfRef.__VicoFirstIteration));
#ifdef VL_DEBUG
    if (VL_UNLIKELY(vlSymsp->_vm_contextp__->debug())) {
        Vpfcache___024root___dump_triggers__ico(vlSelf);
    }
#endif
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vpfcache___024root___dump_triggers__act(Vpfcache___024root* vlSelf);
#endif  // VL_DEBUG

void Vpfcache___024root___eval_triggers__act(Vpfcache___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    Vpfcache__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vpfcache___024root___eval_triggers__act\n"); );
    auto &vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.__VactTriggered.set(0U, ((IData)(vlSelfRef.i_clk) 
                                       & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__i_clk__0))));
    vlSelfRef.__Vtrigprevexpr___TOP__i_clk__0 = vlSelfRef.i_clk;
#ifdef VL_DEBUG
    if (VL_UNLIKELY(vlSymsp->_vm_contextp__->debug())) {
        Vpfcache___024root___dump_triggers__act(vlSelf);
    }
#endif
}
