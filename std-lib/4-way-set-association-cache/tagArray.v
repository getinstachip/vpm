module tagArray(input clk, input reset, input [7:0] we, input [23:0] tag, output [23:0] tagOut0, output [23:0] tagOut1, output [23:0] tagOut2, output [23:0]  tagOut3, output [23:0]  tagOut4, output [23:0]  tagOut5, output [23:0]  tagOut6, output [23:0]  tagOut7);
  tagBlock t0(clk,reset,we[0],tag,tagOut0);
  tagBlock t1(clk,reset,we[1],tag,tagOut1);
  tagBlock t2(clk,reset,we[2],tag,tagOut2);
  tagBlock t3(clk,reset,we[3],tag,tagOut3);
  tagBlock t4(clk,reset,we[4],tag,tagOut4);
  tagBlock t5(clk,reset,we[5],tag,tagOut5);
  tagBlock t6(clk,reset,we[6],tag,tagOut6);
  tagBlock t7(clk,reset,we[7],tag,tagOut7);
endmodule