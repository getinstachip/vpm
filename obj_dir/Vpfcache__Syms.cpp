// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table implementation internals

#include "Vpfcache__pch.h"
#include "Vpfcache.h"
#include "Vpfcache___024root.h"

// FUNCTIONS
Vpfcache__Syms::~Vpfcache__Syms()
{
}

Vpfcache__Syms::Vpfcache__Syms(VerilatedContext* contextp, const char* namep, Vpfcache* modelp)
    : VerilatedSyms{contextp}
    // Setup internal state of the Syms class
    , __Vm_modelp{modelp}
    // Setup module instances
    , TOP{this, namep}
{
        // Check resources
        Verilated::stackCheck(75);
    // Configure time unit / time precision
    _vm_contextp__->timeunit(-12);
    _vm_contextp__->timeprecision(-12);
    // Setup each module's pointers to their submodules
    // Setup each module's pointer back to symbol table (for public functions)
    TOP.__Vconfigure(true);
}
