module mux4to1_256b(input [255:0] in1,input [255:0] in2, input [255:0] in3, input [255:0] in4, input[1:0] sel,output reg[255:0] muxOut);
  always@(in1,in2,in3,in4,sel)
  case(sel)
    2'b00: muxOut=in1;
    2'b01: muxOut=in2;
    2'b10: muxOut=in3;
    2'b11: muxOut=in4;
  endcase
endmodule