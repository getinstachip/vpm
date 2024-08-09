module priorityBlock(input clk, input reset, input write, input [1:0] priority,output [1:0] priorityData);
  D_FF d00(clk, reset,write,priority[0],priorityData[0]);
  D_FF d01(clk, reset,write,priority[1],priorityData[1]);
endmodule