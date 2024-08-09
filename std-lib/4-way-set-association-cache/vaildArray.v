module vaildArray(input clk, input reset, input [7:0] we, input validBit,output validOut0, output validOut1, output validOut2, output validOut3,output validOut4, output validOut5, output validOut6, output validOut7);
  D_FF v0(clk, reset,we[0],validBit,validOut0);
  D_FF v1(clk, reset,we[1],validBit,validOut1);
  D_FF v2(clk, reset,we[2],validBit,validOut2);
  D_FF v3(clk, reset,we[3],validBit,validOut3);
  D_FF v4(clk, reset,we[4],validBit,validOut4);
  D_FF v5(clk, reset,we[5],validBit,validOut5);
  D_FF v6(clk, reset,we[6],validBit,validOut6);
  D_FF v7(clk, reset,we[7],validBit,validOut7);
endmodule