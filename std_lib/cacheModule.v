module cacheModule(input clk, input reset, input [31:0] pa, input writeORread, input [7:0] inByte, input [255:0] dataBlock, output [7:0] cacheOutput);

	wire hit,replace,finalWrite, finalRead, bufferWrite,dataSel;
	wire [1:0] way,way0,way1;
	wire [3:0] validBits;
	wire [255:0] writeBlock,bufferBlock;
	CtrlCkt controlSignal(clk, reset, writeORread, hit, replace, finalWrite, finalRead, bufferWrite, dataSel);
	fourWaySACache tagComp(clk, reset, way, replace | finalWrite, pa[31:8], pa[7:5], hit, way0, validBits);
	mux2to1_256b mx256btm0(dataBlock,{32{inByte}}, dataSel,writeBlock);
	dataCache dataOp(clk, reset,way,replace | finalWrite, finalRead,dataSel,writeBlock,pa[7:5],pa[4:0],cacheOutput, bufferBlock);
	buffer buffCac0(clk, reset,bufferWrite,pa[31:5],bufferBlock);
	FIFO fifoReplacement0(clk,reset,replace,pa[7:5], validBits, way1);
	mux2to1_2b mx2btm0(way1, way0, finalRead, way);
endmodule