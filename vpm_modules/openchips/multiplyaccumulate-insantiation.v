module DW02_mac_inst(
    inst_A, 
    inst_B, 
    inst_C, 
    inst_TC, 
    MAC_inst 
);

  // Parameter definitions for input widths
  parameter A_width = 8;
  parameter B_width = 8;

  // Input and output port declarations
  input [A_width-1 : 0] inst_A;      
  input [B_width-1 : 0] inst_B;             
  input [A_width+B_width-1 : 0] inst_C;       
  input inst_TC;                    
  output [A_width+B_width-1 : 0] MAC_inst;

  // Instance of DW02_mac
  DW02_mac #(A_width, B_width)
    U1 (
      .A(inst_A), 
      .B(inst_B), 
      .C(inst_C), 
      .TC(inst_TC), 
      .MAC(MAC_inst) 
    );

endmodule
