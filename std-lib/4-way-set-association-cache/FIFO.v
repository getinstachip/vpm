module FIFO(input clk,input reset,input replace,input [2:0] index, input  [3:0] validBits, output [1:0] finalWay);

wire [7:0] decOut;
decoder3to8 ind(index,decOut); 

wire [1:0] priority0,priority1,priority2,priority3,
priorityOut00,priorityOut01,priorityOut02,priorityOut03,priorityOut04,priorityOut05,priorityOut06,priorityOut07,
priorityOut10,priorityOut11,priorityOut12,priorityOut13,priorityOut14,priorityOut15,priorityOut16,priorityOut17,
priorityOut20,priorityOut21,priorityOut22,priorityOut23,priorityOut24,priorityOut25,priorityOut26,priorityOut27,
priorityOut30,priorityOut31,priorityOut32,priorityOut33,priorityOut34,priorityOut35,priorityOut36,priorityOut37;

wire [7:0] we;
assign we = decOut & {8{replace}};
priorityArray arr0(clk,reset,we,priority0,priorityOut00,priorityOut01,priorityOut02,priorityOut03,priorityOut04,priorityOut05,priorityOut06,priorityOut07);
priorityArray arr1(clk,reset,we,priority1,priorityOut10,priorityOut11,priorityOut12,priorityOut13,priorityOut14,priorityOut15,priorityOut16,priorityOut17);
priorityArray arr2(clk,reset,we,priority2,priorityOut20,priorityOut21,priorityOut22,priorityOut23,priorityOut24,priorityOut25,priorityOut26,priorityOut27);
priorityArray arr3(clk,reset,we,priority3,priorityOut30,priorityOut31,priorityOut32,priorityOut33,priorityOut34,priorityOut35,priorityOut36,priorityOut37);

wire [1:0] muxOut0,muxOut1,muxOut2,muxOut3;
mux8to1_2b m0(priorityOut00,priorityOut01,priorityOut02,priorityOut03,priorityOut04,priorityOut05,priorityOut06,priorityOut07,index,muxOut0);
mux8to1_2b m1(priorityOut10,priorityOut11,priorityOut12,priorityOut13,priorityOut14,priorityOut15,priorityOut16,priorityOut17,index,muxOut1);
mux8to1_2b m2(priorityOut20,priorityOut21,priorityOut22,priorityOut23,priorityOut24,priorityOut25,priorityOut26,priorityOut27,index,muxOut2);
mux8to1_2b m3(priorityOut30,priorityOut31,priorityOut32,priorityOut33,priorityOut34,priorityOut35,priorityOut36,priorityOut37,index,muxOut3);
wire validReplace;    // it is 0 when replacement is based on atleast 1 invalid cache block
wire [1:0] validWayOut,priorityWayOut;
validityEncoder decision(validBits,validReplace,validWayOut); 
priorityEncoder_4x2 encode({muxOut3[1]|muxOut3[0],muxOut2[1]|muxOut2[0],muxOut1[1]|muxOut1[0],muxOut0[1]|muxOut0[0]},priorityWayOut);

mux2to1_2b mx2b00(validWayOut,priorityWayOut,validReplace,finalWay);
updatePriority update(finalWay,muxOut0,muxOut1,muxOut2,muxOut3,priority0,priority1,priority2,priority3);
endmodule