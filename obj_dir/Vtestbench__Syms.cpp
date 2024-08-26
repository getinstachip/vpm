// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table implementation internals

#include "Vtestbench__pch.h"
#include "Vtestbench.h"
#include "Vtestbench___024root.h"
#include "Vtestbench___024unit.h"

// FUNCTIONS
Vtestbench__Syms::~Vtestbench__Syms()
{
}

Vtestbench__Syms::Vtestbench__Syms(VerilatedContext* contextp, const char* namep, Vtestbench* modelp)
    : VerilatedSyms{contextp}
    // Setup internal state of the Syms class
    , __Vm_modelp{modelp}
    // Setup module instances
    , TOP{this, namep}
    , TOP____024unit{this, Verilated::catName(namep, "$unit")}
{
        // Check resources
        Verilated::stackCheck(18);
    // Configure time unit / time precision
    _vm_contextp__->timeunit(-9);
    _vm_contextp__->timeprecision(-12);
    // Setup each module's pointers to their submodules
    TOP.__PVT____024unit = &TOP____024unit;
    // Setup each module's pointer back to symbol table (for public functions)
    TOP.__Vconfigure(true);
    TOP____024unit.__Vconfigure(true);
    // Setup scopes
    __Vscope_testbench.configure(this, name(), "testbench", "testbench", -9, VerilatedScope::SCOPE_OTHER);
}
