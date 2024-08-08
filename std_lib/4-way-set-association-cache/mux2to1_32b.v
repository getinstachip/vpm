module mux2to1_32b(input [31:0] in1, input [31:0] in2, input sel, output reg [31:0] muxOut);  
  always@(in1 or in2 or sel)
    case (sel)
      1'b0: muxOut=in1;
      1'b1: muxOut=in2;
    endcase 
endmodule