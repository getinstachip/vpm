// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vpfcache.h for the primary calling header

#include "Vpfcache__pch.h"
#include "Vpfcache__Syms.h"
#include "Vpfcache___024root.h"

void Vpfcache___024root___ctor_var_reset(Vpfcache___024root* vlSelf);

Vpfcache___024root::Vpfcache___024root(Vpfcache__Syms* symsp, const char* v__name)
    : VerilatedModule{v__name}
    , vlSymsp{symsp}
 {
    // Reset structure values
    Vpfcache___024root___ctor_var_reset(this);
}

void Vpfcache___024root::__Vconfigure(bool first) {
    (void)first;  // Prevent unused variable warning
}

Vpfcache___024root::~Vpfcache___024root() {
}
