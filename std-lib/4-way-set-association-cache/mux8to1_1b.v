module mux8to1_1b(input in1, input in2, input in3, input in4, input in5, input in6, input in7, input in8,input[2:0] sel,output reg validOut);
  always@(in1,in2,in3,in4,in5,in6,in7,in8,sel)
  case(sel)
    3'b000: validOut=in1;
    3'b001: validOut=in2;
    3'b010: validOut=in3;
    3'b011: validOut=in4;
    3'b100: validOut=in5;
    3'b101: validOut=in6;
    3'b110: validOut=in7;
    3'b111: validOut=in8;
  endcase
endmodule