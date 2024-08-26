// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table internal header
//
// Internal details; most calling programs do not need this header,
// unless using verilator public meta comments.

#ifndef VERILATED_VPFCACHE__SYMS_H_
#define VERILATED_VPFCACHE__SYMS_H_  // guard

#include "verilated.h"

// INCLUDE MODEL CLASS

#include "Vpfcache.h"

// INCLUDE MODULE CLASSES
#include "Vpfcache___024root.h"

// SYMS CLASS (contains all model state)
class alignas(VL_CACHE_LINE_BYTES)Vpfcache__Syms final : public VerilatedSyms {
  public:
    // INTERNAL STATE
    Vpfcache* const __Vm_modelp;
    VlDeleter __Vm_deleter;
    bool __Vm_didInit = false;

    // MODULE INSTANCE STATE
    Vpfcache___024root             TOP;

    // CONSTRUCTORS
    Vpfcache__Syms(VerilatedContext* contextp, const char* namep, Vpfcache* modelp);
    ~Vpfcache__Syms();

    // METHODS
    const char* name() { return TOP.name(); }
};

#endif  // guard
