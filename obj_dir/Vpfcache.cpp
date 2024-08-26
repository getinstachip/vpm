// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Model implementation (design independent parts)

#include "Vpfcache__pch.h"

//============================================================
// Constructors

Vpfcache::Vpfcache(VerilatedContext* _vcontextp__, const char* _vcname__)
    : VerilatedModel{*_vcontextp__}
    , vlSymsp{new Vpfcache__Syms(contextp(), _vcname__, this)}
    , i_clk{vlSymsp->TOP.i_clk}
    , i_reset{vlSymsp->TOP.i_reset}
    , i_new_pc{vlSymsp->TOP.i_new_pc}
    , i_clear_cache{vlSymsp->TOP.i_clear_cache}
    , i_ready{vlSymsp->TOP.i_ready}
    , o_valid{vlSymsp->TOP.o_valid}
    , o_illegal{vlSymsp->TOP.o_illegal}
    , o_wb_cyc{vlSymsp->TOP.o_wb_cyc}
    , o_wb_stb{vlSymsp->TOP.o_wb_stb}
    , o_wb_we{vlSymsp->TOP.o_wb_we}
    , i_wb_stall{vlSymsp->TOP.i_wb_stall}
    , i_wb_ack{vlSymsp->TOP.i_wb_ack}
    , i_wb_err{vlSymsp->TOP.i_wb_err}
    , i_pc{vlSymsp->TOP.i_pc}
    , o_insn{vlSymsp->TOP.o_insn}
    , o_pc{vlSymsp->TOP.o_pc}
    , o_wb_addr{vlSymsp->TOP.o_wb_addr}
    , o_wb_data{vlSymsp->TOP.o_wb_data}
    , i_wb_data{vlSymsp->TOP.i_wb_data}
    , rootp{&(vlSymsp->TOP)}
{
    // Register model with the context
    contextp()->addModel(this);
}

Vpfcache::Vpfcache(const char* _vcname__)
    : Vpfcache(Verilated::threadContextp(), _vcname__)
{
}

//============================================================
// Destructor

Vpfcache::~Vpfcache() {
    delete vlSymsp;
}

//============================================================
// Evaluation function

#ifdef VL_DEBUG
void Vpfcache___024root___eval_debug_assertions(Vpfcache___024root* vlSelf);
#endif  // VL_DEBUG
void Vpfcache___024root___eval_static(Vpfcache___024root* vlSelf);
void Vpfcache___024root___eval_initial(Vpfcache___024root* vlSelf);
void Vpfcache___024root___eval_settle(Vpfcache___024root* vlSelf);
void Vpfcache___024root___eval(Vpfcache___024root* vlSelf);

void Vpfcache::eval_step() {
    VL_DEBUG_IF(VL_DBG_MSGF("+++++TOP Evaluate Vpfcache::eval_step\n"); );
#ifdef VL_DEBUG
    // Debug assertions
    Vpfcache___024root___eval_debug_assertions(&(vlSymsp->TOP));
#endif  // VL_DEBUG
    vlSymsp->__Vm_deleter.deleteAll();
    if (VL_UNLIKELY(!vlSymsp->__Vm_didInit)) {
        vlSymsp->__Vm_didInit = true;
        VL_DEBUG_IF(VL_DBG_MSGF("+ Initial\n"););
        Vpfcache___024root___eval_static(&(vlSymsp->TOP));
        Vpfcache___024root___eval_initial(&(vlSymsp->TOP));
        Vpfcache___024root___eval_settle(&(vlSymsp->TOP));
    }
    VL_DEBUG_IF(VL_DBG_MSGF("+ Eval\n"););
    Vpfcache___024root___eval(&(vlSymsp->TOP));
    // Evaluate cleanup
    Verilated::endOfEval(vlSymsp->__Vm_evalMsgQp);
}

//============================================================
// Events and timing
bool Vpfcache::eventsPending() { return false; }

uint64_t Vpfcache::nextTimeSlot() {
    VL_FATAL_MT(__FILE__, __LINE__, "", "%Error: No delays in the design");
    return 0;
}

//============================================================
// Utilities

const char* Vpfcache::name() const {
    return vlSymsp->name();
}

//============================================================
// Invoke final blocks

void Vpfcache___024root___eval_final(Vpfcache___024root* vlSelf);

VL_ATTR_COLD void Vpfcache::final() {
    Vpfcache___024root___eval_final(&(vlSymsp->TOP));
}

//============================================================
// Implementations of abstract methods from VerilatedModel

const char* Vpfcache::hierName() const { return vlSymsp->name(); }
const char* Vpfcache::modelName() const { return "Vpfcache"; }
unsigned Vpfcache::threads() const { return 1; }
void Vpfcache::prepareClone() const { contextp()->prepareClone(); }
void Vpfcache::atClone() const {
    contextp()->threadPoolpOnClone();
}
