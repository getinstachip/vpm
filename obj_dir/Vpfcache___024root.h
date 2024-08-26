// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design internal header
// See Vpfcache.h for the primary calling header

#ifndef VERILATED_VPFCACHE___024ROOT_H_
#define VERILATED_VPFCACHE___024ROOT_H_  // guard

#include "verilated.h"


class Vpfcache__Syms;

class alignas(VL_CACHE_LINE_BYTES) Vpfcache___024root final : public VerilatedModule {
  public:

    // DESIGN SPECIFIC STATE
    VL_IN8(i_clk,0,0);
    VL_IN8(i_reset,0,0);
    VL_IN8(i_new_pc,0,0);
    VL_IN8(i_clear_cache,0,0);
    VL_IN8(i_ready,0,0);
    VL_OUT8(o_valid,0,0);
    VL_OUT8(o_illegal,0,0);
    VL_OUT8(o_wb_cyc,0,0);
    VL_OUT8(o_wb_stb,0,0);
    VL_OUT8(o_wb_we,0,0);
    VL_IN8(i_wb_stall,0,0);
    VL_IN8(i_wb_ack,0,0);
    VL_IN8(i_wb_err,0,0);
    CData/*0:0*/ pfcache__DOT__r_v;
    CData/*0:0*/ pfcache__DOT__r_v_from_pc;
    CData/*0:0*/ pfcache__DOT__r_v_from_last;
    CData/*0:0*/ pfcache__DOT__rvsrc;
    CData/*0:0*/ pfcache__DOT__w_v_from_last;
    CData/*0:0*/ pfcache__DOT__illegal_valid;
    CData/*0:0*/ pfcache__DOT__isrc;
    CData/*1:0*/ pfcache__DOT__delay;
    CData/*0:0*/ pfcache__DOT__svmask;
    CData/*0:0*/ pfcache__DOT__last_ack;
    CData/*0:0*/ pfcache__DOT__needload;
    CData/*0:0*/ pfcache__DOT__last_addr;
    CData/*0:0*/ pfcache__DOT__bus_abort;
    CData/*0:0*/ pfcache__DOT__w_advance;
    CData/*0:0*/ pfcache__DOT__w_invalidate_result;
    CData/*0:0*/ __VstlFirstIteration;
    CData/*0:0*/ __VicoFirstIteration;
    CData/*0:0*/ __Vtrigprevexpr___TOP__i_clk__0;
    CData/*0:0*/ __VactContinue;
    SData/*11:0*/ pfcache__DOT__wraddr;
    SData/*8:0*/ pfcache__DOT__saddr;
    VL_IN(i_pc,31,0);
    VL_OUT(o_insn,31,0);
    VL_OUT(o_pc,31,0);
    VL_OUT(o_wb_addr,29,0);
    VL_OUT(o_wb_data,31,0);
    VL_IN(i_wb_data,31,0);
    VlWide<16>/*511:0*/ pfcache__DOT__valid_mask;
    IData/*31:0*/ pfcache__DOT__lastpc;
    IData/*26:0*/ pfcache__DOT__pc_tag_lookup;
    IData/*26:0*/ pfcache__DOT__last_tag_lookup;
    IData/*26:0*/ pfcache__DOT__tag_lookup;
    IData/*26:0*/ pfcache__DOT__illegal_cache;
    IData/*31:0*/ pfcache__DOT__r_pc_cache;
    IData/*31:0*/ pfcache__DOT__r_last_cache;
    IData/*31:0*/ pfcache__DOT__r_pc;
    IData/*31:0*/ __VactIterCount;
    VlUnpacked<IData/*31:0*/, 4096> pfcache__DOT__cache;
    VlUnpacked<IData/*17:0*/, 512> pfcache__DOT__cache_tags;
    VlTriggerVec<1> __VstlTriggered;
    VlTriggerVec<1> __VicoTriggered;
    VlTriggerVec<1> __VactTriggered;
    VlTriggerVec<1> __VnbaTriggered;

    // INTERNAL VARIABLES
    Vpfcache__Syms* const vlSymsp;

    // CONSTRUCTORS
    Vpfcache___024root(Vpfcache__Syms* symsp, const char* v__name);
    ~Vpfcache___024root();
    VL_UNCOPYABLE(Vpfcache___024root);

    // INTERNAL METHODS
    void __Vconfigure(bool first);
};


#endif  // guard
