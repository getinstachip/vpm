module testbench;
  reg clk, reset, writeORread;
  reg [7:0] inbyte;
  reg [31:0] pa;
  reg [255:0] dataBlock;
  wire [7:0] cacheOutput;
  cacheModule uut(clk, reset,pa, writeORread, inbyte, dataBlock, cacheOutput);
  always #5 clk=~clk;
  initial
  begin
    clk=0; reset=1;//writeORread=0;inbyte=7'd9;dataBlock=256'h123456;pa=32'h9876ABC0;
    #10 reset=0;writeORread=0;inbyte=8'd9;dataBlock=256'h123456;pa=32'h9876ABC0;//read miss
    #30 writeORread=0;inbyte=8'h77;dataBlock=256'h123456;pa=32'hABCDABC0;       //read miss
    #30 writeORread=1;inbyte=8'hAA;pa=32'hABCDABC0;                             //write hit
    #30 writeORread=1;inbyte=8'hBB;dataBlock=256'h666666;pa=32'h12345678;       //write miss
    #40 $finish;
  end
endmodule