// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vtestbench.h for the primary calling header

#include "Vtestbench__pch.h"
#include "Vtestbench__Syms.h"
#include "Vtestbench___024unit.h"

void Vtestbench___024unit___ctor_var_reset(Vtestbench___024unit* vlSelf);

Vtestbench___024unit::Vtestbench___024unit(Vtestbench__Syms* symsp, const char* v__name)
    : VerilatedModule{v__name}
    , vlSymsp{symsp}
 {
    // Reset structure values
    Vtestbench___024unit___ctor_var_reset(this);
}

void Vtestbench___024unit::__Vconfigure(bool first) {
    (void)first;  // Prevent unused variable warning
}

Vtestbench___024unit::~Vtestbench___024unit() {
}
