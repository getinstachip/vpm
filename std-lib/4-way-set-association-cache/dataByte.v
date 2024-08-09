module dataByte(input clk, input reset, input write, input [7:0] data,output [7:0] byteData);
  D_FF d00(clk, reset,write,data[0],byteData[0]);
  D_FF d01(clk, reset,write,data[1],byteData[1]);
  D_FF d02(clk, reset,write,data[2],byteData[2]);
  D_FF d03(clk, reset,write,data[3],byteData[3]);
  D_FF d04(clk, reset,write,data[4],byteData[4]);
  D_FF d05(clk, reset,write,data[5],byteData[5]);
  D_FF d06(clk, reset,write,data[6],byteData[6]);
  D_FF d07(clk, reset,write,data[7],byteData[7]);
endmodule