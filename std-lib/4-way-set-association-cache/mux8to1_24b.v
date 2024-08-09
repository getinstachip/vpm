module mux8to1_24b(input [23:0] in1,input [23:0] in2, input [23:0] in3, input [23:0] in4, input [23:0] in5,input [23:0] in6, input [23:0] in7, input [23:0] in8, input[2:0] sel,output reg[23:0] tagOut);
  always@(in1,in2,in3,in4,in5,in6,in7,in8,sel)
  case(sel)
    3'b000: tagOut=in1;
    3'b001: tagOut=in2;
    3'b010: tagOut=in3;
    3'b011: tagOut=in4;
    3'b100: tagOut=in5;
    3'b101: tagOut=in6;
    3'b110: tagOut=in7;
    3'b111: tagOut=in8;
  endcase
endmodule