module priorityArray(input clk, input reset, input [7:0] we, input [1:0] priority,output [1:0] priorityOut0, 
output [1:0] priorityOut1, output [1:0] priorityOut2, output [1:0] priorityOut3,output [1:0] priorityOut4, 
output [1:0] priorityOut5, output [1:0] priorityOut6, output [1:0] priorityOut7);
  priorityBlock p0(clk,reset,we[0],priority,priorityOut0);
  priorityBlock p1(clk,reset,we[1],priority,priorityOut1);
  priorityBlock p2(clk,reset,we[2],priority,priorityOut2);
  priorityBlock p3(clk,reset,we[3],priority,priorityOut3);
  priorityBlock p4(clk,reset,we[4],priority,priorityOut4);
  priorityBlock p5(clk,reset,we[5],priority,priorityOut5);
  priorityBlock p6(clk,reset,we[6],priority,priorityOut6);
  priorityBlock p7(clk,reset,we[7],priority,priorityOut7);
endmodule