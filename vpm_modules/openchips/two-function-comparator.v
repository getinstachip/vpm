// Instantiation only

module DW01_cmp2_inst( inst_A, inst_B, inst_LEQ, inst_TC, 
                       LT_LE_inst, GE_GT_inst );

  parameter width = 8;

  input [width-1 : 0] inst_A;
  input [width-1 : 0] inst_B;
  input inst_LEQ;
  input inst_TC;
  output LT_LE_inst;
  output GE_GT_inst;

  // Instance of DW01_cmp2
  DW01_cmp2 #(width)
    U1 ( .A(inst_A), .B(inst_B), .LEQ(inst_LEQ), .TC(inst_TC),
         .LT_LE(LT_LE_inst), .GE_GT(GE_GT_inst) );

endmodule
