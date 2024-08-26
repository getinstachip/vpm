// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vtestbench.h for the primary calling header

#include "Vtestbench__pch.h"
#include "Vtestbench__Syms.h"
#include "Vtestbench___024root.h"

void Vtestbench___024root___ctor_var_reset(Vtestbench___024root* vlSelf);

Vtestbench___024root::Vtestbench___024root(Vtestbench__Syms* symsp, const char* v__name)
    : VerilatedModule{v__name}
    , __VdlySched{*symsp->_vm_contextp__}
    , vlSymsp{symsp}
 {
    // Reset structure values
    Vtestbench___024root___ctor_var_reset(this);
}

void Vtestbench___024root::__Vconfigure(bool first) {
    (void)first;  // Prevent unused variable warning
}

Vtestbench___024root::~Vtestbench___024root() {
}
