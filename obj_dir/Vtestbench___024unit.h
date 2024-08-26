// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design internal header
// See Vtestbench.h for the primary calling header

#ifndef VERILATED_VTESTBENCH___024UNIT_H_
#define VERILATED_VTESTBENCH___024UNIT_H_  // guard

#include "verilated.h"
#include "verilated_timing.h"


class Vtestbench__Syms;

class alignas(VL_CACHE_LINE_BYTES) Vtestbench___024unit final : public VerilatedModule {
  public:

    // DESIGN SPECIFIC STATE
    CData/*0:0*/ __VmonitorOff;
    QData/*63:0*/ __VmonitorNum;

    // INTERNAL VARIABLES
    Vtestbench__Syms* const vlSymsp;

    // CONSTRUCTORS
    Vtestbench___024unit(Vtestbench__Syms* symsp, const char* v__name);
    ~Vtestbench___024unit();
    VL_UNCOPYABLE(Vtestbench___024unit);

    // INTERNAL METHODS
    void __Vconfigure(bool first);
};


#endif  // guard
