module dataArray(input clk, input reset, input [31:0] we0, input [31:0] we1, input [31:0] we2, input [31:0] we3, input [31:0] we4, input [31:0] we5, input [31:0] we6, input [31:0] we7, input [255:0] block, output [255:0] blockOut0, output [255:0] blockOut1, output [255:0] blockOut2, output [255:0]  blockOut3, output [255:0]  blockOut4, output [255:0]  blockOut5, output [255:0]  blockOut6, output [255:0]  blockOut7);
  dataBlock block0(clk,reset,we0,block,blockOut0);
  dataBlock block1(clk,reset,we1,block,blockOut1);
  dataBlock block2(clk,reset,we2,block,blockOut2);
  dataBlock block3(clk,reset,we3,block,blockOut3);
  dataBlock block4(clk,reset,we4,block,blockOut4);
  dataBlock block5(clk,reset,we5,block,blockOut5);
  dataBlock block6(clk,reset,we6,block,blockOut6);
  dataBlock block7(clk,reset,we7,block,blockOut7);
endmodule