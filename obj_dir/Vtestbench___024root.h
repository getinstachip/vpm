// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design internal header
// See Vtestbench.h for the primary calling header

#ifndef VERILATED_VTESTBENCH___024ROOT_H_
#define VERILATED_VTESTBENCH___024ROOT_H_  // guard

#include "verilated.h"
#include "verilated_timing.h"
class Vtestbench___024unit;


class Vtestbench__Syms;

class alignas(VL_CACHE_LINE_BYTES) Vtestbench___024root final : public VerilatedModule {
  public:
    // CELLS
    Vtestbench___024unit* __PVT____024unit;

    // DESIGN SPECIFIC STATE
    CData/*0:0*/ testbench__DOT__opt_sub;
    CData/*0:0*/ testbench__DOT__cin;
    CData/*0:0*/ testbench__DOT__cout;
    CData/*0:0*/ testbench__DOT__zero;
    CData/*0:0*/ testbench__DOT__neg;
    CData/*0:0*/ testbench__DOT__overflow;
    CData/*3:0*/ testbench__DOT__a;
    CData/*3:0*/ testbench__DOT__b;
    CData/*3:0*/ testbench__DOT__sum;
    CData/*3:0*/ testbench__DOT__uut__DOT__b_sub;
    CData/*0:0*/ __VstlFirstIteration;
    CData/*3:0*/ __Vtrigprevexpr___TOP__testbench__DOT__a__0;
    CData/*3:0*/ __Vtrigprevexpr___TOP__testbench__DOT__b__0;
    CData/*0:0*/ __Vtrigprevexpr___TOP__testbench__DOT__cin__0;
    CData/*0:0*/ __Vtrigprevexpr___TOP__testbench__DOT__cout__0;
    CData/*0:0*/ __Vtrigprevexpr___TOP__testbench__DOT__neg__0;
    CData/*0:0*/ __Vtrigprevexpr___TOP__testbench__DOT__opt_sub__0;
    CData/*0:0*/ __Vtrigprevexpr___TOP__testbench__DOT__overflow__0;
    CData/*3:0*/ __Vtrigprevexpr___TOP__testbench__DOT__sum__0;
    CData/*0:0*/ __Vtrigprevexpr___TOP__testbench__DOT__zero__0;
    CData/*0:0*/ __VactDidInit;
    CData/*0:0*/ __VactContinue;
    IData/*31:0*/ __VactIterCount;
    VlUnpacked<CData/*0:0*/, 2> __Vm_traceActivity;
    VlDelayScheduler __VdlySched;
    VlTriggerVec<1> __VstlTriggered;
    VlTriggerVec<2> __VactTriggered;
    VlTriggerVec<2> __VnbaTriggered;

    // INTERNAL VARIABLES
    Vtestbench__Syms* const vlSymsp;

    // CONSTRUCTORS
    Vtestbench___024root(Vtestbench__Syms* symsp, const char* v__name);
    ~Vtestbench___024root();
    VL_UNCOPYABLE(Vtestbench___024root);

    // INTERNAL METHODS
    void __Vconfigure(bool first);
};


#endif  // guard
